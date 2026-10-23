use osmos_core::brain::Genome;
use osmos_evo::{EcologyTrainer, Engine, Scenario};
use osmos_sim::SimConfig;

fn main() {
    let mut seed = 42_u64;
    let mut generations = 1_u32;
    let mut motes = 64_u32;
    let mut pellets = 236_u32;
    let mut episode = 2400_u64;
    let mut pellet_reserve = SimConfig::default().pellet_reserve;
    let mut scenario = 0_u32;
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
            "--pellet-reserve" => pellet_reserve = parse(&mut arguments, "pellet reserve"),
            "--scenario" => scenario = parse(&mut arguments, "scenario"),
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
        pellet_reserve,
        ..SimConfig::default()
    };
    if ecology {
        let mut trainer = EcologyTrainer::with_config(config);
        for _ in 0..generations {
            let report = trainer.run_generation();
            println!(
                "{{\"generation\":{},\"best_mass\":{:.4},\"mean_mass\":{:.4},\"survivors\":{},\"predation\":{},\"pellet_intake\":{},\"flee_events\":{},\"hunt_events\":{},\"behavior_samples\":{}}}",
                report.generation,
                report.best_mass,
                report.mean_mass,
                report.survivors,
                report.predation,
                report.pellet_intake,
                report.flee_events,
                report.hunt_events,
                report.behavior_samples
            );
        }
        return;
    }

    let mut engine = Engine::with_scenario_config(seed, Scenario::from_id(scenario), config);

    for _ in 0..generations {
        let report = engine.run_generation();
        println!(
            "{{\"generation\":{},\"best_mass\":{:.4},\"median_mass\":{:.4},\"mean_mass\":{:.4},\"mean_speed\":{:.4},\"ejected_mass\":{:.4},\"alive\":{},\"pellet_intake\":{},\"best_score\":{:.4}}}",
            report.generation,
            report.best_mass,
            report.median_mass,
            report.mean_mass,
            report.mean_speed,
            report.ejected_mass,
            report.alive,
            report.pellet_intake,
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
