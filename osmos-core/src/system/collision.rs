pub fn process(cell_list: &mut Vec<crate::cell::Cell>) {
    for current_cell_index in 0..cell_list.len() {
        if cell_list[current_cell_index].energy == 0 {
            continue;
        }
        for other_cell_index in 0..cell_list.len() {
            if current_cell_index == other_cell_index {
                continue;
            }
            if cell_list[other_cell_index].energy == 0 {
                continue;
            }
            let distance = nalgebra::distance(
                &cell_list[current_cell_index].position,
                &cell_list[other_cell_index].position,
            );
            let energy_sum =
                cell_list[current_cell_index].energy + cell_list[other_cell_index].energy;
            if distance >= energy_sum as f32 / 1000.0 {
                continue;
            }
            if cell_list[current_cell_index].energy > cell_list[other_cell_index].energy {
                cell_list[current_cell_index].energy += 1;
                cell_list[other_cell_index].energy -= 1;
            }
            if cell_list[current_cell_index].energy < cell_list[other_cell_index].energy {
                cell_list[current_cell_index].energy -= 1;
                cell_list[other_cell_index].energy += 1;
            }
        }
    }
    cell_list.retain(|cell| cell.energy > 0);
}
