#[test]
fn get_gene_from_network_test() {
    let mut rng = rand::thread_rng();
    let network = osmos_nn::network::Network::random(&mut rng, &[4, 6, 2]);
    let gene = osmos_ga::gene::get_gene_from_network(&network);
    assert_eq!(gene.len(), (4 * 6 + 6) + (6 * 2 + 2));
}

#[test]
fn build_network_from_gene_test() {
    let mut rng = rand::thread_rng();
    let network = osmos_nn::network::Network::random(&mut rng, &[4, 6, 2]);
    let gene = osmos_ga::gene::get_gene_from_network(&network);

    let network_2 = osmos_ga::gene::build_network_from_gene(&[4, 6, 2], &gene);

    assert_eq!(
        gene,
        osmos_ga::gene::get_gene_from_network(&network_2)
    );
}
