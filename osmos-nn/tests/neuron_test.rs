#[test]
fn random_test() {
    let mut rng = rand::thread_rng();
    let neuron = osmos_nn::neuron::Neuron::random(&mut rng, 100);
    assert!(neuron.weight_list.len() == 100);
    assert!((-1.0..=1.0).contains(&neuron.bias));
    assert!(neuron
        .weight_list
        .iter()
        .all(|weight| (-1.0..=1.0).contains(weight)));
}

#[test]
fn feed_test() {
    let neuron = osmos_nn::neuron::Neuron::new(1.0, &[2.0, 3.0, 4.0]);
    let input_list = [2.0, 2.0, 2.0];
    let output = neuron.feed(&input_list);
    assert!(output == 19.0);
}
