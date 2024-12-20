// 9 = cell.energy(1) + cell.velocity.x(1) + cell.velocity.y(1) + cell.position.x(1) + cell.position.y(1) + cell.sensor.data_list(4)
// 2 = [direction_x, direction_y]
const NETWORK_LAYER_TOPOLOGY: [usize; 3] = [9, 16, 2];

pub struct Object {
    pub id: usize,
    pub cell: osmos_core::cell::Cell,
    pub network: osmos_nn::network::Network,
}

impl Object {
    pub fn new(rng: &mut rand::rngs::ThreadRng, id: usize, max_x: f64, max_y: f64) -> Self {
        Self {
            id,
            cell: osmos_core::cell::Cell::random(rng, max_x, max_y),
            network: osmos_nn::network::Network::random(rng, &NETWORK_LAYER_TOPOLOGY),
        }
    }

    pub fn from_network(
        rng: &mut rand::rngs::ThreadRng,
        network: osmos_nn::network::Network,
        id: usize,
        max_x: f64,
        max_y: f64,
    ) -> Self {
        Self {
            id,
            cell: osmos_core::cell::Cell::random(rng, max_x, max_y),
            network,
        }
    }
}

impl osmos_ga::gene::GeneObject for Object {
    fn gene(&self) -> osmos_ga::gene::Gene {
        get_gene_from_network(&self.network)
    }

    fn fitness(&self) -> isize {
        self.cell.energy as isize
    }

    fn build(
        rng: &mut rand::rngs::ThreadRng,
        gene: osmos_ga::gene::Gene,
        id: usize,
        max_x: f64,
        max_y: f64,
    ) -> Self {
        let network = build_network(&NETWORK_LAYER_TOPOLOGY, &gene);
        Self::from_network(rng, network, id, max_x, max_y)
    }
}

fn get_gene_from_network(network: &osmos_nn::network::Network) -> osmos_ga::gene::Gene {
    network
        .layer_list
        .iter()
        .flat_map(get_gene_from_layer)
        .collect()
}

fn get_gene_from_layer(layer: &osmos_nn::layer::Layer) -> osmos_ga::gene::Gene {
    layer
        .neuron_list
        .iter()
        .flat_map(get_gene_from_neuron)
        .collect()
}

fn get_gene_from_neuron(neuron: &osmos_nn::neuron::Neuron) -> osmos_ga::gene::Gene {
    let mut gene = Vec::with_capacity(neuron.weight_list.len() + 1);
    gene.push(neuron.bias);
    gene.append(&mut neuron.weight_list.clone());
    gene
}

pub fn build_network(layer_topology: &[usize], gene: &[f64]) -> osmos_nn::network::Network {
    let mut gene_data_iter = gene.iter().copied();
    let layer_list = layer_topology
        .windows(2)
        .map(|window| build_layer(window[0], window[1], &mut gene_data_iter))
        .collect();
    osmos_nn::network::Network::new(layer_list)
}

fn build_layer(
    weight_list_len_per_neuron: usize,
    neuron_count: usize,
    gene_data_iter: &mut impl Iterator<Item = f64>,
) -> osmos_nn::layer::Layer {
    let neuron_list = (0..neuron_count)
        .map(|_| build_neuron(weight_list_len_per_neuron, gene_data_iter))
        .collect();
    osmos_nn::layer::Layer::new(neuron_list)
}

fn build_neuron(
    weight_list_len: usize,
    gene_data_iter: &mut impl Iterator<Item = f64>,
) -> osmos_nn::neuron::Neuron {
    let bias = gene_data_iter
        .next()
        .expect("build neuron from gene_data_iter failed");
    let weight_list = (0..weight_list_len)
        .map(|_| {
            gene_data_iter
                .next()
                .expect("build neuron from gene_data_iter failed")
        })
        .collect::<osmos_ga::gene::Gene>();
    osmos_nn::neuron::Neuron::new(bias, &weight_list)
}
