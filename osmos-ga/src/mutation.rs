pub fn mutation(
    rng: &mut impl rand::Rng,
    mutate_chance: f64,
    mutate_coeff: f32,
    gene: &mut crate::gene::Gene,
) {
    gene.iter_mut().for_each(|gene| {
        let mutate_flag = rand::RngExt::random_bool(rng, mutate_chance);
        if mutate_flag {
            let sign = if rand::RngExt::random_bool(rng, 0.5) {
                -1.0
            } else {
                1.0
            };
            *gene += sign * mutate_coeff * rand::RngExt::random::<f32>(rng);
        }
    });
}
