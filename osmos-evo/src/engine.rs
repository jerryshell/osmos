use osmos_core::brain::{ControlNetwork, Genome, SENSOR_INPUTS};
use osmos_sim::{SimConfig, Snapshot, World};
use rand::rngs::StdRng;
use rand::SeedableRng;

use crate::fitness::{median, score};
use crate::ga::{next_population, random_population};
use crate::scenario::Scenario;

#[derive(Clone, Debug)]
pub struct GenerationReport {
    pub generation: u32,
    pub best_mass: f32,
    pub median_mass: f32,
    pub mean_mass: f32,
    pub mean_speed: f32,
    pub ejected_mass: f32,
    pub alive: u32,
    pub best_score: f32,
}

#[derive(Clone, Debug)]
pub struct Inspect {
    pub id: u32,
    pub mass: f32,
    pub role: u8,
    pub sensor: [f32; SENSOR_INPUTS],
    pub thrust: [f32; 2],
    pub absorbed_pellets: u32,
    pub absorbed_motes: u32,
    pub ejected_mass: f32,
}

pub struct Engine {
    config: SimConfig,
    scenario: Scenario,
    hunters: u32,
    prey: u32,
    seed: u64,
    rng: StdRng,
    generation: u32,
    population: Vec<Genome>,
    best_genome: Genome,
    best_score: f32,
    world: World,
}

impl Engine {
    pub fn new(seed: u64) -> Self {
        Self::with_config(SimConfig {
            seed,
            ..SimConfig::default()
        })
    }

    pub fn with_config(config: SimConfig) -> Self {
        Self::build(config, Scenario::Ambient)
    }

    pub fn with_scenario(seed: u64, scenario: Scenario) -> Self {
        Self::with_scenario_config(seed, scenario, SimConfig::default())
    }

    pub fn with_scenario_config(seed: u64, scenario: Scenario, base: SimConfig) -> Self {
        let config = scenario.config(SimConfig { seed, ..base });
        Self::build(config, scenario)
    }

    fn build(config: SimConfig, scenario: Scenario) -> Self {
        let seed = config.seed;
        let mut rng = StdRng::seed_from_u64(seed ^ 0xA5A5_5A5A_1234_5678);
        let population = random_population(&mut rng, config.motes as usize);
        let best_genome = population[0].clone();
        let hunters = scenario.hunters();
        let prey = scenario.prey();
        let world = World::with_scenario(config, controllers(&population), hunters, prey);

        Self {
            config,
            scenario,
            hunters,
            prey,
            seed,
            rng,
            generation: 0,
            population,
            best_genome,
            best_score: f32::NEG_INFINITY,
            world,
        }
    }

    pub fn step(&mut self, steps: u32) {
        for _ in 0..steps {
            if self.world.tick() >= self.config.episode_ticks {
                self.evolve();
            }
            self.world.step();
            if self.world.tick() >= self.config.episode_ticks {
                self.evolve();
            }
        }
    }

    pub fn run_generation(&mut self) -> GenerationReport {
        while self.world.tick() < self.config.episode_ticks {
            self.world.step();
        }
        self.evolve()
    }

    pub fn snapshot(&self) -> Snapshot {
        let mut snapshot = self.world.snapshot();
        snapshot.stats.generation = self.generation;
        snapshot
    }

    pub fn tick(&self) -> u64 {
        self.world.tick()
    }

    pub fn generation(&self) -> u32 {
        self.generation
    }

    pub fn config(&self) -> SimConfig {
        self.config
    }

    pub fn scenario(&self) -> Scenario {
        self.scenario
    }

    pub fn inspect(&self, id: u32) -> Option<Inspect> {
        self.world
            .motes
            .iter()
            .find(|mote| mote.id.get() == id && mote.is_alive())
            .map(|mote| Inspect {
                id: mote.id.get(),
                mass: mote.mass.get(),
                role: mote.role.as_u8(),
                sensor: mote.sensor,
                thrust: [mote.thrust.x, mote.thrust.y],
                absorbed_pellets: mote.absorbed_pellets,
                absorbed_motes: mote.absorbed_motes,
                ejected_mass: mote.ejected_mass,
            })
    }

    pub fn reset(&mut self, seed: u64) {
        self.seed = seed;
        self.config.seed = seed;
        self.generation = 0;
        self.rng = StdRng::seed_from_u64(seed ^ 0xA5A5_5A5A_1234_5678);
        self.population = random_population(&mut self.rng, self.config.motes as usize);
        self.best_genome = self.population[0].clone();
        self.best_score = f32::NEG_INFINITY;
        self.world = World::with_scenario(
            self.config,
            controllers(&self.population),
            self.hunters,
            self.prey,
        );
    }

    fn evolve(&mut self) -> GenerationReport {
        let evaluation = self.evaluate();
        let report = evaluation.report.clone();

        if report.best_score > self.best_score {
            self.best_score = report.best_score;
            if let Some(index) = evaluation.best_index {
                self.best_genome = self.population[index].clone();
            }
        }

        self.population = next_population(&mut self.rng, &self.population, &evaluation.scores);
        self.generation += 1;

        let mut next_config = self.config;
        next_config.seed = self.seed;
        self.world = World::with_scenario(
            next_config,
            controllers(&self.population),
            self.hunters,
            self.prey,
        );

        report
    }

    fn evaluate(&self) -> Evaluation {
        let mut masses = vec![0.0_f32; self.population.len()];
        for mote in &self.world.motes {
            if mote.is_alive() && (mote.id.get() as usize) < masses.len() {
                masses[mote.id.get() as usize] = mote.mass.get();
            }
        }

        let target = self.config.target_mass;
        let scores = masses
            .iter()
            .map(|mass| score(*mass, target))
            .collect::<Vec<_>>();

        let best_index = scores
            .iter()
            .enumerate()
            .max_by(|(_, left), (_, right)| left.total_cmp(right))
            .filter(|(_, score)| score.is_finite())
            .map(|(index, _)| index);
        let best_score = best_index.map(|index| scores[index]).unwrap_or(0.0);
        let best_mass = masses.iter().copied().fold(0.0_f32, f32::max);
        let median_mass = median(&masses);
        let alive_motes = self
            .world
            .motes
            .iter()
            .filter(|mote| mote.is_alive())
            .collect::<Vec<_>>();
        let alive = alive_motes.len() as u32;
        let mean_mass = if alive == 0 {
            0.0
        } else {
            alive_motes.iter().map(|mote| mote.mass.get()).sum::<f32>() / alive as f32
        };
        let mean_speed = if alive == 0 {
            0.0
        } else {
            alive_motes
                .iter()
                .map(|mote| mote.velocity.length())
                .sum::<f32>()
                / alive as f32
        };
        let ejected_mass = self.world.ejected_total();

        Evaluation {
            report: GenerationReport {
                generation: self.generation,
                best_mass,
                median_mass,
                mean_mass,
                mean_speed,
                ejected_mass,
                alive,
                best_score,
            },
            scores,
            best_index,
        }
    }
}

struct Evaluation {
    report: GenerationReport,
    scores: Vec<f32>,
    best_index: Option<usize>,
}

fn controllers(population: &[Genome]) -> Vec<ControlNetwork> {
    population.iter().map(ControlNetwork::from_genome).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn small_config() -> SimConfig {
        SimConfig {
            seed: 11,
            motes: 8,
            pellets: 16,
            episode_ticks: 30,
            ..SimConfig::default()
        }
    }

    #[test]
    fn same_seed_produces_the_same_snapshot() {
        let mut first = Engine::new(11);
        let mut second = Engine::new(11);

        first.step(120);
        second.step(120);

        let first = first.snapshot();
        let second = second.snapshot();

        assert_eq!(first.mote_ids, second.mote_ids);
        assert_eq!(first.mote_state, second.mote_state);
        assert_eq!(first.pellet_state, second.pellet_state);
    }

    #[test]
    fn tick_advances_by_the_requested_steps() {
        let mut engine = Engine::new(3);
        engine.step(7);
        assert_eq!(engine.tick(), 7);
    }

    #[test]
    fn reset_restarts_from_the_seed() {
        let mut engine = Engine::new(5);
        let initial = engine.snapshot();
        engine.step(30);
        engine.reset(5);
        let reset = engine.snapshot();

        assert_eq!(initial.mote_state, reset.mote_state);
        assert_eq!(initial.pellet_state, reset.pellet_state);
    }

    #[test]
    fn generation_advances_and_rebuilds_the_world() {
        let mut engine = Engine::with_config(small_config());
        let report = engine.run_generation();

        assert_eq!(report.generation, 0);
        assert_eq!(engine.generation(), 1);
        assert_eq!(engine.tick(), 0);
        assert_eq!(engine.snapshot().stats.generation, 1);
    }

    #[test]
    fn scenario_spawns_hunters_and_prey() {
        let engine = Engine::with_scenario(7, Scenario::Sentient);
        let snapshot = engine.snapshot();

        assert_eq!(snapshot.stats.alive, 48 + 4 + 12);
        assert_eq!(
            snapshot.mote_role.iter().filter(|role| **role == 1).count(),
            4
        );
        assert_eq!(
            snapshot.mote_role.iter().filter(|role| **role == 2).count(),
            12
        );
    }
}
