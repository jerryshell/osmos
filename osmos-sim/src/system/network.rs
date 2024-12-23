pub fn process(object_list: &mut [crate::object::Object]) {
    let sensor_up_array = object_list
        .iter()
        .map(|item| item.cell.sensor.data_list[0])
        .collect::<Vec<f32>>();
    let sensor_up_array_zscore = crate::statistics::array_zscore(&sensor_up_array);

    let sensor_right_array = object_list
        .iter()
        .map(|item| item.cell.sensor.data_list[1])
        .collect::<Vec<f32>>();
    let sensor_right_array_zscore = crate::statistics::array_zscore(&sensor_right_array);

    let sensor_down_array = object_list
        .iter()
        .map(|item| item.cell.sensor.data_list[2])
        .collect::<Vec<f32>>();
    let sensor_down_array_zscore = crate::statistics::array_zscore(&sensor_down_array);

    let sensor_left_array = object_list
        .iter()
        .map(|item| item.cell.sensor.data_list[3])
        .collect::<Vec<f32>>();
    let sensor_left_array_zscore = crate::statistics::array_zscore(&sensor_left_array);

    object_list
        .iter_mut()
        .enumerate()
        .for_each(|(index, object)| {
            let nn_input = [
                sensor_up_array_zscore[index],
                sensor_right_array_zscore[index],
                sensor_down_array_zscore[index],
                sensor_left_array_zscore[index],
            ];
            let nn_output = object.network.feed(&nn_input);
            let direction_x = nn_output[0].sin();
            let direction_y = nn_output[1].sin();
            object.cell.direction = nalgebra::Vector2::new(direction_x, direction_y);
        });
}
