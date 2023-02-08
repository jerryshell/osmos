pub type GeneList = Vec<f32>;

impl crate::object::Object {
    pub fn get_gene_list(&self) -> GeneList {
        get_gene_list_from_network(&self.network)
    }
}

fn get_gene_list_from_neuron(neuron: &osmos_nn::neuron::Neuron) -> GeneList {
    let mut gene_list = vec![neuron.bias];
    gene_list.append(&mut neuron.weight_list.clone());
    gene_list
}

fn build_neuron_from_gene_data_iter(
    weight_list_len: usize,
    gene_data_iter: &mut impl Iterator<Item = f32>,
) -> osmos_nn::neuron::Neuron {
    let bias = gene_data_iter.next().unwrap();
    let weight_list = (0..weight_list_len)
        .map(|_| gene_data_iter.next().unwrap())
        .collect::<GeneList>();
    osmos_nn::neuron::Neuron::new(bias, &weight_list)
}

fn get_gene_list_from_layer(layer: &osmos_nn::layer::Layer) -> GeneList {
    layer
        .neuron_list
        .iter()
        .flat_map(get_gene_list_from_neuron)
        .collect::<GeneList>()
}

fn build_layer_from_gene_data_iter(
    weight_list_len_per_neuron: usize,
    neuron_count: usize,
    gene_data_iter: &mut impl Iterator<Item = f32>,
) -> osmos_nn::layer::Layer {
    let neuron_list = (0..neuron_count)
        .map(|_| build_neuron_from_gene_data_iter(weight_list_len_per_neuron, gene_data_iter))
        .collect::<Vec<osmos_nn::neuron::Neuron>>();
    osmos_nn::layer::Layer::new(neuron_list)
}

fn get_gene_list_from_network(network: &osmos_nn::network::Network) -> GeneList {
    network
        .layer_list
        .iter()
        .flat_map(get_gene_list_from_layer)
        .collect::<GeneList>()
}

pub fn build_network_from_gene_list(
    layer_topology: &[usize],
    gene_list: &[f32],
) -> osmos_nn::network::Network {
    let mut gene_data_iter = gene_list.iter().copied();
    let layer_list = layer_topology
        .windows(2)
        .map(|window| build_layer_from_gene_data_iter(window[0], window[1], &mut gene_data_iter))
        .collect();
    osmos_nn::network::Network::new(layer_list)
}

#[cfg(test)]
mod tests {
    mod get_gene_list_from_network {
        #[test]
        fn test() {
            let mut rng = rand::thread_rng();
            let network = osmos_nn::network::Network::random(&mut rng, &[4, 6, 2]);
            let gene_list = crate::ga::gene::get_gene_list_from_network(&network);
            assert!(gene_list.len() == (4 * 6 + 6) + (6 * 2 + 2));
        }
    }

    mod build_network_from_gene_list {
        #[test]
        fn test() {
            let mut rng = rand::thread_rng();
            let network = osmos_nn::network::Network::random(&mut rng, &[4, 6, 2]);
            let gene_list = crate::ga::gene::get_gene_list_from_network(&network);

            let network_2 = crate::ga::gene::build_network_from_gene_list(&[4, 6, 2], &gene_list);

            assert_eq!(
                gene_list,
                crate::ga::gene::get_gene_list_from_network(&network_2)
            );
        }
    }
}
