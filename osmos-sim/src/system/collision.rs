pub fn process(object_list: &mut Vec<crate::object::Object>) {
    for current_object_index in 0..object_list.len() {
        for other_object_index in 0..object_list.len() {
            if current_object_index == other_object_index {
                continue;
            }

            let current_object_energy = object_list[current_object_index].cell.energy;
            if current_object_energy == 0 {
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

            match current_object_energy.cmp(&other_object_energy) {
                std::cmp::Ordering::Greater => {
                    object_list[current_object_index].cell.energy += 1;
                    object_list[other_object_index].cell.energy -= 1;
                }
                std::cmp::Ordering::Less => {
                    object_list[current_object_index].cell.energy -= 1;
                    object_list[other_object_index].cell.energy += 1;
                }
                std::cmp::Ordering::Equal => {
                    let direction = (current_object_position - other_object_position).normalize();
                    let current_speed = object_list[current_object_index].cell.get_speed();
                    let other_speed = object_list[other_object_index].cell.get_speed();
                    object_list[current_object_index].cell.velocity = direction * current_speed;
                    object_list[other_object_index].cell.velocity = -direction * other_speed;
                }
            }
        }
    }
    // delete Object with energy <= 0
    object_list.retain(|object| object.cell.energy > 0);
}
