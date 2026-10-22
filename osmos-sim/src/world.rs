use std::f32::consts::TAU;

use osmos_core::{LineageId, Mass, MoteId, PelletId, Vec2};
use rand::rngs::StdRng;
use rand::{RngExt, SeedableRng};

use crate::config::{SimConfig, SUBSTEPS, TICKS_PER_SECOND};
use crate::grid::Grid;
use crate::mote::{Mote, MoteRole, MoteState};
use crate::pellet::Pellet;
use crate::snapshot::{Snapshot, Stats, FLAG_ALIVE, MOTE_STATE_STRIDE, PELLET_STATE_STRIDE};

const LINEAGES: u16 = 12;

pub struct World {
    config: SimConfig,
    tick: u64,
    pub motes: Vec<Mote>,
    pub pellets: Vec<Pellet>,
    mote_grid: Grid,
    pellet_grid: Grid,
    scratch: Vec<u32>,
}

impl World {
    pub fn new(config: SimConfig) -> Self {
        config.validate();

        let mut rng = StdRng::seed_from_u64(config.seed);
        let mut motes = Vec::with_capacity(config.motes as usize);
        let mut pellets = Vec::with_capacity(config.pellets as usize);

        for index in 0..config.motes {
            let angle = rng.random_range(0.0..TAU);
            let speed = rng.random_range(0.02..0.06);
            motes.push(Mote::new(
                MoteId::new(index),
                LineageId::new((index % LINEAGES as u32) as u16),
                MoteRole::Ai,
                Mass::new(config.mote_mass),
                random_position(&mut rng),
                Vec2::new(angle.cos() * speed, angle.sin() * speed),
            ));
        }

        for index in 0..config.pellets {
            pellets.push(Pellet::new(
                PelletId::new(index),
                Mass::new(config.pellet_mass),
                random_position(&mut rng),
            ));
        }

        Self {
            config,
            tick: 0,
            motes,
            pellets,
            mote_grid: Grid::new(),
            pellet_grid: Grid::new(),
            scratch: Vec::new(),
        }
    }

    pub fn config(&self) -> SimConfig {
        self.config
    }

    pub fn tick(&self) -> u64 {
        self.tick
    }

    pub fn step(&mut self) {
        self.tick += 1;
        let dt = 1.0 / (TICKS_PER_SECOND as f32 * SUBSTEPS as f32);

        for _ in 0..SUBSTEPS {
            self.integrate(dt);
            self.rebuild_grids();
            self.resolve_pellets();
            self.resolve_motes();
            self.remove_dead();
        }
    }

    pub fn total_mass(&self) -> f32 {
        self.motes
            .iter()
            .filter(|mote| mote.is_alive())
            .map(|mote| mote.mass.get())
            .sum::<f32>()
            + self
                .pellets
                .iter()
                .filter(|pellet| pellet.alive)
                .map(|pellet| pellet.mass.get())
                .sum::<f32>()
    }

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

        let alive = snapshot.mote_ids.len() as u32;
        let pellets = snapshot.pellet_state.len() as u32 / PELLET_STATE_STRIDE as u32;
        snapshot.stats = Stats {
            tick: self.tick,
            alive,
            pellets,
            best_mass,
            total_mass: self.total_mass(),
        };

        snapshot
    }

    fn integrate(&mut self, dt: f32) {
        let max_speed = self.config.max_speed;
        let wall_restitution = self.config.wall_restitution;

        for mote in &mut self.motes {
            if !mote.is_alive() {
                continue;
            }

            mote.position += mote.velocity * dt;
            mote.velocity = mote.velocity.clamp_length(max_speed);

            let radius = mote.radius();
            if mote.position.x < radius {
                mote.position.x = radius;
                if mote.velocity.x < 0.0 {
                    mote.velocity.x = -mote.velocity.x * wall_restitution;
                }
            } else if mote.position.x > 1.0 - radius {
                mote.position.x = 1.0 - radius;
                if mote.velocity.x > 0.0 {
                    mote.velocity.x = -mote.velocity.x * wall_restitution;
                }
            }

            if mote.position.y < radius {
                mote.position.y = radius;
                if mote.velocity.y < 0.0 {
                    mote.velocity.y = -mote.velocity.y * wall_restitution;
                }
            } else if mote.position.y > 1.0 - radius {
                mote.position.y = 1.0 - radius;
                if mote.velocity.y > 0.0 {
                    mote.velocity.y = -mote.velocity.y * wall_restitution;
                }
            }

            mote.velocity = mote.velocity.clamp_length(max_speed);
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
                if position.distance_squared(pellet.position) <= reach * reach {
                    gained += pellet.mass.get();
                    count += 1;
                    self.pellets[pellet_index].alive = false;
                }
            }

            if count > 0 {
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

    fn resolve_motes(&mut self) {
        let absorb_ratio = self.config.absorb_ratio;
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
                if current_position.distance_squared(other_position) > reach * reach {
                    continue;
                }

                let current_mass = self.motes[index].mass.get();
                let other_mass = self.motes[other_index].mass.get();

                if current_mass >= absorb_ratio * other_mass {
                    self.absorb(index, other_index, max_speed);
                } else if other_mass >= absorb_ratio * current_mass {
                    self.absorb(other_index, index, max_speed);
                    break;
                } else {
                    self.bounce(index, other_index, restitution, max_speed);
                }
            }

            self.scratch = nearby;
        }
    }

    fn absorb(&mut self, winner: usize, loser: usize, max_speed: f32) {
        if winner == loser {
            return;
        }

        let winner_mass = self.motes[winner].mass.get();
        let loser_mass = self.motes[loser].mass.get();
        let total_mass = winner_mass + loser_mass;
        let velocity = (self.motes[winner].velocity * winner_mass
            + self.motes[loser].velocity * loser_mass)
            * (1.0 / total_mass);

        self.motes[winner].mass = Mass::new(total_mass);
        self.motes[winner].velocity = velocity.clamp_length(max_speed);
        self.motes[winner].absorbed_motes += 1;
        self.motes[loser].state = MoteState::Absorbed;
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

        let delta = second_position - first_position;
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

        self.motes[first].position =
            clamp_to_world(self.motes[first].position, self.motes[first].radius());
        self.motes[second].position =
            clamp_to_world(self.motes[second].position, self.motes[second].radius());

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

    fn remove_dead(&mut self) {
        self.motes.retain(|mote| mote.is_alive());
        self.pellets.retain(|pellet| pellet.alive);
    }
}

fn random_position(rng: &mut StdRng) -> Vec2 {
    Vec2::new(rng.random_range(0.03..0.97), rng.random_range(0.03..0.97))
}

fn clamp_to_world(position: Vec2, radius: f32) -> Vec2 {
    Vec2::new(
        position.x.clamp(radius, 1.0 - radius),
        position.y.clamp(radius, 1.0 - radius),
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::snapshot::FLAG_TARGET_REACHED;

    fn config(motes: u32, pellets: u32) -> SimConfig {
        SimConfig {
            seed: 7,
            motes,
            pellets,
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
    fn larger_mote_absorbs_smaller_and_keeps_mass() {
        let mut world = World::new(config(2, 0));
        set_mote(&mut world, 0, Vec2::new(0.5, 0.5), Vec2::ZERO, 2.0);
        set_mote(&mut world, 1, Vec2::new(0.505, 0.5), Vec2::ZERO, 1.0);

        world.step();

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
    fn one_mote_can_absorb_three_targets_in_one_substep() {
        let mut world = World::new(config(3, 0));
        set_mote(&mut world, 0, Vec2::new(0.5, 0.5), Vec2::ZERO, 1.2);
        set_mote(&mut world, 1, Vec2::new(0.52, 0.5), Vec2::ZERO, 1.0);
        set_mote(&mut world, 2, Vec2::new(0.525, 0.5), Vec2::ZERO, 0.5);

        world.step();

        assert_eq!(world.motes.len(), 1);
        assert!((world.motes[0].mass.get() - 2.7).abs() < 1e-5);
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
    fn wall_bounce_keeps_the_mote_inside() {
        let mut world = World::new(config(1, 0));
        set_mote(
            &mut world,
            0,
            Vec2::new(0.99, 0.5),
            Vec2::new(0.25, 0.0),
            1.0,
        );

        world.step();

        assert!(world.motes[0].velocity.x < 0.0);
        assert!(world.motes[0].position.x <= 1.0 - world.motes[0].radius() + 1e-4);
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
