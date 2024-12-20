const MUTATE_CHANCE: f64 = 0.01;
const MUTATE_COEFF: f64 = 0.3;

pub fn evolve<T>(
    rng: &mut rand::rngs::ThreadRng,
    object_list: &[T],
    object_count: usize,
    max_x: f64,
    max_y: f64,
) -> Vec<T>
where
    T: crate::gene::GeneObject,
{
    (0..object_count)
        .map(|id| {
            // select parent_a and parent_b
            let parent_a_index = crate::selection::selection(rng, object_list);
            let parent_b_index = crate::selection::selection(rng, object_list);

            let parent_a = &object_list[parent_a_index];
            let parent_b = &object_list[parent_b_index];

            // get parent gene_list
            let parent_a_gene_list = parent_a.get_gene_list();
            let parent_b_gene_list = parent_b.get_gene_list();

            // get child_gene_list by crossover
            let mut child_gene_list =
                crate::crossover::crossover(rng, &parent_a_gene_list, &parent_b_gene_list);

            // mutate child_gene_list
            crate::mutation::mutation(rng, MUTATE_CHANCE, MUTATE_COEFF, &mut child_gene_list);

            // build a new object
            T::build(rng, child_gene_list, id, max_x, max_y)
        })
        .collect::<Vec<T>>()
}
