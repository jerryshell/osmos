pub fn crossover(
    rng: &mut rand::rngs::ThreadRng,
    parent_a_gene_list: &crate::ga::gene::GeneList,
    parent_b_gene_list: &crate::ga::gene::GeneList,
) -> crate::ga::gene::GeneList {
    parent_a_gene_list
        .iter()
        .zip(parent_b_gene_list)
        .map(|(a, b)| {
            if rand::Rng::gen_bool(rng, 0.5) {
                *a
            } else {
                *b
            }
        })
        .collect()
}
