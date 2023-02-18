pub struct Object {
    pub cell: osmos_core::cell::Cell,
    pub network: osmos_nn::network::Network,
}

impl Object {
    pub fn new(rng: &mut rand::rngs::ThreadRng) -> Self {
        let network_layer_topology = vec![6, 16, 4];
        Self {
            cell: osmos_core::cell::Cell::random(rng),
            network: osmos_nn::network::Network::random(rng, &network_layer_topology),
        }
    }

    pub fn from_network(
        rng: &mut rand::rngs::ThreadRng,
        network: osmos_nn::network::Network,
    ) -> Self {
        Self {
            cell: osmos_core::cell::Cell::random(rng),
            network,
        }
    }
}
