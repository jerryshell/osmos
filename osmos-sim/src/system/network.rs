pub fn process(object_list: &mut [crate::object::Object]) {
    for object in object_list {
        let nn_output = object.network.feed(&object.cell.sensor.data_list);

        let up = nn_output[0];
        let right = nn_output[1];
        let down = nn_output[2];
        let left = nn_output[3];

        let acc_x = right - left;
        let acc_y = down - up;
        object.cell.acceleration = nalgebra::Vector2::new(acc_x, acc_y);

        object.network_output = nn_output;
    }
}

// fn sigmoid(x: f32) -> f32 {
//     1.0 / (1.0 + std::f32::consts::E.powf(-x))
// }
