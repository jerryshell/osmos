pub struct Network {
    pub layer_list: Vec<crate::layer::Layer>,
}

impl Network {
    pub fn new(layer_list: Vec<crate::layer::Layer>) -> Self {
        Self { layer_list }
    }

    pub fn random(rng: &mut impl rand::RngCore, layer_topology: &[usize]) -> Self {
        assert!(layer_topology.len() > 1);
        let layer_list = layer_topology
            .windows(2)
            .map(|window| crate::layer::Layer::random(rng, window[0], window[1]))
            .collect();
        Self { layer_list }
    }

    pub fn feed(&self, input_list: &[f32]) -> Vec<f32> {
        self.layer_list
            .iter()
            .fold(input_list.to_vec(), |input_list, x| x.feed(&input_list))
    }
}
