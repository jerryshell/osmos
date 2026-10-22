use osmos_core::brain::{Genome, GENOME_LENGTH};
use rand::rngs::StdRng;
use rand::RngExt;

pub const ELITES: usize = 4;
pub const TOURNAMENT_SIZE: usize = 3;
pub const MUTATION_CHANCE: f64 = 0.01;
pub const MUTATION_COEFFICIENT: f32 = 0.3;

pub fn random_population(rng: &mut StdRng, size: usize) -> Vec<Genome> {
    (0..size)
        .map(|_| {
            Genome::new(
                (0..GENOME_LENGTH)
                    .map(|_| rng.random_range(-0.3..0.3))
                    .collect(),
            )
        })
        .collect()
}

pub fn next_population(rng: &mut StdRng, population: &[Genome], scores: &[f32]) -> Vec<Genome> {
    let mut order = (0..population.len()).collect::<Vec<_>>();
    order.sort_by(|left, right| scores[*right].total_cmp(&scores[*left]));

    let mut next = Vec::with_capacity(population.len());
    for index in order.into_iter().take(ELITES.min(population.len())) {
        next.push(population[index].clone());
    }

    while next.len() < population.len() {
        let a = tournament(rng, scores);
        let b = tournament(rng, scores);
        let mut child = crossover(rng, &population[a], &population[b]);
        mutation(rng, &mut child);
        next.push(child);
    }

    next
}

fn tournament(rng: &mut StdRng, scores: &[f32]) -> usize {
    let mut best = rng.random_range(0..scores.len());
    for _ in 1..TOURNAMENT_SIZE {
        let candidate = rng.random_range(0..scores.len());
        if scores[candidate] > scores[best] {
            best = candidate;
        }
    }
    best
}

fn crossover(rng: &mut StdRng, left: &Genome, right: &Genome) -> Genome {
    let values = left
        .as_slice()
        .iter()
        .zip(right.as_slice())
        .map(|(a, b)| if rng.random_bool(0.5) { *a } else { *b })
        .collect();
    Genome::new(values)
}

fn mutation(rng: &mut StdRng, genome: &mut Genome) {
    for value in genome.as_mut_slice() {
        if rng.random_bool(MUTATION_CHANCE) {
            let sign = if rng.random_bool(0.5) { -1.0 } else { 1.0 };
            *value = (*value + sign * MUTATION_COEFFICIENT * rng.random::<f32>()).clamp(-3.0, 3.0);
        }
    }
}
