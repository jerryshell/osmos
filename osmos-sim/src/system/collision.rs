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

            // check collide
            let current_object_position = &object_list[current_object_index].cell.position;
            let other_object_position = &object_list[other_object_index].cell.position;
            let distance = nalgebra::distance(current_object_position, other_object_position);
            let energy_sum = current_object_energy + other_object_energy;
            let r_sum = energy_sum as f64;
            if distance >= r_sum {
                // current_object and other_object did not collide
                continue;
            }

            match () {
                _ if current_object_energy > other_object_energy => {
                    object_list[current_object_index].cell.energy += 1;
                    object_list[other_object_index].cell.energy -= 1;
                }
                _ if current_object_energy < other_object_energy => {
                    object_list[current_object_index].cell.energy -= 1;
                    object_list[other_object_index].cell.energy += 1;
                }
                _ => {
                    // energy equal, pull two Object apart using the opposite force
                    let current_object_velocity = (current_object_position - other_object_position)
                        .cap_magnitude(
                            object_list[current_object_index]
                                .cell
                                .get_max_velocity_magnitude(),
                        );

                    let other_object_velocity = (other_object_position - current_object_position)
                        .cap_magnitude(
                            object_list[other_object_index]
                                .cell
                                .get_max_velocity_magnitude(),
                        );

                    object_list[current_object_index].cell.velocity = current_object_velocity;
                    object_list[other_object_index].cell.velocity = other_object_velocity;
                }
            }
        }
    }
    // delete Object with energy <= 0
    object_list.retain(|object| object.cell.energy > 0);
}
