mod random {
    #[test]
    fn test() {
        let mut rng = rand::thread_rng();
        let layer_topology = [4, 8, 2];
        let network = osmos_nn::network::Network::random(&mut rng, &layer_topology);
        assert_eq!(network.layer_list[0].neuron_list.len(), layer_topology[1]);
        assert_eq!(
            network.layer_list[0].neuron_list[0].weight_list.len(),
            layer_topology[0]
        );
        assert!(layer_topology.windows(2).zip(network.layer_list).all(
            |(layer_topology_window, layer)| {
                layer_topology_window[1] == layer.neuron_list.len()
                    && layer_topology_window[0] == layer.neuron_list[0].weight_list.len()
            },
        ));
    }
}

mod feed {
    #[test]
    fn test() {
        let layer_1 = osmos_nn::layer::Layer::new(vec![
            osmos_nn::neuron::Neuron::new(1.0, &[2.0, 3.0, 4.0]),
            osmos_nn::neuron::Neuron::new(1.0, &[2.0, 3.0, 4.0]),
        ]);
        let layer_2 = osmos_nn::layer::Layer::new(vec![
            osmos_nn::neuron::Neuron::new(1.0, &[2.0, 3.0]),
            osmos_nn::neuron::Neuron::new(1.0, &[2.0, 3.0]),
        ]);
        let layer_list = vec![layer_1, layer_2];
        let network = osmos_nn::network::Network::new(layer_list);
        let input_list = [2.0, 2.0, 2.0];
        let output_list = network.feed(&input_list);
        assert_eq!(output_list, [96.0, 96.0]);
    }
}
