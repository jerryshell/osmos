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
            if object_list[current_object_index].cell.energy
                == object_list[other_object_index].cell.energy
            {
                let current_object = &object_list[current_object_index];
                let other_object = &object_list[other_object_index];
                let mut current_object_velocity = nalgebra::Vector2::new(
                    current_object.cell.position.x - other_object.cell.position.x,
                    current_object.cell.position.y - other_object.cell.position.y,
                );
                current_object_velocity.set_magnitude(current_object.cell.velocity_max_magnitude);
                let mut other_object_velocity = nalgebra::Vector2::new(
                    other_object.cell.position.x - current_object.cell.position.x,
                    other_object.cell.position.y - current_object.cell.position.y,
                );
                other_object_velocity.set_magnitude(other_object.cell.velocity_max_magnitude);
                object_list[current_object_index].cell.velocity = current_object_velocity;
                object_list[other_object_index].cell.velocity = other_object_velocity;
            }
        }
    }
    object_list.retain(|object| object.cell.energy > 0);
}
