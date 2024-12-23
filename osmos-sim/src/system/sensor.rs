pub fn process(object_list: &mut [crate::object::Object]) {
    for current_object_index in 0..object_list.len() {
        // get other object index list
        let other_object_index_list = object_list
            .iter()
            .enumerate()
            .map(|(index, _)| index)
            .filter(|&index| index != current_object_index)
            .collect::<Vec<usize>>();

        // get in sensor range other object index list
        let in_sensor_range_other_object_index_list = other_object_index_list
            .iter()
            .filter(|&&other_object_index| {
                let distance = nalgebra::distance(
                    &object_list[current_object_index].cell.position,
                    &object_list[other_object_index].cell.position,
                );
                distance <= object_list[current_object_index].cell.sensor.range
            })
            .copied()
            .collect::<Vec<usize>>();

        // set sensor_data_list by energy and position
        // [up, right, down, left]
        let mut sensor_data_list = [0.0; 4];
        in_sensor_range_other_object_index_list
            .iter()
            .for_each(|&other_object_index| {
                let current_object_energy = object_list[current_object_index].cell.energy;
                let other_object_energy = object_list[other_object_index].cell.energy;

                let status = (current_object_energy as f32 - other_object_energy as f32)
                    / current_object_energy as f32;

                let current_object_position = object_list[current_object_index].cell.position;
                let other_object_position = object_list[other_object_index].cell.position;

                // up
                if other_object_position.y <= current_object_position.y {
                    sensor_data_list[0] += status;
                }
                // right
                if other_object_position.x >= current_object_position.x {
                    sensor_data_list[1] += status;
                }
                // down
                if other_object_position.y >= current_object_position.y {
                    sensor_data_list[2] += status;
                }
                // left
                if other_object_position.x <= current_object_position.x {
                    sensor_data_list[3] += status;
                }
            });
        object_list[current_object_index].cell.sensor.data_list = sensor_data_list;
    }
}
