use std::f32::consts::TAU;

pub struct Neuron {
    pub bias: f32,
    pub weight_list: Vec<f32>,
}

impl Neuron {
    pub fn new(bias: f32, weight_list: &[f32]) -> Self {
        assert!(!weight_list.is_empty());
        Self {
            bias,
            weight_list: weight_list.to_vec(),
        }
    }

    pub fn random(rng: &mut impl rand::RngCore, weight_list_len: usize) -> Self {
        let bias = rand::Rng::gen_range(rng, 0.0..=TAU);
        let weight_list = (0..weight_list_len)
            .map(|_| rand::Rng::gen_range(rng, 0.0..=TAU))
            .collect();
        Self { bias, weight_list }
    }

    pub fn feed(&self, input_list: &[f32]) -> f32 {
        let sum = input_list
            .iter()
            .zip(&self.weight_list)
            .map(|(input, weight)| input * weight)
            .sum::<f32>();
        (sum + self.bias).max(0.0)
    }
}
