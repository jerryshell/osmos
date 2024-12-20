pub struct Network {
    pub layer_topology: Vec<usize>,
    pub layer_list: Vec<crate::layer::Layer>,
}

impl Network {
    pub fn new(layer_list: Vec<crate::layer::Layer>) -> Self {
        let mut layer_topology = Vec::with_capacity(layer_list.len());
        layer_topology.push(layer_list[0].neuron_list[0].weight_list.len());
        layer_topology.append(
            &mut layer_list
                .iter()
                .map(|layer| layer.neuron_list.len())
                .collect::<Vec<usize>>(),
        );
        Self {
            layer_topology,
            layer_list,
        }
    }

    pub fn random(rng: &mut impl rand::RngCore, layer_topology: &[usize]) -> Self {
        let layer_list = layer_topology
            .windows(2)
            .map(|window| crate::layer::Layer::random(rng, window[0], window[1]))
            .collect();
        Self {
            layer_topology: layer_topology.to_vec(),
            layer_list,
        }
    }

    pub fn feed(&self, input_list: &[f64]) -> Vec<f64> {
        let mut output_list = self.layer_list[0].feed(input_list);
        self.layer_list
            .iter()
            .skip(1)
            .for_each(|layer| output_list = layer.feed(&output_list));
        output_list
    }
}
