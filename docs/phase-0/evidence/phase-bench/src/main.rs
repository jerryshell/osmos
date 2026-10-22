use std::time::{Duration, Instant};

fn main() {
    let steps: usize = std::env::args()
        .nth(1)
        .and_then(|value| value.parse().ok())
        .unwrap_or(5_000);
    let objects: usize = std::env::args()
        .nth(2)
        .and_then(|value| value.parse().ok())
        .unwrap_or(300);

    let mut simulator = osmos_sim::simulator::Simulator::default();
    while simulator.object_list.len() < objects {
        let id = simulator.object_list.len();
        let object = osmos_sim::object::Object::new(&mut simulator.rng, id);
        simulator.object_list.push(object);
    }
    simulator.object_count = objects;

    let mut sensor = Duration::ZERO;
    let mut network = Duration::ZERO;
    let mut collision = Duration::ZERO;
    let mut movement = Duration::ZERO;
    let mut epoch = Duration::ZERO;

    for _ in 0..steps {
        let start = Instant::now();
        osmos_sim::system::sensor::process(&mut simulator.object_list);
        sensor += start.elapsed();

        let start = Instant::now();
        osmos_sim::system::network::process(&mut simulator.object_list);
        network += start.elapsed();

        let start = Instant::now();
        osmos_sim::system::collision::process(&mut simulator.object_list);
        collision += start.elapsed();

        let start = Instant::now();
        osmos_sim::system::movement::process(&mut simulator.rng, &mut simulator.object_list);
        movement += start.elapsed();

        let start = Instant::now();
        osmos_sim::system::epoch::process(&mut simulator);
        epoch += start.elapsed();
    }

    let total = sensor + network + collision + movement + epoch;
    let pct = |value: Duration| value.as_secs_f64() / total.as_secs_f64() * 100.0;
    println!(
        "steps={} objects={} total_s={:.3} steps_per_s={:.1} sensor={:.1}% network={:.1}% collision={:.1}% movement={:.1}% epoch={:.1}% population={} generations={}",
        steps,
        objects,
        total.as_secs_f64(),
        steps as f64 / total.as_secs_f64(),
        pct(sensor),
        pct(network),
        pct(collision),
        pct(movement),
        pct(epoch),
        simulator.object_list.len(),
        simulator.epoch_count
    );
}
