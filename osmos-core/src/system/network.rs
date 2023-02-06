pub fn process(cell_list: &mut [crate::cell::Cell]) {
    for cell in cell_list {
        let nn_output = cell.network.feed(&cell.sensor_data_list);
        cell.network_output = nn_output.clone();
        let up = nn_output[0];
        let right = nn_output[1];
        let down = nn_output[2];
        let left = nn_output[3];
        let idle = nn_output[4];
        let max = nn_output
            .iter()
            .max_by(|x, y| x.partial_cmp(y).unwrap())
            .unwrap();
        if idle.partial_cmp(max).unwrap().is_eq() {
            cell.acceleration = nalgebra::Vector2::new(0.0, 0.0);
            return;
        }
        let left_or_right = if left.partial_cmp(&right).unwrap().is_ge() {
            -1.0
        } else {
            1.0
        };
        let up_or_down = if up.partial_cmp(&down).unwrap().is_ge() {
            -1.0
        } else {
            1.0
        };
        cell.acceleration = nalgebra::Vector2::new(left_or_right / 1000.0, up_or_down / 1000.0);
    }
}

// fn sigmoid(x: f32) -> f32 {
//     1.0 / (1.0 + std::f32::consts::E.powf(-x))
// }
