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
        let rate = 0.001;
        let v_x = rand::Rng::gen_range(rng, -rate..=rate);
        let v_y = rand::Rng::gen_range(rng, -rate..=rate);
        let v = nalgebra::Vector2::new(v_x, v_y);
        self.velocity += v;
        self.velocity = nalgebra::clamp(
            self.velocity,
            nalgebra::Vector2::new(-self.velocity_max_magnitude, -self.velocity_max_magnitude),
            nalgebra::Vector2::new(self.velocity_max_magnitude, self.velocity_max_magnitude),
        );
        self.position += self.velocity;
        self.position.x = wrap(self.position.x, 0.0, 1.0);
        self.position.y = wrap(self.position.y, 0.0, 1.0);
    }
}

fn wrap(n: f32, min: f32, max: f32) -> f32 {
    let mut result = n;
    if n > max {
        result = 0.0
    } else if n < min {
        result = 1.0
    }
    result
}
