pub fn process(cell_list: &mut [crate::cell::Cell]) {
    for current_cell_index in 0..cell_list.len() {
        // get oter cell index
        let other_cell_index_list = cell_list
            .iter()
            .enumerate()
            .map(|(index, _)| index)
            .filter(|&index| index != current_cell_index)
            .collect::<Vec<usize>>();
        // get in distance other cell index
        let mut in_distance_other_cell_index_list = other_cell_index_list
            .iter()
            .filter(|&&other_cell_index| {
                let distance = nalgebra::distance(
                    &cell_list[current_cell_index].position,
                    &cell_list[other_cell_index].position,
                );
                distance <= cell_list[current_cell_index].sensor.range
            })
            .copied()
            .collect::<Vec<usize>>();

        // in_distance_other_cell_index_list order by distance desc
        // old code:
        // in_distance_other_cell_index_list.sort_by(|&a_index, &b_index| {
        //     let a_distence = nalgebra::distance(
        //         &cell_list[a_index].position,
        //         &cell_list[current_cell_index].position,
        //     );
        //     let b_distence = nalgebra::distance(
        //         &cell_list[b_index].position,
        //         &cell_list[current_cell_index].position,
        //     );
        //     b_distence.partial_cmp(&a_distence).unwrap()
        // });
        // ---
        // use sort_by_cached_key() optimize
        in_distance_other_cell_index_list.sort_by_cached_key(|&other_cell_index| {
            let distance = nalgebra::distance(
                &cell_list[other_cell_index].position,
                &cell_list[current_cell_index].position,
            );
            -(distance * 1000.0) as isize
        });

        // set sensor_data_list by energy and position
        // danger -1
        // normal 0
        // food   1
        // [up, right, down, left]
        let mut result = vec![0.0, 0.0, 0.0, 0.0];
        in_distance_other_cell_index_list
            .iter()
            .for_each(|&other_cell_index| {
                let status = match other_cell_index {
                    other_cell_index
                        if cell_list[other_cell_index].energy
                            > cell_list[current_cell_index].energy =>
                    {
                        -1.0
                    }
                    other_cell_index
                        if cell_list[other_cell_index].energy
                            < cell_list[current_cell_index].energy =>
                    {
                        1.0
                    }
                    _ => 0.0,
                };
                if cell_list[other_cell_index].position.y < cell_list[current_cell_index].position.y
                {
                    // up
                    result[0] = status;
                }
                if cell_list[other_cell_index].position.x > cell_list[current_cell_index].position.x
                {
                    // right
                    result[1] = status;
                }
                if cell_list[other_cell_index].position.y > cell_list[current_cell_index].position.y
                {
                    // down
                    result[2] = status;
                }
                if cell_list[other_cell_index].position.x < cell_list[current_cell_index].position.x
                {
                    // left
                    result[3] = status;
                }
                cell_list[current_cell_index].sensor_data_list = result.clone();
            });
    }
}

#[cfg(test)]
mod tests {
    mod process {
        mod danger_rd {
            #[test]
            fn test() {
                let mut rng = rand::thread_rng();
                let mut cell_1 = crate::cell::Cell::random(&mut rng);
                cell_1.position.x = 0.0;
                cell_1.position.y = 0.0;
                cell_1.energy = 1;
                let mut cell_2 = crate::cell::Cell::random(&mut rng);
                cell_2.position.x = 0.1;
                cell_2.position.y = 0.1;
                cell_2.energy = 2;
                let mut cell_list = vec![cell_1, cell_2];
                crate::system::sensor::process(&mut cell_list);
                assert_eq!(cell_list[0].sensor_data_list, vec![0.0, -1.0, -1.0, 0.0]);
                assert_eq!(cell_list[1].sensor_data_list, vec![1.0, 0.0, 0.0, 1.0]);
            }
        }
        mod danger_ul {
            #[test]
            fn test() {
                let mut rng = rand::thread_rng();
                let mut cell_1 = crate::cell::Cell::random(&mut rng);
                cell_1.position.x = 0.0;
                cell_1.position.y = 0.0;
                cell_1.energy = 1;
                let mut cell_2 = crate::cell::Cell::random(&mut rng);
                cell_2.position.x = -0.1;
                cell_2.position.y = -0.1;
                cell_2.energy = 2;
                let mut cell_list = vec![cell_1, cell_2];
                crate::system::sensor::process(&mut cell_list);
                assert_eq!(cell_list[0].sensor_data_list, vec![-1.0, 0.0, 0.0, -1.0])
            }
        }
        mod danger_ur {
            #[test]
            fn test() {
                let mut rng = rand::thread_rng();
                let mut cell_1 = crate::cell::Cell::random(&mut rng);
                cell_1.position.x = 0.0;
                cell_1.position.y = 0.0;
                cell_1.energy = 1;
                let mut cell_2 = crate::cell::Cell::random(&mut rng);
                cell_2.position.x = 0.1;
                cell_2.position.y = -0.1;
                cell_2.energy = 2;
                let mut cell_list = vec![cell_1, cell_2];
                crate::system::sensor::process(&mut cell_list);
                assert_eq!(cell_list[0].sensor_data_list, vec![-1.0, -1.0, 0.0, 0.0])
            }
        }
        mod danger_dl {
            #[test]
            fn test() {
                let mut rng = rand::thread_rng();
                let mut cell_1 = crate::cell::Cell::random(&mut rng);
                cell_1.position.x = 0.0;
                cell_1.position.y = 0.0;
                cell_1.energy = 1;
                let mut cell_2 = crate::cell::Cell::random(&mut rng);
                cell_2.position.x = -0.3;
                cell_2.position.y = 0.3;
                cell_2.energy = 2;
                let mut cell_list = vec![cell_1, cell_2];
                crate::system::sensor::process(&mut cell_list);
                assert_eq!(cell_list[0].sensor_data_list, vec![0.0, 0.0, -1.0, -1.0])
            }
        }
        mod safe_dl {
            #[test]
            fn test() {
                let mut rng = rand::thread_rng();
                let mut cell_1 = crate::cell::Cell::random(&mut rng);
                cell_1.position.x = 0.0;
                cell_1.position.y = 0.0;
                cell_1.energy = 2;
                let mut cell_2 = crate::cell::Cell::random(&mut rng);
                cell_2.position.x = -0.1;
                cell_2.position.y = 0.1;
                cell_2.energy = 3;
                let mut cell_3 = crate::cell::Cell::random(&mut rng);
                cell_3.position.x = -0.01;
                cell_3.position.y = 0.01;
                cell_3.energy = 1;
                let mut cell_list = vec![cell_1, cell_2, cell_3];
                crate::system::sensor::process(&mut cell_list);
                assert_eq!(cell_list[0].sensor_data_list, vec![0.0, 0.0, 1.0, 1.0])
            }
        }
    }
}
