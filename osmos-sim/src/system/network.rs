pub fn process(object_list: &mut [crate::object::Object]) {
    object_list.iter_mut().for_each(|object| {
        let mut nn_input = vec![
            object.cell.energy as f64,
            object.cell.velocity.x,
            object.cell.velocity.y,
        ];
        nn_input.extend(object.cell.sensor.data_list);
        assert_eq!(nn_input.len(), object.network.layer_topology[0]);

        let nn_output = object.network.feed(&nn_input);
        assert_eq!(nn_output.len(), 4);

        let up = nn_output[0];
        let right = nn_output[1];
        let down = nn_output[2];
        let left = nn_output[3];

        let acc_x = right - left;
        let acc_y = down - up;
        object.cell.acceleration = nalgebra::Vector2::new(acc_x, acc_y)
            .cap_magnitude(object.cell.get_max_velocity_magnitude() / 2.0);
    });
}

// fn sigmoid(x: f64) -> f64 {
//     1.0 / (1.0 + std::f64::consts::E.powf(-x))
// }
