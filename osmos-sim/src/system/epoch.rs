pub fn process(simulator: &mut crate::simulator::Simulator) {
    simulator.step_count += 1;
    if is_epoch_end(simulator) {
        let new_object_list = osmos_ga::evolve::evolve(
            &mut simulator.rng,
            &simulator.object_list,
            simulator.object_count,
        );
        simulator.object_list = new_object_list;
        simulator.step_count = 0;
        simulator.epoch_count += 1;
    }
}

fn is_epoch_end(simulator: &crate::simulator::Simulator) -> bool {
    simulator.step_count >= simulator.max_step_count_per_epoch
        || simulator.object_list.len() <= simulator.min_object_count_per_epoch
}
