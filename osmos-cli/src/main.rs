use osmos_sim::simulator;

fn main() {
    let mut sim = simulator::Simulator::default();
    let mut max_step = 0;
    loop {
        sim.step();
        max_step = max_step.max(sim.step_count);
        println!(
            "{}\t{}\t{:?}\t{:?}",
            max_step,
            sim.epoch_count,
            sim.object_list[0].cell.sensor.data_list,
            sim.object_list[0].cell.direction
        );
    }
}
