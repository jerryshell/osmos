pub struct Network {
    pub layer_list: Vec<crate::layer::Layer>,
}

impl Network {
    pub fn new(layer_list: Vec<crate::layer::Layer>) -> Self {
        Self { layer_list }
    }

    pub fn random(rng: &mut rand::rngs::ThreadRng, layer_topology: &[usize]) -> Self {
        let layer_list = layer_topology
            .windows(2)
            .map(|window| crate::layer::Layer::random(rng, window[0], window[1]))
            .collect();
        Self { layer_list }
    }

    pub fn feed(&self, input_list: Vec<f32>) -> Vec<f32> {
        let mut output_list = self.layer_list[0].feed(&input_list);
        self.layer_list
            .iter()
            .skip(1)
            .for_each(|layer| output_list = layer.feed(&output_list));
        output_list
    }
}

#[cfg(test)]
mod tests {
    mod random {
        #[test]
        fn test() {
            let mut rng = rand::thread_rng();
            let layer_topology = vec![4, 8, 6];
            let network = crate::network::Network::random(&mut rng, &layer_topology);
            assert!(network.layer_list[0].neuron_list.len() == 8);
            assert!(network.layer_list[0].neuron_list[0].weight_list.len() == 4);
            let result = layer_topology.windows(2).zip(network.layer_list).all(
                |(layer_topology_window, layer)| {
                    layer_topology_window[1] == layer.neuron_list.len()
                        && layer_topology_window[0] == layer.neuron_list[0].weight_list.len()
                },
            );
            assert!(result)
        }
    }

    mod feed {
        #[test]
        fn test() {
            let layer_1 = crate::layer::Layer::new(vec![
                crate::neuron::Neuron::new(1.0, vec![2.0, 3.0, 4.0]),
                crate::neuron::Neuron::new(1.0, vec![2.0, 3.0, 4.0]),
            ]);
            let layer_2 = crate::layer::Layer::new(vec![
                crate::neuron::Neuron::new(1.0, vec![2.0, 3.0]),
                crate::neuron::Neuron::new(1.0, vec![2.0, 3.0]),
            ]);
            let layer_list = vec![layer_1, layer_2];
            let network = crate::network::Network::new(layer_list);
            let input_list = vec![2.0, 2.0, 2.0];
            let output_list = network.feed(input_list);
            assert_eq!(output_list, vec![96.0, 96.0]);
        }
    }
}
