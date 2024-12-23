use osmos_sim::simulator;

fn main() {
    let mut sim = simulator::Simulator::default();
    for _ in 0..1000 {
        sim.step();
        println!("{:?}", sim.object_list[0].cell.sensor.data_list);
    }
}
