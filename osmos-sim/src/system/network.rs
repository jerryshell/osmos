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

    let position_x_array = object_list
        .iter()
        .map(|item| item.cell.position.x)
        .collect::<Vec<f64>>();
    let position_x_array_zscore = crate::statistics::array_zscore(&position_x_array);

    let position_y_array = object_list
        .iter()
        .map(|item| item.cell.position.y)
        .collect::<Vec<f64>>();
    let position_y_array_zscore = crate::statistics::array_zscore(&position_y_array);

    let sensor_up_array = object_list
        .iter()
        .map(|item| item.cell.sensor.data_list[0])
        .collect::<Vec<f64>>();
    let sensor_up_array_zscore = crate::statistics::array_zscore(&sensor_up_array);

    let sensor_right_array = object_list
        .iter()
        .map(|item| item.cell.sensor.data_list[1])
        .collect::<Vec<f64>>();
    let sensor_right_array_zscore = crate::statistics::array_zscore(&sensor_right_array);

    let sensor_down_array = object_list
        .iter()
        .map(|item| item.cell.sensor.data_list[2])
        .collect::<Vec<f64>>();
    let sensor_down_array_zscore = crate::statistics::array_zscore(&sensor_down_array);

    let sensor_left_array = object_list
        .iter()
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
            assert_eq!(
                nn_output.len(),
                object.network.layer_topology[object.network.layer_topology.len() - 1]
            );

            let direction_x = nn_output[0].sin();
            let direction_y = nn_output[1].sin();

            object.cell.direction = nalgebra::Vector2::new(direction_x, direction_y);
        });
}
