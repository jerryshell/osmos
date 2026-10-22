use rand::RngExt;

pub struct Cell {
    pub position: nalgebra::Point2<f32>,
    pub direction: nalgebra::Vector2<f32>,
    pub velocity: nalgebra::Vector2<f32>,
    pub energy: usize,
    pub sensor: crate::sensor::Sensor,
}

impl Cell {
    pub fn random(rng: &mut impl rand::Rng) -> Self {
        let sensor_range = 0.5;
        let mut cell = Self {
            position: nalgebra::Point2::new(0.0, 0.0),
            direction: nalgebra::Vector2::new(0.0, 0.0),
            velocity: nalgebra::Vector2::new(0.0, 0.0),
            energy: rand::RngExt::random_range(rng, 1..=2),
            sensor: crate::sensor::Sensor::new(sensor_range),
        };
        cell.random_position(rng);
        cell
    }

    pub fn random_position(&mut self, rng: &mut impl rand::Rng) {
        self.position.x = rng.random();
        self.position.y = rng.random();
    }

    pub fn get_speed(&self) -> f32 {
        0.001 + (1.0 / self.energy as f32) * 0.003
    }
}
