pub struct Object {
    pub cell: osmos_core::cell::Cell,
    pub network: osmos_nn::network::Network,
}

impl Object {
    pub fn new(rng: &mut rand::rngs::ThreadRng) -> Self {
        // 7 = cell.energy(1) + cell.velocity.x(1) + cell.velocity.y(1) + cell.sensor.data_list(4)
        // 4 = [up, right, down, left]
        let network_layer_topology = vec![7, 16, 4];
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
