use osmos_core::brain::Genome;
use osmos_evo::{Engine, Trainer};
use osmos_sim::SimConfig;

fn main() {
    let mut seed = 42_u64;
    let mut generations = 1_u32;
    let mut motes = 64_u32;
    let mut pellets = 236_u32;
    let mut episode = 2400_u64;
    let mut train = false;
    let mut holdout = 0_usize;

    let mut arguments = std::env::args().skip(1);
    while let Some(argument) = arguments.next() {
        match argument.as_str() {
            "--seed" => seed = parse(&mut arguments, "seed"),
            "--generations" => generations = parse(&mut arguments, "generations"),
            "--motes" => motes = parse(&mut arguments, "motes"),
            "--pellets" => pellets = parse(&mut arguments, "pellets"),
            "--episode" => episode = parse(&mut arguments, "episode"),
            "--train" => train = true,
            "--holdout" => holdout = parse(&mut arguments, "holdout"),
            other => panic!("unknown argument: {other}"),
        }
    }

    let config = SimConfig {
        seed,
        motes,
        pellets,
        episode_ticks: episode,
        ..SimConfig::default()
    };
    if train {
        let mut trainer = Trainer::with_config(config);
        for _ in 0..generations {
            let report = trainer.run_generation();
            println!(
                "{{\"generation\":{},\"best_mass\":{:.4},\"median_mass\":{:.4},\"mean_mass\":{:.4},\"best_score\":{:.4},\"mean_score\":{:.4},\"target_rate\":{:.4}}}",
                report.generation,
                report.best_mass,
                report.median_mass,
                report.mean_mass,
                report.best_score,
                report.mean_score,
                report.target_rate
            );
        }

        if holdout > 0 {
            let seeds = (1000..1000 + holdout as u64).collect::<Vec<_>>();
            let baseline_trainer = Trainer::with_config(config);
            let baseline_masses =
                masses_for(&baseline_trainer, baseline_trainer.population(), &seeds);
            let evolved_masses = masses_for(&trainer, trainer.population(), &seeds);
            let baseline_median = median(&baseline_masses);
            let evolved_median = median(&evolved_masses);
            println!(
                "{{\"phase\":\"holdout\",\"seed\":{},\"holdout\":{},\"baseline_median\":{:.4},\"evolved_median\":{:.4},\"ratio\":{:.4}}}",
                seed,
                holdout,
                baseline_median,
                evolved_median,
                evolved_median / baseline_median.max(1e-6)
            );
        }
        return;
    }

    let mut engine = Engine::with_config(config);

    for _ in 0..generations {
        let report = engine.run_generation();
        println!(
            "{{\"generation\":{},\"best_mass\":{:.4},\"median_mass\":{:.4},\"mean_mass\":{:.4},\"mean_speed\":{:.4},\"ejected_mass\":{:.4},\"alive\":{},\"best_score\":{:.4}}}",
            report.generation,
            report.best_mass,
            report.median_mass,
            report.mean_mass,
            report.mean_speed,
            report.ejected_mass,
            report.alive,
            report.best_score
        );
    }
}

fn parse<T: std::str::FromStr>(arguments: &mut impl Iterator<Item = String>, name: &str) -> T {
    arguments
        .next()
        .unwrap_or_else(|| panic!("missing value for --{name}"))
        .parse()
        .unwrap_or_else(|_| panic!("invalid value for --{name}"))
}

fn masses_for(trainer: &Trainer, population: &[Genome], seeds: &[u64]) -> Vec<f32> {
    population
        .iter()
        .map(|genome| trainer.evaluate_genome_mass(genome, seeds))
        .collect()
}

fn median(values: &[f32]) -> f32 {
    if values.is_empty() {
        return 0.0;
    }
    let mut sorted = values.to_vec();
    sorted.sort_by(f32::total_cmp);
    sorted[sorted.len() / 2]
}
