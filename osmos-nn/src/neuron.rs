pub struct Neuron {
    pub bias: f32,
    pub weight_list: Vec<f32>,
}

impl Neuron {
    pub fn new(bias: f32, weight_list: Vec<f32>) -> Self {
        Self { bias, weight_list }
    }

    pub fn random(rng: &mut rand::rngs::ThreadRng, weight_list_size: usize) -> Self {
        let bias = rand::Rng::gen_range(rng, -1.0..=1.0);
        let weight_list = (0..weight_list_size)
            .map(|_| rand::Rng::gen_range(rng, -1.0..=1.0))
            .collect::<Vec<f32>>();
        Self { bias, weight_list }
    }

    pub fn feed(&self, input_list: &[f32]) -> f32 {
        let sum = input_list
            .iter()
            .zip(&self.weight_list)
            .map(|(input, weight)| input * weight)
            .sum::<f32>();
        sum + self.bias
    }
}

#[cfg(test)]
mod tests {
    mod random {
        #[test]
        fn test() {
            let mut rng = rand::thread_rng();
            let neuron = crate::neuron::Neuron::random(&mut rng, 100);
            assert!(neuron.weight_list.len() == 100);
            assert!((-1.0..=1.0).contains(&neuron.bias));
            assert!(neuron
                .weight_list
                .iter()
                .all(|weight| (-1.0..=1.0).contains(weight)));
        }
    }

    mod feed {
        #[test]
        fn test() {
            let neuron = crate::neuron::Neuron::new(1.0, vec![2.0, 3.0, 4.0]);
            let input_list = vec![2.0, 2.0, 2.0];
            let output = neuron.feed(&input_list);
            assert!(output == 19.0);
        }
    }
}
