pub struct Cell {
    pub position: nalgebra::Point2<f32>,
    pub velocity: nalgebra::Vector2<f32>,
    pub velocity_max_magnitude: f32,
    pub energy: usize,
}

impl Cell {
    pub fn random(rng: &mut rand::rngs::ThreadRng) -> Self {
        let x = rand::Rng::gen_range(rng, 0.0..=1.0);
        let y = rand::Rng::gen_range(rng, 0.0..=1.0);
        let position = nalgebra::Point2::new(x, y);
        let velocity = nalgebra::Vector2::new(0.0, 0.0);
        Self {
            position,
            velocity,
            velocity_max_magnitude: 0.005,
            energy: 1,
        }
    }

    pub fn step(&mut self, rng: &mut rand::rngs::ThreadRng) {
        let rate = 0.0005;
        let acceleration_x = rand::Rng::gen_range(rng, -rate..=rate);
        let acceleration_y = rand::Rng::gen_range(rng, -rate..=rate);
        let acceleration = nalgebra::Vector2::new(acceleration_x, acceleration_y);

        self.velocity += acceleration;
        self.velocity = nalgebra::clamp(
            self.velocity,
            nalgebra::Vector2::new(-self.velocity_max_magnitude, -self.velocity_max_magnitude),
            nalgebra::Vector2::new(self.velocity_max_magnitude, self.velocity_max_magnitude),
        );

        self.position += self.velocity;
        self.position.x = nalgebra::wrap(self.position.x, 0.0, 1.0);
        self.position.y = nalgebra::wrap(self.position.y, 0.0, 1.0);
    }
}
