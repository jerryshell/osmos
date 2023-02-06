#[derive(Clone)]
pub struct Layer {
    pub neuron_list: Vec<crate::neuron::Neuron>,
}

impl Layer {
    pub fn new(neuron_list: Vec<crate::neuron::Neuron>) -> Self {
        Self { neuron_list }
    }

    pub fn random(
        rng: &mut rand::rngs::ThreadRng,
        weight_list_size_per_neuron: usize,
        neuron_count: usize,
    ) -> Self {
        let neuron_list = (0..neuron_count)
            .map(|_| crate::neuron::Neuron::random(rng, weight_list_size_per_neuron))
            .collect::<Vec<crate::neuron::Neuron>>();
        Self { neuron_list }
    }

    pub fn feed(&self, input_list: &[f32]) -> Vec<f32> {
        self.neuron_list
            .iter()
            .map(|neuron| neuron.feed(input_list))
            // .map(sigmoid)
            .collect()
    }
}

fn relu(n: f32) -> f32 {
    n.max(0.0)
}

#[cfg(test)]
mod tests {
    mod random {
        #[test]
        fn test() {
            let mut rng = rand::thread_rng();
            let layer = crate::layer::Layer::random(&mut rng, 4, 10);
            assert!(layer.neuron_list.len() == 10);
            assert!(layer
                .neuron_list
                .iter()
                .all(|neuron| neuron.weight_list.len() == 4));
        }
    }

    mod feed {
        #[test]
        fn test() {
            let neuron_list = vec![
                crate::neuron::Neuron::new(1.0, vec![2.0, 3.0, 4.0]),
                crate::neuron::Neuron::new(1.0, vec![2.0, 3.0, 4.0]),
            ];
            let layer = crate::layer::Layer::new(neuron_list);
            let input_list = vec![2.0, 2.0, 2.0];
            let output_list = layer.feed(&input_list);
            assert_eq!(output_list, &[19.0, 19.0]);
        }
    }
}
