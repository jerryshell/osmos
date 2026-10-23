use osmos_core::{wrap_delta, ControlNetwork, Genome};
use osmos_sim::{MoteRole, SimConfig, World};
use rand::rngs::StdRng;
use rand::{RngExt, SeedableRng};

use crate::ga::{next_population_by_mass, random_population};

#[derive(Clone, Debug, Default)]
pub struct EcologyReport {
    pub generation: u32,
    pub best_mass: f32,
    pub mean_mass: f32,
    pub survivors: u32,
    pub predation: u32,
    pub flee_events: u32,
    pub hunt_events: u32,
    pub behavior_samples: u32,
}

pub struct EcologyTrainer {
    config: SimConfig,
    rng: StdRng,
    generation: u32,
    population: Vec<Genome>,
}

impl EcologyTrainer {
    pub fn new(seed: u64) -> Self {
        Self::with_config(SimConfig {
            seed,
            ..SimConfig::default()
        })
    }

    pub fn with_config(config: SimConfig) -> Self {
        let mut rng = StdRng::seed_from_u64(config.seed ^ 0xEC01_0A11_2026_1023);
        let population = random_population(&mut rng, config.motes as usize);
        Self {
            config,
            rng,
            generation: 0,
            population,
        }
    }

    pub fn generation(&self) -> u32 {
        self.generation
    }

    pub fn population(&self) -> &[Genome] {
        &self.population
    }

    pub fn run_generation(&mut self) -> EcologyReport {
        let masses = (0..self.config.motes)
            .map(|_| self.rng.random_range(0.4..3.0))
            .collect::<Vec<_>>();
        let controllers = self
            .population
            .iter()
            .map(ControlNetwork::from_genome)
            .collect::<Vec<_>>();
        let mut world = World::with_ecology(self.config, controllers, &masses);

        let mut flee_events = 0;
        let mut hunt_events = 0;
        let mut behavior_samples = 0;

        for tick in 0..self.config.episode_ticks {
            world.step();
            if tick % 10 == 0 {
                let (flee, hunt, samples) = sample_behavior(&world);
                flee_events += flee;
                hunt_events += hunt;
                behavior_samples += samples;
            }
        }

        let mut final_masses = vec![0.0_f32; self.population.len()];
        let mut predation = 0;
        for mote in &world.motes {
            if mote.is_alive() && (mote.id.get() as usize) < final_masses.len() {
                final_masses[mote.id.get() as usize] = mote.mass.get();
                predation += mote.absorbed_motes;
            }
        }

        let survivors = final_masses.iter().filter(|mass| **mass > 0.0).count() as u32;
        let best_mass = final_masses.iter().copied().fold(0.0, f32::max);
        let mean_mass = if survivors == 0 {
            0.0
        } else {
            final_masses.iter().sum::<f32>() / survivors as f32
        };

        let report = EcologyReport {
            generation: self.generation,
            best_mass,
            mean_mass,
            survivors,
            predation,
            flee_events,
            hunt_events,
            behavior_samples,
        };

        self.population = next_population_by_mass(&mut self.rng, &self.population, &final_masses);
        self.generation += 1;
        report
    }
}

fn sample_behavior(world: &World) -> (u32, u32, u32) {
    let mut flee = 0;
    let mut hunt = 0;
    let mut samples = 0;
    let range_squared = world.config().sensor_range * world.config().sensor_range;

    for mote in &world.motes {
        if !mote.is_alive() || mote.role != MoteRole::Ai {
            continue;
        }
        let mut larger: Option<(f32, osmos_core::Vec2)> = None;
        let mut smaller: Option<(f32, osmos_core::Vec2)> = None;

        for other in &world.motes {
            if other.id == mote.id || !other.is_alive() {
                continue;
            }
            let delta = wrap_delta(other.position - mote.position);
            let distance_squared = delta.length_squared();
            if distance_squared <= 1e-8 || distance_squared > range_squared {
                continue;
            }
            if other.mass.get() >= mote.mass.get() * world.config().absorb_ratio {
                if larger
                    .map(|(best, _)| distance_squared < best)
                    .unwrap_or(true)
                {
                    larger = Some((distance_squared, delta));
                }
            } else if mote.mass.get() >= other.mass.get() * world.config().absorb_ratio
                && smaller
                    .map(|(best, _)| distance_squared < best)
                    .unwrap_or(true)
            {
                smaller = Some((distance_squared, delta));
            }
        }

        if let Some((_, delta)) = larger {
            if mote.velocity.dot(delta.normalize_or_zero()) < -0.01 {
                flee += 1;
            }
        }
        if let Some((_, delta)) = smaller {
            if mote.velocity.dot(delta.normalize_or_zero()) > 0.01 {
                hunt += 1;
            }
        }
        samples += 1;
    }

    (flee, hunt, samples)
}

#[cfg(test)]
mod tests {
    use super::*;
    use osmos_sim::SimConfig;

    #[test]
    fn one_generation_keeps_the_population_size() {
        let mut trainer = EcologyTrainer::with_config(SimConfig {
            seed: 5,
            motes: 4,
            pellets: 8,
            episode_ticks: 20,
            ..SimConfig::default()
        });

        let report = trainer.run_generation();

        assert!(report.survivors <= 4);
        assert_eq!(trainer.population().len(), 4);
        assert_eq!(trainer.generation(), 1);
    }
}
