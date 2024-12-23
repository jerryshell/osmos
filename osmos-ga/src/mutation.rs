pub fn mutation(
    rng: &mut impl rand::RngCore,
    mutate_chance: f64,
    mutate_coeff: f32,
    gene: &mut crate::gene::Gene,
) {
    gene.iter_mut().for_each(|gene| {
        let mutate_flag = rand::Rng::gen_bool(rng, mutate_chance);
        if mutate_flag {
            let sign = if rand::Rng::gen_bool(rng, 0.5) {
                -1.0
            } else {
                1.0
            };
            *gene += sign * mutate_coeff * rand::Rng::gen::<f32>(rng);
        }
    });
}
