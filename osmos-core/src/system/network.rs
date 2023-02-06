pub fn process(cell_list: &mut [crate::cell::Cell]) {
    for cell in cell_list {
        let mut nn_output = cell.network.feed(&cell.sensor_data_list);
        // 0.0~1.0 => -0.5~0.5
        nn_output = nn_output.iter().map(|n| sigmoid(*n) - 0.5).collect();
        cell.network_output = nn_output.clone();
        // -0.5~0.5 => -0.0005~0.0005
        cell.acceleration = nalgebra::Vector2::new(nn_output[0] / 1000.0, nn_output[1] / 1000.0);
    }
}

fn sigmoid(x: f32) -> f32 {
    1.0 / (1.0 + std::f32::consts::E.powf(-x))
}
