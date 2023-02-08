pub struct Cell {
    pub position: nalgebra::Point2<f64>,
    pub acceleration: nalgebra::Vector2<f64>,
    pub velocity: nalgebra::Vector2<f64>,
    pub velocity_max_magnitude: f64,
    pub energy: usize,
    pub sensor: crate::sensor::Sensor,
}

impl Cell {
    pub fn random(rng: &mut rand::rngs::ThreadRng) -> Self {
        let random_x = rand::Rng::gen_range(rng, 0.0..=1.0);
        let random_y = rand::Rng::gen_range(rng, 0.0..=1.0);
        Self {
            position: nalgebra::Point2::new(random_x, random_y),
            acceleration: nalgebra::Vector2::new(0.0, 0.0),
            velocity: nalgebra::Vector2::new(0.0, 0.0),
            velocity_max_magnitude: 0.001,
            energy: rand::Rng::gen_range(rng, 1..=2),
            sensor: crate::sensor::Sensor::new(0.8),
        }
    }
}
