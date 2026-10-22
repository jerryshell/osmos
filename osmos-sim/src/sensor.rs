use std::f32::consts::{PI, TAU};

use osmos_core::{wrap_delta, Vec2, SENSOR_INPUTS};

use crate::ejecta::Ejecta;
use crate::grid::Grid;
use crate::mote::Mote;
use crate::pellet::Pellet;

pub const SECTORS: usize = 16;
const CHANNEL_OFFSET: usize = 16;
const SELF_OFFSET: usize = SECTORS * 3;
const THREAT_OFFSET: usize = SELF_OFFSET + 3;
const FOOD_OFFSET: usize = SELF_OFFSET + 5;

pub struct SenseContext<'a> {
    motes: &'a [Mote],
    pellets: &'a [Pellet],
    ejecta: &'a [Ejecta],
    mote_grid: &'a Grid,
    pellet_grid: &'a Grid,
    ejecta_grid: &'a Grid,
    range: f32,
    max_speed: f32,
    episode_ticks: u64,
}

impl<'a> SenseContext<'a> {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        motes: &'a [Mote],
        pellets: &'a [Pellet],
        ejecta: &'a [Ejecta],
        mote_grid: &'a Grid,
        pellet_grid: &'a Grid,
        ejecta_grid: &'a Grid,
        range: f32,
        max_speed: f32,
        episode_ticks: u64,
    ) -> Self {
        Self {
            motes,
            pellets,
            ejecta,
            mote_grid,
            pellet_grid,
            ejecta_grid,
            range,
            max_speed,
            episode_ticks,
        }
    }

    pub fn sense(
        &self,
        index: usize,
        tick: u64,
        nearby: &mut Vec<u32>,
        out: &mut [f32; SENSOR_INPUTS],
    ) {
        out.fill(0.0);

        let me = Body::from(&self.motes[index]);

        self.mote_grid.nearby(me.position, nearby);
        for other_index in nearby.iter().copied().map(|value| value as usize) {
            if other_index == index || !self.motes[other_index].is_alive() {
                continue;
            }
            self.add(out, me, Body::from(&self.motes[other_index]));
        }

        self.pellet_grid.nearby(me.position, nearby);
        for pellet_index in nearby.iter().copied().map(|value| value as usize) {
            let pellet = &self.pellets[pellet_index];
            if !pellet.alive {
                continue;
            }
            self.add(out, me, Body::from(pellet));
        }

        self.ejecta_grid.nearby(me.position, nearby);
        for ejecta_index in nearby.iter().copied().map(|value| value as usize) {
            let ejecta = &self.ejecta[ejecta_index];
            if !ejecta.alive {
                continue;
            }
            self.add(out, me, Body::from(ejecta));
        }

        for value in &mut out[..SELF_OFFSET] {
            *value = value.tanh();
        }

        out[SELF_OFFSET] = (me.mass / (me.mass + 1.0)) * 2.0 - 1.0;
        out[SELF_OFFSET + 1] = (me.velocity.length() / self.max_speed).clamp(0.0, 1.0) * 2.0 - 1.0;
        out[SELF_OFFSET + 2] = me.position.x * 2.0 - 1.0;
        out[SELF_OFFSET + 4] = me.position.y * 2.0 - 1.0;
        out[SELF_OFFSET + 6] = if self.episode_ticks == 0 {
            0.0
        } else {
            1.0 - 2.0 * (tick.min(self.episode_ticks) as f32 / self.episode_ticks as f32)
        };
    }

    fn add(&self, out: &mut [f32; SENSOR_INPUTS], me: Body, other: Body) {
        let delta = wrap_delta(other.position - me.position);
        let distance = delta.length();
        if distance <= 1e-6 || distance > self.range {
            return;
        }

        let direction = delta * (1.0 / distance);
        let weight = (1.0 - distance / self.range).powi(2);
        let sector = sector_of(direction);
        let mass_delta = (other.mass - me.mass) / (other.mass + me.mass);
        let radial = (other.velocity - me.velocity).dot(direction) / (2.0 * self.max_speed);

        out[sector] += mass_delta * weight;
        out[CHANNEL_OFFSET + sector] += weight;
        out[CHANNEL_OFFSET * 2 + sector] += radial.clamp(-1.0, 1.0) * weight;

        if other.mass > me.mass {
            out[THREAT_OFFSET] = out[THREAT_OFFSET].max(weight);
        } else if other.mass < me.mass {
            out[FOOD_OFFSET] = out[FOOD_OFFSET].max(weight);
        }
    }
}

#[derive(Clone, Copy)]
struct Body {
    position: Vec2,
    mass: f32,
    velocity: Vec2,
}

impl From<&Mote> for Body {
    fn from(mote: &Mote) -> Self {
        Self {
            position: mote.position,
            mass: mote.mass.get(),
            velocity: mote.velocity,
        }
    }
}

impl From<&Ejecta> for Body {
    fn from(ejecta: &Ejecta) -> Self {
        Self {
            position: ejecta.position,
            mass: ejecta.mass.get(),
            velocity: ejecta.velocity,
        }
    }
}

impl From<&Pellet> for Body {
    fn from(pellet: &Pellet) -> Self {
        Self {
            position: pellet.position,
            mass: pellet.mass.get(),
            velocity: Vec2::ZERO,
        }
    }
}

fn sector_of(direction: Vec2) -> usize {
    let angle = direction.y.atan2(direction.x);
    let normalized = (angle + PI) / TAU;
    ((normalized * SECTORS as f32).floor() as isize).rem_euclid(SECTORS as isize) as usize
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mote::MoteRole;
    use osmos_core::{LineageId, Mass, MoteId, PelletId};

    fn mote(position: Vec2, mass: f32) -> Mote {
        Mote::new(
            MoteId::new(0),
            LineageId::new(0),
            MoteRole::Ai,
            Mass::new(mass),
            position,
            Vec2::ZERO,
        )
    }

    fn pellet(position: Vec2, mass: f32) -> Pellet {
        Pellet::new(PelletId::new(0), Mass::new(mass), position)
    }

    fn context<'a>(
        motes: &'a [Mote],
        pellets: &'a [Pellet],
        mote_grid: &'a Grid,
        pellet_grid: &'a Grid,
        ejecta_grid: &'a Grid,
    ) -> SenseContext<'a> {
        SenseContext::new(
            motes,
            pellets,
            &[],
            mote_grid,
            pellet_grid,
            ejecta_grid,
            0.25,
            0.25,
            2400,
        )
    }

    #[test]
    fn smaller_pellet_on_the_right_is_food() {
        let me = mote(Vec2::new(0.5, 0.5), 1.0);
        let food = pellet(Vec2::new(0.55, 0.5), 0.12);
        let mut mote_grid = Grid::new();
        mote_grid.insert(0, me.position);
        let mut pellet_grid = Grid::new();
        pellet_grid.insert(0, food.position);
        let ejecta_grid = Grid::new();
        let mut out = [0.0; SENSOR_INPUTS];
        let mut nearby = Vec::new();

        context(
            std::slice::from_ref(&me),
            std::slice::from_ref(&food),
            &mote_grid,
            &pellet_grid,
            &ejecta_grid,
        )
        .sense(0, 0, &mut nearby, &mut out);

        assert!(out[8] < 0.0);
        assert!(out[16 + 8] > 0.0);
        assert!(out[FOOD_OFFSET] > 0.0);
    }

    #[test]
    fn wrapped_food_on_the_left_edge_is_still_on_the_right() {
        let me = mote(Vec2::new(0.99, 0.5), 1.0);
        let food = pellet(Vec2::new(0.01, 0.5), 0.12);
        let mut mote_grid = Grid::new();
        mote_grid.insert(0, me.position);
        let mut pellet_grid = Grid::new();
        pellet_grid.insert(0, food.position);
        let ejecta_grid = Grid::new();
        let mut out = [0.0; SENSOR_INPUTS];
        let mut nearby = Vec::new();

        context(
            std::slice::from_ref(&me),
            std::slice::from_ref(&food),
            &mote_grid,
            &pellet_grid,
            &ejecta_grid,
        )
        .sense(0, 0, &mut nearby, &mut out);

        assert!(out[8] < 0.0);
        assert!(out[FOOD_OFFSET] > 0.0);
    }

    #[test]
    fn larger_mote_on_the_right_is_a_threat() {
        let me = mote(Vec2::new(0.5, 0.5), 1.0);
        let other = mote(Vec2::new(0.55, 0.5), 3.0);
        let mut mote_grid = Grid::new();
        mote_grid.insert(0, me.position);
        mote_grid.insert(1, other.position);
        let pellet_grid = Grid::new();
        let ejecta_grid = Grid::new();
        let mut out = [0.0; SENSOR_INPUTS];
        let mut nearby = Vec::new();

        context(&[me, other], &[], &mote_grid, &pellet_grid, &ejecta_grid).sense(
            0,
            0,
            &mut nearby,
            &mut out,
        );

        assert!(out[8] > 0.0);
        assert!(out[THREAT_OFFSET] > 0.0);
    }

    #[test]
    fn empty_threat_and_food_channels_are_centered() {
        let me = mote(Vec2::new(0.5, 0.5), 1.0);
        let mut grid = Grid::new();
        grid.insert(0, me.position);
        let empty = Grid::new();
        let ejecta_grid = Grid::new();
        let mut out = [0.0; SENSOR_INPUTS];
        let mut nearby = Vec::new();

        context(std::slice::from_ref(&me), &[], &grid, &empty, &ejecta_grid).sense(
            0,
            0,
            &mut nearby,
            &mut out,
        );

        assert_eq!(out[50], 0.0);
        assert_eq!(out[51], 0.0);
        assert_eq!(out[52], 0.0);
        assert_eq!(out[53], 0.0);
    }

    #[test]
    fn edge_position_has_no_false_wall_signal() {
        let me = mote(Vec2::new(0.01, 0.5), 1.0);
        let mut grid = Grid::new();
        grid.insert(0, me.position);
        let empty = Grid::new();
        let ejecta_grid = Grid::new();
        let mut out = [0.0; SENSOR_INPUTS];
        let mut nearby = Vec::new();

        context(std::slice::from_ref(&me), &[], &grid, &empty, &ejecta_grid).sense(
            0,
            0,
            &mut nearby,
            &mut out,
        );

        assert!(out[50] < 0.0);
        assert_eq!(out[51], 0.0);
        assert_eq!(out[53], 0.0);
    }
}
