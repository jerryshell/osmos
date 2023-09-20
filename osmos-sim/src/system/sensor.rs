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
        // danger: -1.0 * other_object_energy / distance
        // equal:  -0.5 * other_object_energy / distance
        // normal:  0.0
        // food:    1.0 * other_object_energy / distance
        // [up, right, down, left]
        let mut sensor_data_list = [0.0; 4];
        in_sensor_range_other_object_index_list
            .iter()
            .for_each(|&other_object_index| {
                let distance = nalgebra::distance(
                    &object_list[other_object_index].cell.position,
                    &object_list[current_object_index].cell.position,
                );

                let current_object_energy = object_list[current_object_index].cell.energy;
                let other_object_energy = object_list[other_object_index].cell.energy;

                let status = match current_object_energy {
                    _ if other_object_energy > current_object_energy => -1.0,
                    _ if other_object_energy == current_object_energy => -0.5,
                    _ if other_object_energy < current_object_energy => 1.0,
                    _ => 0.0,
                } * other_object_energy as f64
                    / distance;

                let current_object_position = object_list[current_object_index].cell.position;
                let other_object_position = object_list[other_object_index].cell.position;

                // up
                if other_object_position.y < current_object_position.y {
                    sensor_data_list[0] += status;
                }
                // right
                if other_object_position.x > current_object_position.x {
                    sensor_data_list[1] += status;
                }
                // down
                if other_object_position.y > current_object_position.y {
                    sensor_data_list[2] += status;
                }
                // left
                if other_object_position.x < current_object_position.x {
                    sensor_data_list[3] += status;
                }
            });
        object_list[current_object_index].cell.sensor.data_list = sensor_data_list;
    }
}

#[cfg(test)]
mod tests {
    mod process {
        mod danger_rd {
            #[test]
            fn test() {
                let mut rng = rand::thread_rng();
                let mut object_1 = crate::object::Object::new(&mut rng, 1, 1.0, 1.0);
                object_1.cell.position.x = 0.0;
                object_1.cell.position.y = 0.0;
                object_1.cell.energy = 1;
                let mut object_2 = crate::object::Object::new(&mut rng, 2, 1.0, 1.0);
                object_2.cell.position.x = 0.1;
                object_2.cell.position.y = 0.1;
                object_2.cell.energy = 2;
                let mut object_list = [object_1, object_2];
                crate::system::sensor::process(&mut object_list);
                assert_eq!(
                    object_list[0].cell.sensor.data_list,
                    [0.0, -14.142135623730947, -14.142135623730947, 0.0]
                );
                assert_eq!(
                    object_list[1].cell.sensor.data_list,
                    [7.071067811865474, 0.0, 0.0, 7.071067811865474]
                );
            }
        }
        mod danger_lu {
            #[test]
            fn test() {
                let mut rng = rand::thread_rng();
                let mut object_1 = crate::object::Object::new(&mut rng, 1, 1.0, 1.0);
                object_1.cell.position.x = 0.0;
                object_1.cell.position.y = 0.0;
                object_1.cell.energy = 1;
                let mut object_2 = crate::object::Object::new(&mut rng, 2, 1.0, 1.0);
                object_2.cell.position.x = -0.1;
                object_2.cell.position.y = -0.1;
                object_2.cell.energy = 2;
                let mut object_list = [object_1, object_2];
                crate::system::sensor::process(&mut object_list);
                assert_eq!(
                    object_list[0].cell.sensor.data_list,
                    [-14.142135623730947, 0.0, 0.0, -14.142135623730947]
                );
            }
        }
        mod danger_ru {
            #[test]
            fn test() {
                let mut rng = rand::thread_rng();
                let mut object_1 = crate::object::Object::new(&mut rng, 1, 1.0, 1.0);
                object_1.cell.position.x = 0.0;
                object_1.cell.position.y = 0.0;
                object_1.cell.energy = 1;
                let mut object_2 = crate::object::Object::new(&mut rng, 2, 1.0, 1.0);
                object_2.cell.position.x = 0.1;
                object_2.cell.position.y = -0.1;
                object_2.cell.energy = 2;
                let mut object_list = [object_1, object_2];
                crate::system::sensor::process(&mut object_list);
                assert_eq!(
                    object_list[0].cell.sensor.data_list,
                    [-14.142135623730947, -14.142135623730947, 0.0, 0.0]
                );
            }
        }
        mod danger_ld {
            #[test]
            fn test() {
                let mut rng = rand::thread_rng();
                let mut object_1 = crate::object::Object::new(&mut rng, 1, 1.0, 1.0);
                object_1.cell.position.x = 0.0;
                object_1.cell.position.y = 0.0;
                object_1.cell.energy = 1;
                let mut object_2 = crate::object::Object::new(&mut rng, 2, 1.0, 1.0);
                object_2.cell.position.x = -0.3;
                object_2.cell.position.y = 0.3;
                object_2.cell.energy = 2;
                let mut object_list = [object_1, object_2];
                crate::system::sensor::process(&mut object_list);
                assert_eq!(
                    object_list[0].cell.sensor.data_list,
                    [0.0, 0.0, -4.714045207910317, -4.714045207910317]
                );
            }
        }
        mod food_ld {
            #[test]
            fn test() {
                let mut rng = rand::thread_rng();
                let mut object_1 = crate::object::Object::new(&mut rng, 1, 1.0, 1.0);
                object_1.cell.position.x = 0.0;
                object_1.cell.position.y = 0.0;
                object_1.cell.energy = 2;
                let mut object_2 = crate::object::Object::new(&mut rng, 2, 1.0, 1.0);
                object_2.cell.position.x = -0.1;
                object_2.cell.position.y = 0.1;
                object_2.cell.energy = 3;
                let mut object_3 = crate::object::Object::new(&mut rng, 3, 1.0, 1.0);
                object_3.cell.position.x = -0.01;
                object_3.cell.position.y = 0.01;
                object_3.cell.energy = 1;
                let mut object_list = [object_1, object_2, object_3];
                crate::system::sensor::process(&mut object_list);
                assert_eq!(
                    object_list[0].cell.sensor.data_list,
                    [0.0, 0.0, 49.49747468305833, 49.49747468305833]
                );
            }
        }
        mod equal_rd {
            #[test]
            fn test() {
                let mut rng = rand::thread_rng();
                let mut object_1 = crate::object::Object::new(&mut rng, 1, 1.0, 1.0);
                object_1.cell.position.x = 0.0;
                object_1.cell.position.y = 0.0;
                object_1.cell.energy = 2;
                let mut object_2 = crate::object::Object::new(&mut rng, 2, 1.0, 1.0);
                object_2.cell.position.x = -0.1;
                object_2.cell.position.y = 0.1;
                object_2.cell.energy = 2;
                let mut object_3 = crate::object::Object::new(&mut rng, 3, 1.0, 1.0);
                object_3.cell.position.x = -0.01;
                object_3.cell.position.y = 0.01;
                object_3.cell.energy = 2;
                let mut object_list = [object_1, object_2, object_3];
                crate::system::sensor::process(&mut object_list);
                assert_eq!(
                    object_list[0].cell.sensor.data_list,
                    [0.0, 0.0, -77.78174593052023, -77.78174593052023]
                );
                assert_eq!(
                    object_list[1].cell.sensor.data_list,
                    [-14.927809825049334, -14.927809825049334, 0.0, 0.0]
                );
                assert_eq!(
                    object_list[2].cell.sensor.data_list,
                    [
                        -70.71067811865476,
                        -70.71067811865476,
                        -7.85674201318386,
                        -7.85674201318386
                    ]
                );
            }
        }
    }
}
