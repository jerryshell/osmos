pub fn process(object_list: &mut Vec<crate::object::Object>) {
    for current_object_index in 0..object_list.len() {
        let current_object_energy = object_list[current_object_index].cell.energy;
        if current_object_energy == 0 {
            continue;
        }

        for other_object_index in 0..object_list.len() {
            if current_object_index == other_object_index {
                continue;
            }

            let other_object_energy = object_list[other_object_index].cell.energy;
            if other_object_energy == 0 {
                continue;
            }

            let energy_sum = current_object_energy + other_object_energy;
            let distance = nalgebra::distance(
                &object_list[current_object_index].cell.position,
                &object_list[other_object_index].cell.position,
            );
            if distance >= energy_sum as f64 / 1000.0 {
                continue;
            }

            match current_object_energy {
                _ if current_object_energy > other_object_energy => {
                    object_list[current_object_index].cell.energy += 1;
                    object_list[other_object_index].cell.energy -= 1;
                }
                _ if current_object_energy < other_object_energy => {
                    object_list[current_object_index].cell.energy -= 1;
                    object_list[other_object_index].cell.energy += 1;
                }
                _ => {
                    // energy equal
                    let current_object = &object_list[current_object_index];
                    let other_object = &object_list[other_object_index];

                    let mut current_object_velocity = nalgebra::Vector2::new(
                        current_object.cell.position.x - other_object.cell.position.x,
                        current_object.cell.position.y - other_object.cell.position.y,
                    );
                    current_object_velocity = current_object_velocity
                        .cap_magnitude(current_object.cell.get_velocity_max_magnitude());

                    let mut other_object_velocity = nalgebra::Vector2::new(
                        other_object.cell.position.x - current_object.cell.position.x,
                        other_object.cell.position.y - current_object.cell.position.y,
                    );
                    other_object_velocity = other_object_velocity
                        .cap_magnitude(other_object.cell.get_velocity_max_magnitude());

                    object_list[current_object_index].cell.velocity = current_object_velocity;
                    object_list[other_object_index].cell.velocity = other_object_velocity;
                }
            }
        }
    }
    object_list.retain(|object| object.cell.energy > 0);
}
