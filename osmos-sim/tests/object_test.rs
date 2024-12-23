#[test]
fn get_gene_from_network_test() {
    let mut rng = rand::thread_rng();
    let network = osmos_nn::network::Network::random(&mut rng, &[4, 6, 2]);
    let gene = osmos_sim::object::get_gene_from_network(&network);
    assert_eq!(gene.len(), (4 * 6 + 6) + (6 * 2 + 2));
}

#[test]
fn build_network_from_gene_test() {
    let mut rng = rand::thread_rng();
    let network = osmos_nn::network::Network::random(&mut rng, &[4, 6, 2]);
    let gene = osmos_sim::object::get_gene_from_network(&network);

    let network_2 = osmos_sim::object::build_network(&[4, 6, 2], &gene);

    assert_eq!(gene, osmos_sim::object::get_gene_from_network(&network_2));
}
