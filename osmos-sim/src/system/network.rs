pub fn process(object_list: &mut [crate::object::Object]) {
    for object in object_list {
        let nn_output = object.network.feed(&object.cell.sensor.data_list);
        object.network_output = nn_output.clone();
        let up = nn_output[0];
        let right = nn_output[1];
        let down = nn_output[2];
        let left = nn_output[3];
        object.cell.acceleration =
            nalgebra::Vector2::new((right - left) / 1000.0, (down - up) / 1000.0);
    }
}

// fn sigmoid(x: f32) -> f32 {
//     1.0 / (1.0 + std::f32::consts::E.powf(-x))
// }
