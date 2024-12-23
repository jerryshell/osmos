pub struct Layer {
    pub neuron_list: Vec<crate::neuron::Neuron>,
}

impl Layer {
    pub fn new(neuron_list: Vec<crate::neuron::Neuron>) -> Self {
        Self { neuron_list }
    }

    pub fn random(
        rng: &mut impl rand::RngCore,
        weight_list_len_per_neuron: usize,
        neuron_count: usize,
    ) -> Self {
        let neuron_list = (0..neuron_count)
            .map(|_| crate::neuron::Neuron::random(rng, weight_list_len_per_neuron))
            .collect();
        Self { neuron_list }
    }

    pub fn feed(&self, input_list: &[f32]) -> Vec<f32> {
        self.neuron_list
            .iter()
            .map(|neuron| neuron.feed(input_list))
            .collect()
    }
}
