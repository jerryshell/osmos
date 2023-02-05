pub struct Cell {
    pub position: nalgebra::Point2<f32>,
    pub velocity: nalgebra::Vector2<f32>,
    pub energy: usize,
}

impl Cell {
    pub fn step(&mut self) {
        let mut rng = rand::thread_rng();
        let x = rand::Rng::gen_range(&mut rng, -1.0..=1.0);
        let y = rand::Rng::gen_range(&mut rng, -1.0..=1.0);
        let v = nalgebra::Vector2::new(x, y);
        self.position += v;
    }
}
