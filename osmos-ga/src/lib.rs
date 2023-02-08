pub trait Score {
    fn score(&self) -> isize;
}

pub fn selection<T: Score>(rng: &mut rand::rngs::ThreadRng, object_list: &[T]) -> usize {
    let total_fitness = object_list.iter().map(|o| o.score()).sum::<isize>();
    loop {
        let random_index = rand::Rng::gen_range(rng, 0..object_list.len());
        let fitness = object_list[random_index].score();
        let probability = fitness as f64 / total_fitness as f64;
        let win = rand::Rng::gen_bool(rng, probability);
        if win {
            return random_index;
        }
    }
}

pub trait Gene {
    fn get_gene_data_list(&self) -> Vec<f32>;
}

pub fn crossover<T: Gene>(rng: &mut rand::rngs::ThreadRng, parent_a: &T, parent_b: &T) -> Vec<f32> {
    let parent_a_gene_data_list = parent_a.get_gene_data_list();
    let parent_b_gene_data_list = parent_b.get_gene_data_list();
    let child_gene_data_list = parent_a_gene_data_list
        .iter()
        .zip(parent_b_gene_data_list)
        .map(|(a, b)| if rand::Rng::gen_bool(rng, 0.5) { *a } else { b })
        .collect::<Vec<f32>>();
    child_gene_data_list
}

pub fn mutation<T: Gene>(object: &T) -> Vec<f32> {
    todo!()
}

pub fn evole<T: Score + Gene>(object_list: &[T]) {
    todo!()
}
