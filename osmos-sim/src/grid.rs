use osmos_core::Vec2;

pub const GRID_SIDE: usize = 8;
const CELL_SIZE: f32 = 1.0 / GRID_SIDE as f32;

#[derive(Clone, Debug, Default)]
pub struct Grid {
    cells: Vec<Vec<u32>>,
}

impl Grid {
    pub fn new() -> Self {
        Self {
            cells: (0..GRID_SIDE * GRID_SIDE).map(|_| Vec::new()).collect(),
        }
    }

    pub fn clear(&mut self) {
        for cell in &mut self.cells {
            cell.clear();
        }
    }

    pub fn insert(&mut self, index: usize, position: Vec2) {
        let x = axis(position.x);
        let y = axis(position.y);
        self.cells[y * GRID_SIDE + x].push(index as u32);
    }

    pub fn nearby(&self, position: Vec2, out: &mut Vec<u32>) {
        out.clear();
        let x = axis(position.x);
        let y = axis(position.y);
        let side = GRID_SIDE as isize;

        for dy in -2..=2 {
            for dx in -2..=2 {
                let cx = (x as isize + dx).rem_euclid(side) as usize;
                let cy = (y as isize + dy).rem_euclid(side) as usize;
                out.extend_from_slice(&self.cells[cy * GRID_SIDE + cx]);
            }
        }
    }
}

fn axis(value: f32) -> usize {
    let wrapped = if value < 0.0 {
        value + 1.0
    } else if value >= 1.0 {
        value - 1.0
    } else {
        value
    };
    ((wrapped / CELL_SIZE).floor() as isize).clamp(0, GRID_SIDE as isize - 1) as usize
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn nearby_returns_cells_in_the_three_by_three_window() {
        let mut grid = Grid::new();
        grid.insert(0, Vec2::new(0.1, 0.1));
        grid.insert(1, Vec2::new(0.4, 0.4));
        grid.insert(2, Vec2::new(0.9, 0.9));

        let mut found = Vec::new();
        grid.nearby(Vec2::new(0.15, 0.15), &mut found);

        assert!(found.contains(&0));
        assert!(found.contains(&1));
        assert!(found.contains(&2));
    }

    #[test]
    fn edges_wrap() {
        let mut grid = Grid::new();
        grid.insert(7, Vec2::new(0.98, 0.02));

        let mut found = Vec::new();
        grid.nearby(Vec2::new(0.02, 0.02), &mut found);

        assert_eq!(found, vec![7]);
    }
}
