use std::f32::consts::{PI, TAU};

use osmos_core::{
    wrap_delta, wrap_position, ControlNetwork, LineageId, Mass, MoteId, PelletId, Vec2,
    SENSOR_INPUTS,
};
use rand::rngs::StdRng;
use rand::{RngExt, SeedableRng};

use crate::config::{SimConfig, SUBSTEPS, TICKS_PER_SECOND};
use crate::ejecta::Ejecta;
use crate::gravity::GravityWell;
use crate::grid::Grid;
use crate::mote::{Mote, MoteRole, MoteState};
use crate::pellet::Pellet;
use crate::sensor::SenseContext;
use crate::snapshot::{
    Snapshot, Stats, EJECTA_STATE_STRIDE, FLAG_ALIVE, MOTE_STATE_STRIDE, PELLET_STATE_STRIDE,
};

const LINEAGES: u16 = 12;
const HUNTER_MASS: f32 = 2.0;
const PREY_MASS: f32 = 0.6;

// === 世界状态 ===
pub struct World {
    config: SimConfig,
    tick: u64,
    pub motes: Vec<Mote>,
    pub pellets: Vec<Pellet>,
    pub ejecta: Vec<Ejecta>,
    mote_grid: Grid,
    pellet_grid: Grid,
    ejecta_grid: Grid,
    gravity: GravityWell,
    scratch: Vec<u32>,
    ejected_total: f32,
    absorbed_pellets: u32,
    pellet_reserve: f32,
    pellet_target: u32,
    pellet_rng: StdRng,
    next_ejecta_id: u32,
    next_pellet_id: u32,
}

impl World {
    // === 世界构建 ===
    pub fn new(config: SimConfig) -> Self {
        Self::build(config, Vec::new(), 0, 0)
    }

    pub fn with_controllers(config: SimConfig, controllers: Vec<ControlNetwork>) -> Self {
        Self::with_scenario(config, controllers, 0, 0)
    }

    /// 构建所有质体都是学习者的共享世界。
    ///
    /// `initial_masses` 与 `controllers` 使用相同索引。
    pub fn with_ecology(
        config: SimConfig,
        controllers: Vec<ControlNetwork>,
        initial_masses: &[f32],
    ) -> Self {
        assert_eq!(controllers.len(), initial_masses.len());
        let mut world = Self::with_scenario(config, controllers, 0, 0);
        for (mote, mass) in world.motes.iter_mut().zip(initial_masses.iter().copied()) {
            mote.mass = Mass::new(mass);
            mote.velocity = Vec2::ZERO;
        }
        world
    }

    pub fn with_predator_training(config: SimConfig, controllers: Vec<ControlNetwork>) -> Self {
        let mut world = Self::with_scenario(config, controllers, 0, 2);
        if world.motes.len() >= 3 {
            world.motes[0].position = Vec2::new(0.5, 0.5);
            world.motes[1].position = Vec2::new(0.2, 0.5);
            world.motes[2].position = Vec2::new(0.8, 0.5);
            for mote in &mut world.motes {
                mote.velocity = Vec2::ZERO;
            }
        }
        world
    }

    pub fn with_prey_training(config: SimConfig, controllers: Vec<ControlNetwork>) -> Self {
        let mut world = Self::with_scenario(config, controllers, 1, 0);
        if world.motes.len() >= 2 {
            world.motes[0].position = Vec2::new(0.5, 0.5);
            world.motes[1].position = Vec2::new(0.95, 0.5);
            for mote in &mut world.motes {
                mote.velocity = Vec2::ZERO;
            }
        }
        world
    }

    pub fn with_training(config: SimConfig, controllers: Vec<ControlNetwork>) -> Self {
        let mut world = Self::with_scenario(config, controllers, 1, 2);
        if world.motes.len() >= 4 {
            world.motes[0].position = Vec2::new(0.5, 0.5);
            world.motes[1].position = Vec2::new(0.92, 0.5);
            world.motes[2].position = Vec2::new(0.08, 0.5);
            world.motes[3].position = Vec2::new(0.5, 0.08);
            for mote in &mut world.motes {
                mote.velocity = Vec2::ZERO;
            }
        }
        world
    }

    pub fn with_scenario(
        config: SimConfig,
        controllers: Vec<ControlNetwork>,
        hunters: u32,
        prey: u32,
    ) -> Self {
        assert_eq!(
            controllers.len(),
            config.motes as usize,
            "controller count must match mote count"
        );
        Self::build(
            config,
            controllers.into_iter().map(Some).collect(),
            hunters,
            prey,
        )
    }

    fn build(
        config: SimConfig,
        controllers: Vec<Option<ControlNetwork>>,
        hunters: u32,
        prey: u32,
    ) -> Self {
        config.validate();

        let mut rng = StdRng::seed_from_u64(config.seed);
        let gravity = GravityWell::from_config(&config);
        let mut motes = Vec::with_capacity(config.motes as usize);
        let mut pellets = Vec::with_capacity(config.pellets as usize);

        for index in 0..config.motes {
            // 先消耗角度抽取，再抽取位置，保持种子布局稳定。
            let _angle = rng.random_range(0.0..TAU);
            let position = random_position(&mut rng);
            let mass = if config.initial_mass_spread > 0.0 {
                (config.mote_mass
                    + rng.random_range(-config.initial_mass_spread..config.initial_mass_spread))
                .max(0.2)
            } else {
                config.mote_mass
            };
            let velocity = if config.orbit_speed_factor > 0.0 {
                let radial = position - Vec2::new(config.gravity_center_x, config.gravity_center_y);
                let distance = radial.length();
                let acceleration = gravity.acceleration(position).length();
                let circular_speed = (acceleration * distance).sqrt();
                let tangent = Vec2::new(-radial.y, radial.x).normalize_or_zero();
                tangent * (circular_speed * config.orbit_speed_factor)
            } else {
                Vec2::ZERO
            };
            let mut mote = Mote::new(
                MoteId::new(index),
                LineageId::new((index % LINEAGES as u32) as u16),
                MoteRole::Ai,
                Mass::new(mass),
                position,
                velocity,
            );
            mote.controller = controllers.get(index as usize).cloned().flatten();
            motes.push(mote);
        }

        for index in 0..config.pellets {
            pellets.push(Pellet::new(
                PelletId::new(index),
                Mass::new(config.pellet_mass),
                random_position(&mut rng),
            ));
        }

        let mut next_id = config.motes;
        for _ in 0..hunters {
            motes.push(Mote::new(
                MoteId::new(next_id),
                LineageId::new(0),
                MoteRole::Hunter,
                Mass::new(HUNTER_MASS),
                random_position(&mut rng),
                random_velocity(&mut rng),
            ));
            next_id += 1;
        }

        for _ in 0..prey {
            motes.push(Mote::new(
                MoteId::new(next_id),
                LineageId::new(0),
                MoteRole::Prey,
                Mass::new(PREY_MASS),
                random_position(&mut rng),
                random_velocity(&mut rng),
            ));
            next_id += 1;
        }

        Self {
            config,
            tick: 0,
            motes,
            pellets,
            ejecta: Vec::new(),
            mote_grid: Grid::new(),
            pellet_grid: Grid::new(),
            ejecta_grid: Grid::new(),
            gravity,
            scratch: Vec::new(),
            ejected_total: 0.0,
            absorbed_pellets: 0,
            pellet_reserve: config.pellet_reserve,
            pellet_target: config.pellets,
            pellet_rng: StdRng::seed_from_u64(config.seed ^ 0x5045_4C4C_4554_0001),
            next_ejecta_id: 0,
            next_pellet_id: config.pellets,
        }
    }

    pub fn config(&self) -> SimConfig {
        self.config
    }

    pub fn tick(&self) -> u64 {
        self.tick
    }

    pub fn ejected_total(&self) -> f32 {
        self.ejected_total
    }

    pub fn absorbed_pellets(&self) -> u32 {
        self.absorbed_pellets
    }

    // === 主循环 ===
    pub fn step(&mut self) {
        self.tick += 1;
        for ejecta in &mut self.ejecta {
            ejecta.age += 1;
        }

        self.rebuild_grids();
        self.sense_and_control();
        self.drive_hunters();
        self.drive_prey();
        self.apply_thrust(1.0 / TICKS_PER_SECOND as f32);
        self.rebuild_grids();

        let dt = 1.0 / (TICKS_PER_SECOND as f32 * SUBSTEPS as f32);
        for _ in 0..SUBSTEPS {
            self.integrate(dt);
            self.integrate_ejecta(dt);
            self.rebuild_grids();
            self.resolve_pellets();
            self.resolve_ejecta();
            self.resolve_motes();
            self.remove_dead();
        }

        self.expire_ejecta();
        self.replenish_pellets();
    }

    // === 控制 ===
    fn drive_hunters(&mut self) {
        let hunter_speed = self.config.hunter_speed;

        for index in 0..self.motes.len() {
            if !self.motes[index].is_alive() || self.motes[index].role != MoteRole::Hunter {
                continue;
            }

            let hunter = &self.motes[index];
            let mut target = None;
            let mut best = f32::INFINITY;
            for other_index in 0..self.motes.len() {
                if other_index == index {
                    continue;
                }
                let other = &self.motes[other_index];
                if !other.is_alive() || other.mass.get() >= hunter.mass.get() {
                    continue;
                }
                let distance = wrap_delta(other.position - hunter.position).length_squared();
                if distance < best {
                    best = distance;
                    target = Some(other.position);
                }
            }

            if let Some(target) = target {
                let direction = wrap_delta(target - hunter.position).normalize_or_zero();
                if direction != Vec2::ZERO {
                    self.motes[index].velocity = direction * hunter_speed;
                }
            }
        }
    }

    fn drive_prey(&mut self) {
        let prey_speed = self.config.prey_speed;

        for index in 0..self.motes.len() {
            if !self.motes[index].is_alive() || self.motes[index].role != MoteRole::Prey {
                continue;
            }

            let prey = &self.motes[index];
            let mut threat = None;
            let mut best = f32::INFINITY;
            for other_index in 0..self.motes.len() {
                if other_index == index {
                    continue;
                }
                let other = &self.motes[other_index];
                if !other.is_alive() || other.mass.get() <= prey.mass.get() {
                    continue;
                }
                let distance = wrap_delta(other.position - prey.position).length_squared();
                if distance < best {
                    best = distance;
                    threat = Some(other.position);
                }
            }

            if let Some(threat) = threat {
                let direction = wrap_delta(prey.position - threat).normalize_or_zero();
                if direction != Vec2::ZERO {
                    self.motes[index].velocity = direction * prey_speed;
                }
            }
        }
    }

    fn sense_and_control(&mut self) {
        let mut nearby = Vec::new();

        for index in 0..self.motes.len() {
            if !self.motes[index].is_alive() {
                continue;
            }
            let Some(controller) = self.motes[index].controller.take() else {
                continue;
            };

            let mut sensor = [0.0_f32; SENSOR_INPUTS];
            SenseContext::new(
                &self.motes,
                &self.pellets,
                &self.ejecta,
                &self.mote_grid,
                &self.pellet_grid,
                &self.ejecta_grid,
                self.config.sensor_range,
                self.config.max_speed,
                self.config.episode_ticks,
            )
            .sense(index, self.tick, &mut nearby, &mut sensor);
            let output = controller.feed(&sensor);

            let mote = &mut self.motes[index];
            mote.sensor = sensor;
            mote.thrust = Vec2::new(output[0], output[1]);
            mote.controller = Some(controller);
        }
    }

    fn apply_thrust(&mut self, dt: f32) {
        let max_eject_rate = self.config.max_eject_rate;
        let min_mass = self.config.min_mass;
        let eject_speed = self.config.eject_speed;
        let max_speed = self.config.max_speed;
        let ejecta_enabled = self.config.ejecta_enabled;
        let ejecta_packet = self.config.ejecta_packet;
        let max_ejecta = self.config.max_ejecta;
        let mut ejected = 0.0_f32;
        let mut spawns = Vec::new();
        let mut next_id = self.next_ejecta_id;

        for mote in &mut self.motes {
            if !mote.is_alive() {
                continue;
            }

            let magnitude = mote.thrust.clamp_length(1.0).length();
            if magnitude <= 0.05 || mote.mass.get() <= min_mass + 1e-6 {
                continue;
            }

            let direction = mote.thrust.normalize_or_zero();
            if direction == Vec2::ZERO {
                continue;
            }

            let available = mote.mass.get() - min_mass;
            let mass_flow = (max_eject_rate * magnitude * dt).min(available);
            if mass_flow <= 0.0 {
                continue;
            }

            let old_mass = mote.mass.get();
            let new_mass = old_mass - mass_flow;
            mote.mass = Mass::new(new_mass);
            mote.velocity += direction * (eject_speed * mass_flow / new_mass);
            mote.velocity = mote.velocity.clamp_length(max_speed);
            mote.ejected_mass += mass_flow;
            ejected += mass_flow;

            if ejecta_enabled {
                mote.pending_ejecta += mass_flow;
                while mote.pending_ejecta >= ejecta_packet && spawns.len() < max_ejecta {
                    mote.pending_ejecta -= ejecta_packet;
                    spawns.push(Ejecta::new(
                        next_id,
                        mote.id,
                        Mass::new(ejecta_packet),
                        mote.position - direction * (mote.radius() + 0.003),
                        mote.velocity - direction * eject_speed,
                        mote.hue(),
                    ));
                    next_id = next_id.wrapping_add(1);
                }
            }
        }

        self.ejected_total += ejected;
        self.next_ejecta_id = next_id;
        self.ejecta.extend(spawns);
        while self.ejecta.len() > max_ejecta {
            self.convert_ejecta_to_pellet(0);
            self.ejecta.remove(0);
        }
    }

    pub fn total_mass(&self) -> f32 {
        self.motes
            .iter()
            .filter(|mote| mote.is_alive())
            .map(|mote| mote.mass.get() + mote.pending_ejecta)
            .sum::<f32>()
            + self
                .pellets
                .iter()
                .filter(|pellet| pellet.alive)
                .map(|pellet| pellet.mass.get())
                .sum::<f32>()
            + self
                .ejecta
                .iter()
                .filter(|ejecta| ejecta.alive)
                .map(|ejecta| ejecta.mass.get())
                .sum::<f32>()
            + self.pellet_reserve
    }

    fn replenish_pellets(&mut self) {
        if self.pellet_reserve <= 0.0 {
            return;
        }
        while self.pellets.len() < self.pellet_target as usize {
            let mass = self.config.pellet_mass;
            if self.pellet_reserve < mass {
                break;
            }
            self.pellet_reserve -= mass;
            self.pellets.push(Pellet::new(
                PelletId::new(self.next_pellet_id),
                Mass::new(mass),
                random_position(&mut self.pellet_rng),
            ));
            self.next_pellet_id = self.next_pellet_id.wrapping_add(1);
        }
    }

    // === 快照 ===
    pub fn snapshot(&self) -> Snapshot {
        let mut snapshot = Snapshot {
            tick: self.tick,
            ..Snapshot::default()
        };
        snapshot.mote_ids.reserve(self.motes.len());
        snapshot
            .mote_state
            .reserve(self.motes.len() * MOTE_STATE_STRIDE);
        snapshot.mote_role.reserve(self.motes.len());
        snapshot.mote_flags.reserve(self.motes.len());
        snapshot
            .pellet_state
            .reserve(self.pellets.len() * PELLET_STATE_STRIDE);
        snapshot
            .ejecta_state
            .reserve(self.ejecta.len() * EJECTA_STATE_STRIDE);

        let mut best_mass = 0.0_f32;
        for mote in &self.motes {
            if !mote.is_alive() {
                continue;
            }

            snapshot.mote_ids.push(mote.id.get());
            snapshot.mote_state.extend_from_slice(&[
                mote.position.x,
                mote.position.y,
                mote.velocity.x,
                mote.velocity.y,
                mote.mass.get(),
                mote.radius(),
                mote.hue(),
            ]);
            snapshot.mote_role.push(mote.role.as_u8());
            snapshot.mote_flags.push(FLAG_ALIVE);
            best_mass = best_mass.max(mote.mass.get());
        }

        for pellet in &self.pellets {
            if !pellet.alive {
                continue;
            }
            snapshot.pellet_state.extend_from_slice(&[
                pellet.position.x,
                pellet.position.y,
                pellet.mass.get(),
                pellet.radius(),
            ]);
        }

        for ejecta in &self.ejecta {
            if !ejecta.alive {
                continue;
            }
            snapshot.ejecta_state.extend_from_slice(&[
                ejecta.position.x,
                ejecta.position.y,
                ejecta.mass.get(),
                ejecta.radius(),
                ejecta.hue,
            ]);
        }

        if self.gravity.is_active() {
            snapshot.well_state.extend_from_slice(&[
                self.gravity.center.x,
                self.gravity.center.y,
                self.gravity.radius,
                self.gravity.kind(),
            ]);
        }

        let alive = snapshot.mote_ids.len() as u32;
        let pellets = snapshot.pellet_state.len() as u32 / PELLET_STATE_STRIDE as u32;
        let ejecta = snapshot.ejecta_state.len() as u32 / EJECTA_STATE_STRIDE as u32;
        snapshot.stats = Stats {
            generation: 0,
            tick: self.tick,
            alive,
            pellets,
            ejecta,
            best_mass,
            total_mass: self.total_mass(),
        };

        snapshot
    }

    // === 物理与碰撞 ===
    fn integrate(&mut self, dt: f32) {
        let max_speed = self.config.max_speed;
        let gravity = self.gravity;
        let field_active = gravity.is_active();

        for mote in &mut self.motes {
            if !mote.is_alive() {
                continue;
            }

            if field_active {
                mote.velocity += gravity.acceleration(mote.position) * dt;
            }
            mote.position = wrap_position(mote.position + mote.velocity * dt);
            mote.velocity = mote.velocity.clamp_length(max_speed);
        }
    }

    fn integrate_ejecta(&mut self, dt: f32) {
        let max_speed = self.config.max_speed;
        let gravity = self.gravity;
        let field_active = gravity.is_active();

        for ejecta in &mut self.ejecta {
            if !ejecta.alive {
                continue;
            }

            if field_active {
                ejecta.velocity += gravity.acceleration(ejecta.position) * dt;
            }
            ejecta.position = wrap_position(ejecta.position + ejecta.velocity * dt);
            ejecta.velocity = ejecta.velocity.clamp_length(max_speed);
        }
    }

    fn rebuild_grids(&mut self) {
        self.mote_grid.clear();
        for (index, mote) in self.motes.iter().enumerate() {
            if mote.is_alive() {
                self.mote_grid.insert(index, mote.position);
            }
        }

        self.pellet_grid.clear();
        for (index, pellet) in self.pellets.iter().enumerate() {
            if pellet.alive {
                self.pellet_grid.insert(index, pellet.position);
            }
        }

        self.ejecta_grid.clear();
        for (index, ejecta) in self.ejecta.iter().enumerate() {
            if ejecta.alive {
                self.ejecta_grid.insert(index, ejecta.position);
            }
        }
    }

    fn resolve_pellets(&mut self) {
        for index in 0..self.motes.len() {
            if !self.motes[index].is_alive() {
                continue;
            }

            let position = self.motes[index].position;
            let radius = self.motes[index].radius();
            let mut nearby = std::mem::take(&mut self.scratch);
            self.pellet_grid.nearby(position, &mut nearby);

            let mut gained = 0.0_f32;
            let mut count = 0_u32;
            for pellet_index in nearby.iter().copied().map(|value| value as usize) {
                let pellet = &self.pellets[pellet_index];
                if !pellet.alive {
                    continue;
                }
                let reach = radius + pellet.radius();
                if wrap_delta(pellet.position - position).length_squared() <= reach * reach {
                    gained += pellet.mass.get();
                    count += 1;
                    self.pellets[pellet_index].alive = false;
                }
            }

            if count > 0 {
                self.absorbed_pellets += count;
                let mote = &mut self.motes[index];
                let old_mass = mote.mass.get();
                let new_mass = old_mass + gained;
                mote.velocity *= old_mass / new_mass;
                mote.mass = Mass::new(new_mass);
                mote.absorbed_pellets += count;
            }

            self.scratch = nearby;
        }
    }

    fn resolve_ejecta(&mut self) {
        let immunity = self.config.owner_immunity_ticks;
        let max_speed = self.config.max_speed;

        for index in 0..self.motes.len() {
            if !self.motes[index].is_alive() {
                continue;
            }

            let position = self.motes[index].position;
            let radius = self.motes[index].radius();
            let owner = self.motes[index].id;
            let mut nearby = std::mem::take(&mut self.scratch);
            self.ejecta_grid.nearby(position, &mut nearby);

            for ejecta_index in nearby.iter().copied().map(|value| value as usize) {
                let ejecta = &self.ejecta[ejecta_index];
                if !ejecta.alive || (ejecta.owner == owner && ejecta.age < immunity) {
                    continue;
                }
                let reach = radius + ejecta.radius();
                if wrap_delta(ejecta.position - position).length_squared() > reach * reach {
                    continue;
                }

                let ejecta_mass = ejecta.mass.get();
                let ejecta_velocity = ejecta.velocity;
                self.ejecta[ejecta_index].alive = false;
                let mote = &mut self.motes[index];
                let total = mote.mass.get() + ejecta_mass;
                mote.velocity = (mote.velocity * mote.mass.get() + ejecta_velocity * ejecta_mass)
                    * (1.0 / total);
                mote.velocity = mote.velocity.clamp_length(max_speed);
                mote.mass = Mass::new(total);
            }

            self.scratch = nearby;
        }
    }

    /// 处理质体重叠。质量不同时开始渐进吸收，质量相同时弹开。
    fn resolve_motes(&mut self) {
        let restitution = self.config.collision_restitution;
        let max_speed = self.config.max_speed;

        for index in 0..self.motes.len() {
            if !self.motes[index].is_alive() {
                continue;
            }

            let position = self.motes[index].position;
            let mut nearby = std::mem::take(&mut self.scratch);
            self.mote_grid.nearby(position, &mut nearby);

            for other_index in nearby.iter().copied().map(|value| value as usize) {
                if other_index <= index || !self.motes[other_index].is_alive() {
                    continue;
                }

                let current_position = self.motes[index].position;
                let other_position = self.motes[other_index].position;
                let reach = self.motes[index].radius() + self.motes[other_index].radius();
                if wrap_delta(other_position - current_position).length_squared() > reach * reach {
                    continue;
                }

                let current_mass = self.motes[index].mass.get();
                let other_mass = self.motes[other_index].mass.get();

                if current_mass > other_mass + 1e-6 {
                    self.absorb_step(index, other_index, max_speed);
                } else if other_mass > current_mass + 1e-6 {
                    self.absorb_step(other_index, index, max_speed);
                    if !self.motes[index].is_alive() {
                        break;
                    }
                } else {
                    self.bounce(index, other_index, restitution, max_speed);
                }
            }

            self.scratch = nearby;
        }
    }

    /// 将失败质体的一小部分质量转移给胜利者。
    fn absorb_step(&mut self, winner: usize, loser: usize, max_speed: f32) {
        if winner == loser {
            return;
        }

        let winner_mass = self.motes[winner].mass.get();
        let loser_mass = self.motes[loser].mass.get();
        let distance =
            wrap_delta(self.motes[loser].position - self.motes[winner].position).length();
        let winner_radius = self.motes[winner].radius();
        let loser_radius = self.motes[loser].radius();
        let contained = distance <= winner_radius - loser_radius;
        let fraction = absorb_fraction(winner_radius, loser_radius, distance);
        // 完全包含时一次并入。部分重叠时，每个 60 Hz tick 转移一次。
        // 本项目每 tick 有两个物理子步，因此部分重叠每步取一半。
        let transfer = if contained {
            loser_mass - 1e-4
        } else {
            (loser_mass * fraction / SUBSTEPS as f32).min(loser_mass - 1e-4)
        };
        if transfer <= 0.0 {
            return;
        }

        self.motes[winner].mass = Mass::new(winner_mass + transfer);
        self.motes[loser].mass = Mass::new(loser_mass - transfer);
        self.motes[winner].velocity = (self.motes[winner].velocity * winner_mass
            + self.motes[loser].velocity * transfer)
            * (1.0 / (winner_mass + transfer));
        self.motes[winner].velocity = self.motes[winner].velocity.clamp_length(max_speed);

        if self.motes[loser].mass.get() <= self.config.min_mass {
            let remaining = self.motes[loser].mass.get();
            self.motes[winner].mass = Mass::new(self.motes[winner].mass.get() + remaining);
            self.motes[loser].state = MoteState::Absorbed;
            self.motes[winner].absorbed_motes += 1;
        }
    }

    fn bounce(&mut self, first: usize, second: usize, restitution: f32, max_speed: f32) {
        let first_position = self.motes[first].position;
        let second_position = self.motes[second].position;
        let first_velocity = self.motes[first].velocity;
        let second_velocity = self.motes[second].velocity;
        let first_mass = self.motes[first].mass.get();
        let second_mass = self.motes[second].mass.get();
        let first_radius = self.motes[first].radius();
        let second_radius = self.motes[second].radius();

        let delta = wrap_delta(second_position - first_position);
        let distance = delta.length();
        let normal = if distance > 1e-6 {
            delta * (1.0 / distance)
        } else {
            Vec2::new(1.0, 0.0)
        };

        let overlap = (first_radius + second_radius) - distance;
        if overlap > 0.0 {
            let offset = normal * (overlap * 0.5);
            self.motes[first].position -= offset;
            self.motes[second].position += offset;
        }

        self.motes[first].position = wrap_position(self.motes[first].position);
        self.motes[second].position = wrap_position(self.motes[second].position);

        let approach = (first_velocity - second_velocity).dot(normal);
        if approach <= 0.0 {
            return;
        }

        let impulse = -(1.0 + restitution) * approach / (1.0 / first_mass + 1.0 / second_mass);
        self.motes[first].velocity =
            (first_velocity + normal * (impulse / first_mass)).clamp_length(max_speed);
        self.motes[second].velocity =
            (second_velocity - normal * (impulse / second_mass)).clamp_length(max_speed);
    }

    fn expire_ejecta(&mut self) {
        let ttl = self.config.ejecta_ttl_ticks;
        let mut index = 0;
        while index < self.ejecta.len() {
            if self.ejecta[index].alive && self.ejecta[index].age >= ttl {
                self.convert_ejecta_to_pellet(index);
                self.ejecta.remove(index);
            } else {
                index += 1;
            }
        }
        self.ejecta.retain(|ejecta| ejecta.alive);
    }

    fn convert_ejecta_to_pellet(&mut self, index: usize) {
        if index >= self.ejecta.len() || !self.ejecta[index].alive {
            return;
        }
        let ejecta = &mut self.ejecta[index];
        ejecta.alive = false;
        let mass = ejecta.mass;
        let position = ejecta.position;
        self.add_pellet(mass, position);
    }

    fn add_pellet(&mut self, mass: Mass, position: Vec2) {
        if self.pellets.len() < self.config.max_pellets {
            self.pellets.push(Pellet::new(
                PelletId::new(self.next_pellet_id),
                mass,
                position,
            ));
            self.next_pellet_id = self.next_pellet_id.wrapping_add(1);
            return;
        }

        if let Some(nearest) = self
            .pellets
            .iter()
            .enumerate()
            .min_by(|(_, left), (_, right)| {
                wrap_delta(left.position - position)
                    .length_squared()
                    .total_cmp(&wrap_delta(right.position - position).length_squared())
            })
            .map(|(index, _)| index)
        {
            let total = self.pellets[nearest].mass.get() + mass.get();
            self.pellets[nearest].mass = Mass::new(total);
        }
    }

    fn remove_dead(&mut self) {
        let pending = self
            .motes
            .iter_mut()
            .filter(|mote| !mote.is_alive() && mote.pending_ejecta > 0.0)
            .map(|mote| {
                let mass = Mass::new(mote.pending_ejecta);
                mote.pending_ejecta = 0.0;
                (mass, mote.position)
            })
            .collect::<Vec<_>>();

        for (mass, position) in pending {
            self.add_pellet(mass, position);
        }

        self.motes.retain(|mote| mote.is_alive());
        self.pellets.retain(|pellet| pellet.alive);
    }
}

// === 几何工具 ===
/// 返回重叠面积比例。
///
/// 分子是半个弦长乘重叠深度，分母是小圆面积。结果再乘小圆质量，就是本次转移的质量。
fn absorb_fraction(winner_radius: f32, loser_radius: f32, distance: f32) -> f32 {
    let reach = winner_radius + loser_radius;
    if distance >= reach || loser_radius <= 0.0 {
        return 0.0;
    }
    if distance <= winner_radius - loser_radius {
        return 1.0;
    }

    let depth = reach - distance;
    let half_chord_squared =
        loser_radius * loser_radius - (distance - winner_radius) * (distance - winner_radius);
    if half_chord_squared <= 0.0 {
        return 1.0;
    }

    let half_chord = half_chord_squared.sqrt();
    (half_chord * depth / (PI * loser_radius * loser_radius)).clamp(0.0, 1.0)
}

fn random_position(rng: &mut StdRng) -> Vec2 {
    Vec2::new(rng.random_range(0.03..0.97), rng.random_range(0.03..0.97))
}

fn random_velocity(rng: &mut StdRng) -> Vec2 {
    let angle = rng.random_range(0.0..TAU);
    let speed = rng.random_range(0.0..0.01);
    Vec2::new(angle.cos() * speed, angle.sin() * speed)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::snapshot::FLAG_TARGET_REACHED;
    use osmos_core::brain::HIDDEN_NEURONS;
    use osmos_core::{ControlNetwork, Genome};

    fn config(motes: u32, pellets: u32) -> SimConfig {
        SimConfig {
            seed: 7,
            motes,
            pellets,
            pellet_reserve: 0.0,
            ..SimConfig::default()
        }
    }

    fn set_mote(world: &mut World, index: usize, position: Vec2, velocity: Vec2, mass: f32) {
        world.motes[index].position = position;
        world.motes[index].velocity = velocity;
        world.motes[index].mass = Mass::new(mass);
    }

    #[test]
    fn world_is_deterministic() {
        let mut first = World::new(config(8, 20));
        let mut second = World::new(config(8, 20));

        for _ in 0..120 {
            first.step();
            second.step();
        }

        let first = first.snapshot();
        let second = second.snapshot();
        assert_eq!(first.mote_ids, second.mote_ids);
        assert_eq!(first.mote_state, second.mote_state);
        assert_eq!(first.pellet_state, second.pellet_state);
    }

    #[test]
    fn mass_is_conserved() {
        let mut world = World::new(config(8, 20));
        let before = world.total_mass();

        for _ in 0..120 {
            world.step();
        }

        let after = world.total_mass();
        assert!((before - after).abs() <= before * 1e-4);
    }

    #[test]
    fn attractor_pulls_a_mote_toward_the_center() {
        let mut setup = config(1, 0);
        setup.gravity_strength = 0.002;
        let mut world = World::new(setup);
        set_mote(&mut world, 0, Vec2::new(0.7, 0.5), Vec2::ZERO, 1.0);

        world.step();

        assert!(world.motes[0].velocity.x < 0.0);
        assert_eq!(world.snapshot().well_state, vec![0.5, 0.5, 0.045, 1.0]);
    }

    #[test]
    fn passive_mote_can_orbit_the_attractor() {
        let mut setup = config(1, 0);
        setup.gravity_strength = 0.002;
        let mut world = World::new(setup);
        set_mote(
            &mut world,
            0,
            Vec2::new(0.75, 0.5),
            Vec2::new(0.0, 0.06),
            1.0,
        );

        for _ in 0..600 {
            world.step();
        }

        let radius = (world.motes[0].position - Vec2::new(0.5, 0.5)).length();
        assert!(radius > 0.08, "mote collapsed into the center: {radius}");
    }

    #[test]
    fn repulsor_pushes_a_mote_away_from_the_center() {
        let mut setup = config(1, 0);
        setup.gravity_strength = -0.002;
        let mut world = World::new(setup);
        set_mote(&mut world, 0, Vec2::new(0.7, 0.5), Vec2::ZERO, 1.0);

        world.step();

        assert!(world.motes[0].velocity.x > 0.0);
        assert_eq!(world.snapshot().well_state[3], -1.0);
    }

    #[test]
    fn pellet_reserve_replenishes_and_conserves_mass() {
        let mut setup = config(1, 1);
        setup.pellet_reserve = 1.0;
        let mut world = World::new(setup);
        set_mote(&mut world, 0, Vec2::new(0.5, 0.5), Vec2::ZERO, 1.0);
        world.pellets[0].position = Vec2::new(0.505, 0.5);
        let before = world.total_mass();

        world.step();

        assert_eq!(world.motes[0].absorbed_pellets, 1);
        assert_eq!(world.pellets.len(), 1);
        assert!((world.total_mass() - before).abs() < 1e-4);
    }

    #[test]
    fn default_reserve_keeps_pellets_available() {
        let mut world = World::new(SimConfig::default());
        for _ in 0..600 {
            world.step();
        }
        assert_eq!(world.pellets.len(), 236);
    }

    #[test]
    fn one_mote_absorbs_a_pellet() {
        let mut world = World::new(config(1, 1));
        set_mote(&mut world, 0, Vec2::new(0.5, 0.5), Vec2::ZERO, 1.0);
        world.pellets[0].position = Vec2::new(0.505, 0.5);

        world.step();

        assert_eq!(world.pellets.len(), 0);
        assert_eq!(world.motes.len(), 1);
        assert!((world.motes[0].mass.get() - 1.12).abs() < 1e-5);
    }

    #[test]
    fn absorption_works_across_the_wrapped_edge() {
        let mut world = World::new(config(2, 0));
        set_mote(&mut world, 0, Vec2::new(0.01, 0.5), Vec2::ZERO, 2.0);
        set_mote(&mut world, 1, Vec2::new(0.99, 0.5), Vec2::ZERO, 1.0);

        for _ in 0..100 {
            world.step();
        }

        assert_eq!(world.motes.len(), 1);
        assert!((world.motes[0].mass.get() - 3.0).abs() < 1e-5);
    }

    #[test]
    fn pellet_is_absorbed_across_the_wrapped_edge() {
        let mut world = World::new(config(1, 1));
        set_mote(&mut world, 0, Vec2::new(0.01, 0.5), Vec2::ZERO, 1.0);
        world.pellets[0].position = Vec2::new(0.995, 0.5);

        world.step();

        assert_eq!(world.pellets.len(), 0);
        assert!((world.motes[0].mass.get() - 1.12).abs() < 1e-5);
    }

    #[test]
    fn contained_mote_merges_in_one_tick() {
        let mut world = World::new(config(2, 0));
        set_mote(&mut world, 0, Vec2::new(0.5, 0.5), Vec2::ZERO, 2.0);
        set_mote(&mut world, 1, Vec2::new(0.505, 0.5), Vec2::ZERO, 0.5);

        world.step();

        assert_eq!(world.motes.len(), 1);
        assert!((world.motes[0].mass.get() - 2.5).abs() < 1e-4);
    }

    #[test]
    fn absorb_fraction_matches_overlap() {
        assert_eq!(absorb_fraction(0.2, 0.1, 0.3), 0.0);
        assert_eq!(absorb_fraction(0.2, 0.1, 0.05), 1.0);

        let fraction = absorb_fraction(0.2, 0.1, 0.25);
        let expected = 0.086_602_54 * 0.05 / (PI * 0.1 * 0.1);
        assert!((fraction - expected).abs() < 1e-6);
    }

    #[test]
    fn larger_mote_absorbs_smaller_and_keeps_mass() {
        let mut world = World::new(config(2, 0));
        set_mote(&mut world, 0, Vec2::new(0.5, 0.5), Vec2::ZERO, 2.0);
        set_mote(&mut world, 1, Vec2::new(0.505, 0.5), Vec2::ZERO, 1.0);

        for _ in 0..100 {
            world.step();
        }

        assert_eq!(world.motes.len(), 1);
        assert!((world.motes[0].mass.get() - 3.0).abs() < 1e-5);
    }

    #[test]
    fn equal_motes_bounce_instead_of_absorbing() {
        let mut world = World::new(config(2, 0));
        set_mote(
            &mut world,
            0,
            Vec2::new(0.5, 0.5),
            Vec2::new(0.05, 0.0),
            1.0,
        );
        set_mote(
            &mut world,
            1,
            Vec2::new(0.51, 0.5),
            Vec2::new(-0.05, 0.0),
            1.0,
        );

        world.step();

        assert_eq!(world.motes.len(), 2);
        assert!(world.motes[0].velocity.x <= 0.0);
        assert!(world.motes[1].velocity.x >= 0.0);
    }

    #[test]
    fn positions_stay_inside_the_world() {
        let mut world = World::new(config(16, 40));

        for _ in 0..240 {
            world.step();
            for mote in &world.motes {
                let radius = mote.radius();
                assert!(mote.position.x.is_finite());
                assert!(mote.position.y.is_finite());
                assert!(mote.position.x >= radius - 1e-4);
                assert!(mote.position.x <= 1.0 - radius + 1e-4);
                assert!(mote.position.y >= radius - 1e-4);
                assert!(mote.position.y <= 1.0 - radius + 1e-4);
            }
        }
    }

    #[test]
    fn snapshot_has_stable_strides() {
        let world = World::new(config(4, 7));
        let snapshot = world.snapshot();

        assert_eq!(snapshot.mote_count(), 4);
        assert_eq!(snapshot.mote_state.len(), 4 * MOTE_STATE_STRIDE);
        assert_eq!(snapshot.mote_role.len(), 4);
        assert_eq!(snapshot.mote_flags.len(), 4);
        assert_eq!(snapshot.pellet_count(), 7);
        assert_eq!(snapshot.pellet_state.len(), 7 * PELLET_STATE_STRIDE);
        assert_eq!(snapshot.stats.alive, 4);
        assert_eq!(snapshot.stats.pellets, 7);
    }

    #[test]
    fn target_flag_is_reserved_for_later() {
        let snapshot = World::new(config(1, 0)).snapshot();
        assert_eq!(snapshot.mote_flags[0] & FLAG_TARGET_REACHED, 0);
        assert_eq!(snapshot.mote_flags[0] & FLAG_ALIVE, FLAG_ALIVE);
    }

    #[test]
    fn larger_mote_gradually_absorbs_three_targets() {
        let mut world = World::new(config(3, 0));
        set_mote(&mut world, 0, Vec2::new(0.5, 0.5), Vec2::ZERO, 2.2);
        set_mote(&mut world, 1, Vec2::new(0.52, 0.5), Vec2::ZERO, 1.0);
        set_mote(&mut world, 2, Vec2::new(0.528, 0.5), Vec2::ZERO, 0.5);

        for _ in 0..3000 {
            world.step();
        }

        assert!(world.motes.len() <= 2);
        let total = world.motes.iter().map(|mote| mote.mass.get()).sum::<f32>();
        assert!((total - 3.7).abs() < 1e-4);
    }

    #[test]
    fn maximum_speed_does_not_tunnel_through_a_pellet() {
        let mut world = World::new(config(1, 1));
        set_mote(
            &mut world,
            0,
            Vec2::new(0.5, 0.5),
            Vec2::new(0.25, 0.0),
            0.05,
        );
        world.pellets[0].position = Vec2::new(0.508, 0.5);

        world.step();

        assert_eq!(world.pellets.len(), 0);
        assert!((world.motes[0].mass.get() - 0.17).abs() < 1e-5);
    }

    #[test]
    fn mote_wraps_at_the_right_edge() {
        let mut world = World::new(config(1, 0));
        set_mote(
            &mut world,
            0,
            Vec2::new(0.999, 0.5),
            Vec2::new(0.25, 0.0),
            1.0,
        );

        world.step();

        assert!(world.motes[0].velocity.x > 0.0);
        assert!(world.motes[0].position.x < 0.01);
    }

    #[test]
    fn controller_thrust_spends_mass_and_moves_the_mote() {
        let config = SimConfig {
            seed: 1,
            motes: 1,
            pellets: 0,
            ejecta_enabled: false,
            ..SimConfig::default()
        };
        let mut genome = Genome::zeros();
        genome.as_mut_slice()[HIDDEN_NEURONS + HIDDEN_NEURONS * SENSOR_INPUTS] = 2.0;
        let network = ControlNetwork::from_genome(&genome);
        let mut world = World::with_controllers(config, vec![network]);
        set_mote(&mut world, 0, Vec2::new(0.5, 0.5), Vec2::ZERO, 1.0);

        let before = world.total_mass();
        world.step();
        let after = world.total_mass();
        assert!((before - after - world.ejected_total()).abs() < 1e-5);
        assert!(world.motes[0].mass.get() < 1.0);
        assert!(world.motes[0].velocity.x > 0.0);
        assert!(world.motes[0].ejected_mass > 0.0);
    }

    #[test]
    fn ejecta_preserves_mass() {
        let config = SimConfig {
            seed: 2,
            motes: 1,
            pellets: 0,
            ..SimConfig::default()
        };
        let mut genome = Genome::zeros();
        genome.as_mut_slice()[HIDDEN_NEURONS + HIDDEN_NEURONS * SENSOR_INPUTS] = 2.0;
        let network = ControlNetwork::from_genome(&genome);
        let mut world = World::with_controllers(config, vec![network]);
        set_mote(&mut world, 0, Vec2::new(0.5, 0.5), Vec2::ZERO, 1.0);

        let before = world.total_mass();
        for _ in 0..120 {
            world.step();
        }
        let after = world.total_mass();

        assert!((before - after).abs() < 1e-4);
        assert!(world.ejected_total() > 0.0);
    }

    #[test]
    fn one_thousand_steps_have_a_stable_quantized_hash() {
        let config = SimConfig {
            seed: 42,
            motes: 64,
            pellets: 236,
            ..SimConfig::default()
        };
        let mut first = World::new(config);
        let mut second = World::new(config);
        let mut different = World::new(SimConfig { seed: 43, ..config });

        for _ in 0..1000 {
            first.step();
            second.step();
            different.step();
        }

        let hash = hash_snapshot(&first.snapshot());
        println!("quantized_hash={hash:016x}");
        assert_eq!(hash, hash_snapshot(&second.snapshot()));
        assert_ne!(hash, hash_snapshot(&different.snapshot()));
    }

    fn hash_snapshot(snapshot: &Snapshot) -> u64 {
        let mut hash = 14_695_981_039_346_656_037_u64;
        let mut write = |value: f32| {
            let quantized = (value * 10_000.0).round() as i64 as u64;
            hash ^= quantized;
            hash = hash.wrapping_mul(1_099_511_628_211);
        };

        for index in (0..snapshot.mote_state.len()).step_by(MOTE_STATE_STRIDE) {
            write(snapshot.mote_state[index]);
            write(snapshot.mote_state[index + 1]);
            write(snapshot.mote_state[index + 4]);
        }
        for index in (0..snapshot.pellet_state.len()).step_by(PELLET_STATE_STRIDE) {
            write(snapshot.pellet_state[index]);
            write(snapshot.pellet_state[index + 1]);
            write(snapshot.pellet_state[index + 2]);
        }

        hash
    }
}
