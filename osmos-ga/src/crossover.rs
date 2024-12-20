pub fn crossover(
    rng: &mut impl rand::RngCore,
    parent_a_gene: &crate::gene::Gene,
    parent_b_gene: &crate::gene::Gene,
) -> crate::gene::Gene {
    parent_a_gene
        .iter()
        .zip(parent_b_gene)
        .map(|(&a, &b)| if rand::Rng::gen_bool(rng, 0.5) { a } else { b })
        .collect()
}
