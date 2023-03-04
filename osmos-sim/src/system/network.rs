pub fn process(object_list: &mut [crate::object::Object]) {
    let energy_array = object_list
        .iter()
        .map(|item| item.cell.energy as f64)
        .collect::<Vec<f64>>();
    let energy_array_zscore = crate::statistics::array_zscore(&energy_array);

    let velocity_x_array = object_list
        .iter()
        .map(|item| item.cell.position.x)
        .collect::<Vec<f64>>();
    let velocity_x_array_zscore = crate::statistics::array_zscore(&velocity_x_array);

    let velocity_y_array = object_list
        .iter()
        .map(|item| item.cell.position.y)
        .collect::<Vec<f64>>();
    let velocity_y_array_zscore = crate::statistics::array_zscore(&velocity_y_array);

    object_list
        .iter_mut()
        .enumerate()
        .for_each(|(index, object)| {
            let mut nn_input = vec![
                energy_array_zscore[index],
                velocity_x_array_zscore[index],
                velocity_y_array_zscore[index],
            ];
            nn_input.extend(object.cell.sensor.data_list);
            assert_eq!(nn_input.len(), object.network.layer_topology[0]);

            let nn_output = object.network.feed(&nn_input);
            assert_eq!(nn_output.len(), 4);

            let up = nn_output[0];
            let right = nn_output[1];
            let down = nn_output[2];
            let left = nn_output[3];

            let cell_max_velocity_magnitude = object.cell.get_max_velocity_magnitude();
            let acc_x = if left > right { -1.0 } else { 1.0 } * cell_max_velocity_magnitude;
            let acc_y = if up > down { -1.0 } else { 1.0 } * cell_max_velocity_magnitude;
            object.cell.acceleration = nalgebra::Vector2::new(acc_x, acc_y)
                .cap_magnitude(cell_max_velocity_magnitude / 2.0);
        });
}

// fn sigmoid(x: f64) -> f64 {
//     1.0 / (1.0 + std::f64::consts::E.powf(-x))
// }
