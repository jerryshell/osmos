#[derive(Clone)]
pub struct Sensor {
    range: f32,
}

impl Sensor {
    pub fn new(range: f32) -> Self {
        Self { range }
    }

    /// danger -1
    /// normal 0
    /// food   1
    /// [up, right, down, left]
    pub fn process(&self, cell: &crate::cell::Cell, cell_list: &[crate::cell::Cell]) -> Vec<f32> {
        let mut result = vec![0.0, 0.0, 0.0, 0.0];
        let other_cell_list = cell_list
            .iter()
            .filter(|item| item.position == cell.position)
            .collect::<Vec<&crate::cell::Cell>>();
        let mut in_distance_cell_list = other_cell_list
            .iter()
            .filter(|item| {
                let distance = nalgebra::distance(&item.position, &cell.position);
                distance <= self.range
            })
            .copied()
            .collect::<Vec<&crate::cell::Cell>>();
        // TODO use sort_by_cached_key() optimize
        in_distance_cell_list.sort_by(|a, b| {
            let a_distence = nalgebra::distance(&a.position, &cell.position);
            let b_distence = nalgebra::distance(&b.position, &cell.position);
            b_distence.partial_cmp(&a_distence).unwrap()
        });
        in_distance_cell_list.iter().for_each(|item| {
            let status = match item {
                item if item.energy > cell.energy => -1.0,
                item if item.energy < cell.energy => 1.0,
                _ => 0.0,
            };
            if item.position.y < cell.position.y {
                // up
                result[0] = status;
            }
            if item.position.x > cell.position.x {
                // right
                result[1] = status;
            }
            if item.position.y > cell.position.y {
                // down
                result[2] = status;
            }
            if item.position.x < cell.position.x {
                // left
                result[3] = status;
            }
        });
        result
    }
}
