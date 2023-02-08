pub fn process(object_list: &mut Vec<crate::object::Object>) {
    for current_object_index in 0..object_list.len() {
        if object_list[current_object_index].cell.energy == 0 {
            continue;
        }
        for other_object_index in 0..object_list.len() {
            if current_object_index == other_object_index {
                continue;
            }
            if object_list[other_object_index].cell.energy == 0 {
                continue;
            }
            let distance = nalgebra::distance(
                &object_list[current_object_index].cell.position,
                &object_list[other_object_index].cell.position,
            );
            let energy_sum = object_list[current_object_index].cell.energy
                + object_list[other_object_index].cell.energy;
            if distance >= energy_sum as f64 / 1000.0 {
                continue;
            }
            if object_list[current_object_index].cell.energy
                > object_list[other_object_index].cell.energy
            {
                object_list[current_object_index].cell.energy += 1;
                object_list[other_object_index].cell.energy -= 1;
            }
            if object_list[current_object_index].cell.energy
                < object_list[other_object_index].cell.energy
            {
                object_list[current_object_index].cell.energy -= 1;
                object_list[other_object_index].cell.energy += 1;
            }
        }
    }
    object_list.retain(|object| object.cell.energy > 0);
}
