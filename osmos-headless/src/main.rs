use osmos_core::brain::Genome;
use osmos_evo::{EcologyTrainer, Engine, Scenario, Trainer};
use osmos_sim::SimConfig;

fn main() {
    let mut seed = 42_u64;
    let mut generations = 1_u32;
    let mut motes = 64_u32;
    let mut pellets = 236_u32;
    let mut episode = 2400_u64;
    let mut train = false;
    let mut holdout = 0_usize;
    let mut scenario = 0_u32;
    let mut probe = false;
    let mut ecology = false;
    let mut export_champion: Option<String> = None;

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
            "--scenario" => scenario = parse(&mut arguments, "scenario"),
            "--probe" => probe = true,
            "--ecology" => ecology = true,
            "--export-champion" => {
                export_champion = Some(next_value(&mut arguments, "champion path"))
            }
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
    if ecology {
        let mut trainer = EcologyTrainer::with_config(config);
        for _ in 0..generations {
            let report = trainer.run_generation();
            println!(
                "{{\"generation\":{},\"best_mass\":{:.4},\"mean_mass\":{:.4},\"survivors\":{},\"predation\":{},\"flee_events\":{},\"hunt_events\":{},\"behavior_samples\":{}}}",
                report.generation,
                report.best_mass,
                report.mean_mass,
                report.survivors,
                report.predation,
                report.flee_events,
                report.hunt_events,
                report.behavior_samples
            );
        }
        return;
    }

    if train {
        let mut trainer = Trainer::with_config(config);
        for _ in 0..generations {
            let report = trainer.run_generation();
            println!(
                "{{\"generation\":{},\"best_mass\":{:.4},\"median_mass\":{:.4},\"mean_mass\":{:.4},\"best_score\":{:.4},\"mean_score\":{:.4},\"target_rate\":{:.4},\"survival_rate\":{:.4}}}",
                report.generation,
                report.best_mass,
                report.median_mass,
                report.mean_mass,
                report.best_score,
                report.mean_score,
                report.target_rate,
                report.survival_rate
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
        if probe {
            let report = trainer.probe_best(9001);
            println!(
                "{{\"phase\":\"probe\",\"seed\":{},\"predator_mass\":{:.4},\"predator_absorbed_motes\":{},\"predator_absorbed_pellets\":{},\"prey_survived\":{},\"prey_alive_ticks\":{},\"prey_mass\":{:.4},\"prey_min_hunter_distance\":{:.4},\"prey_mean_hunter_distance\":{:.4}}}",
                report.seed,
                report.predator_mass,
                report.predator_absorbed_motes,
                report.predator_absorbed_pellets,
                report.prey_survived,
                report.prey_alive_ticks,
                report.prey_mass,
                report.prey_min_hunter_distance,
                report.prey_mean_hunter_distance
            );
        }
        return;
    }

    let mut engine = Engine::with_scenario_config(seed, Scenario::from_id(scenario), config);

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

    if let Some(path) = export_champion {
        write_champion(&path, seed, engine.generation(), engine.best_genome());
    }
}

fn next_value(arguments: &mut impl Iterator<Item = String>, name: &str) -> String {
    arguments
        .next()
        .unwrap_or_else(|| panic!("missing value for {name}"))
}

fn parse<T: std::str::FromStr>(arguments: &mut impl Iterator<Item = String>, name: &str) -> T {
    arguments
        .next()
        .unwrap_or_else(|| panic!("missing value for --{name}"))
        .parse()
        .unwrap_or_else(|_| panic!("invalid value for --{name}"))
}

fn write_champion(path: &str, seed: u64, generation: u32, genome: &Genome) {
    let mut text = format!("{{\"seed\":{seed},\"generation\":{generation},\"genome\":[");
    for (index, value) in genome.as_slice().iter().enumerate() {
        if index > 0 {
            text.push(',');
        }
        text.push_str(&format!("{value:.8}"));
    }
    text.push_str("]}");
    std::fs::write(path, text).unwrap_or_else(|error| panic!("write champion: {error}"));
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
