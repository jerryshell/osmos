use std::time::Instant;

fn main() {
    let steps: usize = std::env::args()
        .nth(1)
        .and_then(|value| value.parse().ok())
        .unwrap_or(10_000);

    let mut simulator = osmos_sim::simulator::Simulator::default();
    let start = Instant::now();

    for _ in 0..steps {
        simulator.step();
    }

    let elapsed = start.elapsed();
    let seconds = elapsed.as_secs_f64();
    println!(
        "steps={} elapsed_s={:.3} steps_per_s={:.1} population={} epoch={}",
        steps,
        seconds,
        steps as f64 / seconds,
        simulator.object_list.len(),
        simulator.epoch_count
    );
}
