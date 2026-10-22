use osmos_core::Vec2;

pub const GRID_SIDE: usize = 4;
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
        let x0 = x.saturating_sub(1);
        let y0 = y.saturating_sub(1);
        let x1 = (x + 1).min(GRID_SIDE - 1);
        let y1 = (y + 1).min(GRID_SIDE - 1);

        for cy in y0..=y1 {
            for cx in x0..=x1 {
                out.extend_from_slice(&self.cells[cy * GRID_SIDE + cx]);
            }
        }
    }
}

fn axis(value: f32) -> usize {
    ((value / CELL_SIZE).floor() as isize).clamp(0, GRID_SIDE as isize - 1) as usize
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
        assert!(!found.contains(&2));
    }

    #[test]
    fn edges_are_clamped() {
        let mut grid = Grid::new();
        grid.insert(7, Vec2::new(1.0, 0.0));

        let mut found = Vec::new();
        grid.nearby(Vec2::new(1.0, 0.0), &mut found);

        assert_eq!(found, vec![7]);
    }
}
