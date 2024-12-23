#[test]
fn test() {
    let mut sim = osmos_sim::simulator::Simulator::default();
    for _ in 0..5 {
        sim.step();
    }
}
