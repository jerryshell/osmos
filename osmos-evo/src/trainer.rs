use osmos_core::brain::{ControlNetwork, Genome};
use osmos_sim::{SimConfig, World};
use rand::rngs::StdRng;
use rand::SeedableRng;

use crate::fitness::{mean, median, score};
use crate::ga::{next_population, random_population};

const SEEDS_PER_GENOME: usize = 2;
#[derive(Clone, Debug, Default)]
pub struct TrainerReport {
    pub generation: u32,
    pub best_mass: f32,
    pub median_mass: f32,
    pub mean_mass: f32,
    pub best_score: f32,
    pub mean_score: f32,
    pub target_rate: f32,
}

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

        for (index, genome) in self.population.iter().enumerate() {
            let mut total_score = 0.0;
            let mut total_mass = 0.0;
            for replicate in 0..SEEDS_PER_GENOME {
                let seed = self.seed
                    ^ (self.generation as u64).wrapping_mul(0x9E37_79B9_7F4A_7C15)
                    ^ (index as u64).wrapping_mul(0x85EB_CA6B)
                    ^ (replicate as u64).wrapping_mul(0xC2B2_AE35);
                let mass = self.run_episode(genome, seed);
                total_mass += mass;
                total_score += score(mass, self.config.target_mass);
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
            },
            scores,
        }
    }

    fn run_episode(&self, genome: &Genome, seed: u64) -> f32 {
        let config = SimConfig {
            seed,
            motes: 1,
            episode_ticks: self.config.episode_ticks,
            ..self.config
        };
        let mut world = World::with_controllers(config, vec![ControlNetwork::from_genome(genome)]);
        for _ in 0..config.episode_ticks {
            world.step();
        }
        world
            .motes
            .first()
            .map(|mote| mote.mass.get())
            .unwrap_or(0.0)
    }

    fn evaluate_on_seed(&self, genome: &Genome, seed: u64) -> f32 {
        let mass = self.run_episode(genome, seed);
        score(mass, self.config.target_mass)
    }
}

struct Evaluation {
    report: TrainerReport,
    scores: Vec<f32>,
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
