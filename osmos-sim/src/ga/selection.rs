// randomly return an Object index
// the greater the fitness of Object, the greater the probability of being selected
pub fn selection(
    rng: &mut rand::rngs::ThreadRng,
    object_list: &[impl crate::ga::gene::GeneObject],
) -> usize {
    let total_fitness = object_list.iter().map(|o| o.fitness()).sum::<isize>();
    loop {
        let random_index = rand::Rng::gen_range(rng, 0..object_list.len());
        let fitness = object_list[random_index].fitness();
        let probability = fitness as f64 / total_fitness as f64;
        if probability <= 0.0 {
            continue;
        }
        let win = rand::Rng::gen_bool(rng, probability);
        if win {
            return random_index;
        }
    }
}
