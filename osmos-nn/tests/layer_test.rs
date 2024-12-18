#[test]
fn random_test() {
    let mut rng = rand::thread_rng();
    let layer = osmos_nn::layer::Layer::random(&mut rng, 4, 10);
    assert!(layer.neuron_list.len() == 10);
    assert!(layer
        .neuron_list
        .iter()
        .all(|neuron| neuron.weight_list.len() == 4));
}

#[test]
fn feed_test() {
    let neuron_list = vec![
        osmos_nn::neuron::Neuron::new(1.0, &[2.0, 3.0, 4.0]),
        osmos_nn::neuron::Neuron::new(1.0, &[2.0, 3.0, 4.0]),
    ];
    let layer = osmos_nn::layer::Layer::new(neuron_list);
    let input_list = [2.0, 2.0, 2.0];
    let output_list = layer.feed(&input_list);
    assert_eq!(output_list, &[19.0, 19.0]);
}
