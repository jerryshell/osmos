use osmos_core::brain::{ControlNetwork, Genome};
use osmos_core::wrap_delta;
use osmos_sim::{MoteRole, SimConfig, World};
use rand::rngs::StdRng;
use rand::SeedableRng;

use crate::fitness::{mean, median, score};
use crate::ga::{next_population, random_population};

const SEEDS_PER_GENOME: usize = 2;
#[derive(Clone, Debug)]
pub struct ProbeReport {
    pub seed: u64,
    pub predator_mass: f32,
    pub predator_absorbed_motes: u32,
    pub predator_absorbed_pellets: u32,
    pub prey_survived: bool,
    pub prey_alive_ticks: u64,
    pub prey_mass: f32,
    pub prey_min_hunter_distance: f32,
    pub prey_mean_hunter_distance: f32,
}

struct ProbeRole {
    survived: bool,
    mass: f32,
    alive_ticks: u64,
    absorbed_motes: u32,
    absorbed_pellets: u32,
    min_hunter_distance: f32,
    mean_hunter_distance: f32,
}

#[derive(Clone, Debug, Default)]
pub struct TrainerReport {
    pub generation: u32,
    pub best_mass: f32,
    pub median_mass: f32,
    pub mean_mass: f32,
    pub best_score: f32,
    pub mean_score: f32,
    pub target_rate: f32,
    pub survival_rate: f32,
}

/// Single-learner baseline with scripted rivals. The main ecology path is
/// `EcologyTrainer`.
pub struct Trainer {
    config: SimConfig,
    seed: u64,
    rng: StdRng,
    generation: u32,
    population: Vec<Genome>,
    best_genome: Genome,
    best_score: f32,
}

impl Trainer {
    pub fn new(seed: u64) -> Self {
        Self::with_config(SimConfig {
            seed,
            ..SimConfig::default()
        })
    }

    pub fn with_config(config: SimConfig) -> Self {
        let mut rng = StdRng::seed_from_u64(config.seed ^ 0x5A5A_A5A5_2468_1357);
        let population = random_population(&mut rng, config.motes as usize);
        let best_genome = population[0].clone();

        Self {
            config,
            seed: config.seed,
            rng,
            generation: 0,
            population,
            best_genome,
            best_score: f32::NEG_INFINITY,
        }
    }

    pub fn run_generation(&mut self) -> TrainerReport {
        let report = self.evaluate_population();
        self.population = next_population(&mut self.rng, &self.population, &report.scores);
        self.generation += 1;
        report.report
    }

    pub fn generation(&self) -> u32 {
        self.generation
    }

    pub fn population(&self) -> &[Genome] {
        &self.population
    }

    pub fn best_genome(&self) -> &Genome {
        &self.best_genome
    }

    pub fn best_score(&self) -> f32 {
        self.best_score
    }

    pub fn evaluate_genome(&self, genome: &Genome, seeds: &[u64]) -> f32 {
        if seeds.is_empty() {
            return 0.0;
        }
        let total = seeds
            .iter()
            .map(|seed| self.evaluate_on_seed(genome, *seed))
            .sum::<f32>();
        total / seeds.len() as f32
    }

    pub fn probe_best(&self, seed: u64) -> ProbeReport {
        let predator = self.probe_role(seed, true);
        let prey = self.probe_role(seed ^ 0xB22D_F602, false);
        ProbeReport {
            seed,
            predator_mass: predator.mass,
            predator_absorbed_motes: predator.absorbed_motes,
            predator_absorbed_pellets: predator.absorbed_pellets,
            prey_survived: prey.survived,
            prey_alive_ticks: prey.alive_ticks,
            prey_mass: prey.mass,
            prey_min_hunter_distance: prey.min_hunter_distance,
            prey_mean_hunter_distance: prey.mean_hunter_distance,
        }
    }

    fn probe_role(&self, seed: u64, predator: bool) -> ProbeRole {
        let config = SimConfig {
            seed,
            motes: 1,
            pellets: 0,
            episode_ticks: self.config.episode_ticks,
            initial_mass_spread: 0.0,
            hunter_speed: 0.03,
            prey_speed: 0.02,
            ..self.config
        };
        let controller = ControlNetwork::from_genome(&self.best_genome);
        let mut world = if predator {
            World::with_predator_training(config, vec![controller])
        } else {
            World::with_prey_training(config, vec![controller])
        };
        let mut alive_ticks = 0;
        let mut min_hunter_distance = f32::INFINITY;
        let mut distance_sum = 0.0;
        let mut distance_samples = 0;
        for tick in 0..config.episode_ticks {
            world.step();
            let Some(learner) = world.motes.iter().find(|mote| mote.id.get() == 0) else {
                break;
            };
            alive_ticks = tick + 1;
            if let Some(hunter) = world
                .motes
                .iter()
                .find(|mote| mote.role == MoteRole::Hunter)
            {
                let distance = wrap_delta(hunter.position - learner.position).length();
                min_hunter_distance = min_hunter_distance.min(distance);
                distance_sum += distance;
                distance_samples += 1;
            }
        }
        let learner = world.motes.iter().find(|mote| mote.id.get() == 0);
        ProbeRole {
            survived: learner.is_some(),
            mass: learner.map(|mote| mote.mass.get()).unwrap_or(0.0),
            alive_ticks,
            absorbed_motes: learner.map(|mote| mote.absorbed_motes).unwrap_or(0),
            absorbed_pellets: learner.map(|mote| mote.absorbed_pellets).unwrap_or(0),
            min_hunter_distance: if min_hunter_distance.is_finite() {
                min_hunter_distance
            } else {
                0.0
            },
            mean_hunter_distance: if distance_samples > 0 {
                distance_sum / distance_samples as f32
            } else {
                0.0
            },
        }
    }

    pub fn evaluate_genome_mass(&self, genome: &Genome, seeds: &[u64]) -> f32 {
        if seeds.is_empty() {
            return 0.0;
        }
        seeds
            .iter()
            .map(|seed| self.run_episode(genome, *seed))
            .sum::<f32>()
            / seeds.len() as f32
    }

    fn evaluate_population(&mut self) -> Evaluation {
        let mut scores = Vec::with_capacity(self.population.len());
        let mut masses = Vec::with_capacity(self.population.len());
        let mut survivals = Vec::with_capacity(self.population.len());

        let generation_seed =
            self.seed ^ (self.generation as u64).wrapping_mul(0x9E37_79B9_7F4A_7C15);
        let seeds = [
            generation_seed ^ 0x1111_1111_1111_1111,
            generation_seed ^ 0x2222_2222_2222_2222,
        ];

        for genome in &self.population {
            let mut total_score = 0.0;
            let mut total_mass = 0.0;
            for seed in seeds {
                let episode = self.run_episode_stats(genome, seed);
                total_mass += episode.mass;
                total_score += episode.fitness;
                survivals.push(episode.survived);
            }

            let mass = total_mass / SEEDS_PER_GENOME as f32;
            let score = total_score / SEEDS_PER_GENOME as f32;
            masses.push(mass);
            scores.push(score);
        }

        let best_index = scores
            .iter()
            .enumerate()
            .max_by(|(_, left), (_, right)| left.total_cmp(right))
            .map(|(index, _)| index);
        let best_score = best_index.map(|index| scores[index]).unwrap_or(0.0);
        if best_score > self.best_score {
            self.best_score = best_score;
            if let Some(index) = best_index {
                self.best_genome = self.population[index].clone();
            }
        }

        Evaluation {
            report: TrainerReport {
                generation: self.generation,
                best_mass: masses.iter().copied().fold(0.0, f32::max),
                median_mass: median(&masses),
                mean_mass: mean(&masses),
                best_score,
                mean_score: mean(&scores),
                target_rate: masses
                    .iter()
                    .filter(|mass| **mass >= self.config.target_mass)
                    .count() as f32
                    / masses.len() as f32,
                survival_rate: survivals.iter().filter(|survived| **survived).count() as f32
                    / survivals.len() as f32,
            },
            scores,
        }
    }

    fn run_episode(&self, genome: &Genome, seed: u64) -> f32 {
        self.run_episode_stats(genome, seed).mass
    }

    fn run_episode_stats(&self, genome: &Genome, seed: u64) -> EpisodeStats {
        let predator = self.run_role(genome, seed ^ 0xA11C_E501, true);
        let prey = self.run_role(genome, seed ^ 0xB22D_F602, false);
        EpisodeStats {
            mass: (predator.mass + prey.mass) * 0.5,
            fitness: (predator.fitness + prey.fitness) * 0.5,
            survived: predator.survived && prey.survived,
        }
    }

    fn run_role(&self, genome: &Genome, seed: u64, predator: bool) -> EpisodeStats {
        let config = SimConfig {
            seed,
            motes: 1,
            pellets: 0,
            episode_ticks: self.config.episode_ticks,
            initial_mass_spread: 0.0,
            hunter_speed: 0.03,
            prey_speed: 0.02,
            ..self.config
        };
        let controller = ControlNetwork::from_genome(genome);
        let mut world = if predator {
            World::with_predator_training(config, vec![controller])
        } else {
            World::with_prey_training(config, vec![controller])
        };
        let mut alive_ticks = 0_u64;
        let mut hunter_distance_sum = 0.0;
        let mut hunter_distance_samples = 0;
        let mut prey_distance_sum = 0.0;
        let mut prey_distance_samples = 0;
        for tick in 0..config.episode_ticks {
            world.step();
            let Some(learner) = world.motes.iter().find(|mote| mote.id.get() == 0) else {
                break;
            };
            alive_ticks = tick + 1;
            if let Some(hunter) = world
                .motes
                .iter()
                .find(|mote| mote.role == MoteRole::Hunter)
            {
                hunter_distance_sum += wrap_delta(hunter.position - learner.position).length();
                hunter_distance_samples += 1;
            }
            for prey in world
                .motes
                .iter()
                .filter(|mote| mote.role == MoteRole::Prey)
            {
                prey_distance_sum += wrap_delta(prey.position - learner.position).length();
                prey_distance_samples += 1;
            }
        }
        let learner = world.motes.iter().find(|mote| mote.id.get() == 0);
        let mass = learner.map(|mote| mote.mass.get()).unwrap_or(0.0);
        let absorbed = learner.map(|mote| mote.absorbed_motes).unwrap_or(0);
        let survival = alive_ticks as f32 / config.episode_ticks.max(1) as f32;
        let survival_weight = if predator { 0.5 } else { 2.0 };
        let safety = if predator || hunter_distance_samples == 0 {
            0.0
        } else {
            hunter_distance_sum / hunter_distance_samples as f32
        };
        let pursuit = if predator && prey_distance_samples > 0 {
            (1.0 - prey_distance_sum / prey_distance_samples as f32).max(0.0)
        } else {
            0.0
        };
        EpisodeStats {
            mass,
            fitness: score(mass, self.config.target_mass)
                + survival * survival_weight
                + absorbed as f32 * 0.5
                + safety * (if predator { 0.0 } else { 1.0 })
                + pursuit * 0.5,
            survived: learner.is_some(),
        }
    }

    fn evaluate_on_seed(&self, genome: &Genome, seed: u64) -> f32 {
        self.run_episode_stats(genome, seed).fitness
    }
}

struct Evaluation {
    report: TrainerReport,
    scores: Vec<f32>,
}

struct EpisodeStats {
    mass: f32,
    fitness: f32,
    survived: bool,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_generation_moves_to_the_next_generation() {
        let mut trainer = Trainer::with_config(SimConfig {
            seed: 3,
            motes: 4,
            pellets: 8,
            episode_ticks: 20,
            ..SimConfig::default()
        });

        let report = trainer.run_generation();

        assert_eq!(report.generation, 0);
        assert_eq!(trainer.generation(), 1);
        assert_eq!(trainer.population().len(), 4);
    }

    #[test]
    fn genome_evaluation_is_deterministic() {
        let trainer = Trainer::with_config(SimConfig {
            seed: 4,
            motes: 2,
            pellets: 8,
            episode_ticks: 20,
            ..SimConfig::default()
        });
        let genome = trainer.population()[0].clone();

        assert_eq!(
            trainer.evaluate_genome(&genome, &[11, 12]),
            trainer.evaluate_genome(&genome, &[11, 12])
        );
    }
}
