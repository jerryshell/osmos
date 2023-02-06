pub fn process(cell_list: &mut [crate::cell::Cell]) {
    for cell in cell_list {
        let nn_output = cell.network.feed(&cell.sensor_data_list);
        cell.network_output = nn_output.clone();
        let up = nn_output[0];
        let right = nn_output[1];
        let down = nn_output[2];
        let left = nn_output[3];
        cell.acceleration = nalgebra::Vector2::new((right - left) / 1000.0, (down - up) / 1000.0);
    }
}

// fn sigmoid(x: f32) -> f32 {
//     1.0 / (1.0 + std::f32::consts::E.powf(-x))
// }
