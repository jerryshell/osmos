const MUTATE_CHANCE: f64 = 0.01;
const MUTATE_COEFF: f32 = 0.3;

pub fn evolve<T>(rng: &mut impl rand::RngCore, object_list: &[T], object_count: usize) -> Vec<T>
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

            // get parent gene
            let parent_a_gene = parent_a.gene();
            let parent_b_gene = parent_b.gene();

            // get child_gene by crossover
            let mut child_gene = crate::crossover::crossover(rng, &parent_a_gene, &parent_b_gene);

            // mutate child_gene
            crate::mutation::mutation(rng, MUTATE_CHANCE, MUTATE_COEFF, &mut child_gene);

            // build a new object
            T::build(rng, child_gene, id)
        })
        .collect::<Vec<T>>()
}
