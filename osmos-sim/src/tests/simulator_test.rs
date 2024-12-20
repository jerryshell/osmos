#[test]
fn test() {
    let mut sim = crate::simulator::Simulator::new(1.0, 1.0);
    for _ in 0..5 {
        sim.step();
        crateevolve::evolve(&mut sim);
    }
}
