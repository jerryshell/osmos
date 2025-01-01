fn main() {
    let mut v = [-100.0, 0.0, 2.0, 3.0];
    normalize(&mut v);
    println!("{:?}", v);

    let mut sim = osmos_sim::simulator::Simulator::default();
    let mut max_step = 0;
    loop {
        sim.step();
        max_step = max_step.max(sim.step_count);
        println!(
            "max_step:{} epoch_count:{} sensor:{:?} network_output:{:?} direction:{:?}",
            max_step,
            sim.epoch_count,
            sim.object_list[0].cell.sensor.data_list,
            sim.object_list[0].network_output,
            sim.object_list[0].cell.direction,
        );
    }
}

fn normalize(v: &mut [f32; 4]) {
    let magnitude = (v[0] * v[0] + v[1] * v[1] + v[2] * v[2] + v[3] * v[3]).sqrt();
    if magnitude != 0.0 {
        v[0] /= magnitude;
        v[1] /= magnitude;
        v[2] /= magnitude;
        v[3] /= magnitude;
    }
}
