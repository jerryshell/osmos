impl osmos_ga::Score for crate::object::Object {
    fn score(&self) -> isize {
        self.cell.energy as isize
    }
}

impl osmos_ga::Gene for crate::object::Object {
    fn get_gene_data_list(&self) -> Vec<f32> {
        get_gene_data_list_from_network(&self.network)
    }
}

fn get_gene_data_list_from_neuron(neuron: &osmos_nn::neuron::Neuron) -> Vec<f32> {
    let mut gene_data_list = vec![neuron.bias];
    gene_data_list.append(&mut neuron.weight_list.clone());
    gene_data_list
}

fn build_neuron_from_gene_data_iter(
    weight_list_len: usize,
    gene_data_iter: &mut impl Iterator<Item = f32>,
) -> osmos_nn::neuron::Neuron {
    let bias = gene_data_iter.next().unwrap();
    let weight_list = (0..weight_list_len)
        .map(|_| gene_data_iter.next().unwrap())
        .collect::<Vec<f32>>();
    osmos_nn::neuron::Neuron::new(bias, &weight_list)
}

fn get_gene_data_list_from_layer(layer: &osmos_nn::layer::Layer) -> Vec<f32> {
    layer
        .neuron_list
        .iter()
        .flat_map(get_gene_data_list_from_neuron)
        .collect::<Vec<f32>>()
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

fn get_gene_data_list_from_network(network: &osmos_nn::network::Network) -> Vec<f32> {
    network
        .layer_list
        .iter()
        .flat_map(get_gene_data_list_from_layer)
        .collect::<Vec<f32>>()
}

pub fn build_network_from_gene_data_list(
    layer_topology: &[usize],
    gene_data_list: &[f32],
) -> osmos_nn::network::Network {
    let mut gene_data_iter = gene_data_list.iter().copied();
    let layer_list = layer_topology
        .windows(2)
        .map(|window| build_layer_from_gene_data_iter(window[0], window[1], &mut gene_data_iter))
        .collect();
    osmos_nn::network::Network::new(layer_list)
}

#[cfg(test)]
mod tests {
    mod get_gene_data_list_from_network {
        #[test]
        fn test() {
            let mut rng = rand::thread_rng();
            let network = osmos_nn::network::Network::random(&mut rng, &[4, 6, 2]);
            let gene_data_list = crate::object_ga::get_gene_data_list_from_network(&network);
            assert!(gene_data_list.len() == (4 * 6 + 6) + (6 * 2 + 2));
        }
    }

    mod build_network_from_gene_data_list {
        #[test]
        fn test() {
            let mut rng = rand::thread_rng();
            let network = osmos_nn::network::Network::random(&mut rng, &[4, 6, 2]);
            let gene_data_list = crate::object_ga::get_gene_data_list_from_network(&network);

            let network_2 =
                crate::object_ga::build_network_from_gene_data_list(&[4, 6, 2], &gene_data_list);

            assert_eq!(
                gene_data_list,
                crate::object_ga::get_gene_data_list_from_network(&network_2)
            );
        }
    }
}
