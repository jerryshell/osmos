pub struct Object {
    pub cell: osmos_core::cell::Cell,
    pub network: osmos_nn::network::Network,
    pub network_output: Vec<f64>,
}

impl Object {
    pub fn new(rng: &mut rand::rngs::ThreadRng) -> Self {
        Self {
            cell: osmos_core::cell::Cell::random(rng),
            network: osmos_nn::network::Network::random(rng, &[4, 16, 4]),
            network_output: vec![],
        }
    }

    pub fn from_network(
        rng: &mut rand::rngs::ThreadRng,
        network: osmos_nn::network::Network,
    ) -> Self {
        Self {
            cell: osmos_core::cell::Cell::random(rng),
            network,
            network_output: vec![],
        }
    }
}
