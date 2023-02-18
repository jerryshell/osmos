pub fn evolve(simulator: &mut crate::simulator::Simulator) {
    let new_object_list = (0..simulator.object_count)
        .map(|_| {
            let parent_a_index =
                crate::ga::selection::selection(&mut simulator.rng, &simulator.object_list);
            let parent_b_index =
                crate::ga::selection::selection(&mut simulator.rng, &simulator.object_list);
            let parent_a = &simulator.object_list[parent_a_index];
            let parent_b = &simulator.object_list[parent_b_index];
            let parent_a_gene_list = parent_a.get_gene_list();
            let parent_b_gene_list = parent_b.get_gene_list();

            let mut child_gene_list = crate::ga::crossover::crossover(
                &mut simulator.rng,
                &parent_a_gene_list,
                &parent_b_gene_list,
            );

            crate::ga::mutation::mutation(&mut simulator.rng, 0.01, 0.3, &mut child_gene_list);

            let child_network = crate::ga::gene::build_network_from_gene_list(
                &parent_a.network.layer_topology,
                &child_gene_list,
            );

            crate::object::Object::from_network(&mut simulator.rng, child_network)
        })
        .collect();
    simulator.object_list = new_object_list;
}
