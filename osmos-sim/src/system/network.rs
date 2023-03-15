use rayon::prelude::*;

pub fn process(object_list: &mut [crate::object::Object]) {
    let energy_array = object_list
        .par_iter()
        .map(|item| item.cell.energy as f64)
        .collect::<Vec<f64>>();
    let energy_array_zscore = crate::statistics::array_zscore(&energy_array);

    let velocity_x_array = object_list
        .par_iter()
        .map(|item| item.cell.position.x)
        .collect::<Vec<f64>>();
    let velocity_x_array_zscore = crate::statistics::array_zscore(&velocity_x_array);

    let velocity_y_array = object_list
        .par_iter()
        .map(|item| item.cell.position.y)
        .collect::<Vec<f64>>();
    let velocity_y_array_zscore = crate::statistics::array_zscore(&velocity_y_array);

    let position_x_array = object_list
        .par_iter()
        .map(|item| item.cell.position.x)
        .collect::<Vec<f64>>();
    let position_x_array_zscore = crate::statistics::array_zscore(&position_x_array);

    let position_y_array = object_list
        .par_iter()
        .map(|item| item.cell.position.y)
        .collect::<Vec<f64>>();
    let position_y_array_zscore = crate::statistics::array_zscore(&position_y_array);

    let sensor_up_array = object_list
        .par_iter()
        .map(|item| item.cell.sensor.data_list[0])
        .collect::<Vec<f64>>();
    let sensor_up_array_zscore = crate::statistics::array_zscore(&sensor_up_array);

    let sensor_right_array = object_list
        .par_iter()
        .map(|item| item.cell.sensor.data_list[1])
        .collect::<Vec<f64>>();
    let sensor_right_array_zscore = crate::statistics::array_zscore(&sensor_right_array);

    let sensor_down_array = object_list
        .par_iter()
        .map(|item| item.cell.sensor.data_list[2])
        .collect::<Vec<f64>>();
    let sensor_down_array_zscore = crate::statistics::array_zscore(&sensor_down_array);

    let sensor_left_array = object_list
        .par_iter()
        .map(|item| item.cell.sensor.data_list[3])
        .collect::<Vec<f64>>();
    let sensor_left_array_zscore = crate::statistics::array_zscore(&sensor_left_array);

    object_list
        .iter_mut()
        .enumerate()
        .for_each(|(index, object)| {
            let nn_input = [
                energy_array_zscore[index],
                velocity_x_array_zscore[index],
                velocity_y_array_zscore[index],
                position_x_array_zscore[index],
                position_y_array_zscore[index],
                sensor_up_array_zscore[index],
                sensor_right_array_zscore[index],
                sensor_down_array_zscore[index],
                sensor_left_array_zscore[index],
            ];
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
            object.cell.acceleration =
                nalgebra::Vector2::new(acc_x, acc_y).cap_magnitude(cell_max_velocity_magnitude);
        });
}

// fn sigmoid(x: f64) -> f64 {
//     1.0 / (1.0 + std::f64::consts::E.powf(-x))
// }
