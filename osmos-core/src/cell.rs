use rand::Rng;

pub struct Cell {
    pub position: nalgebra::Point2<f32>,
    pub direction: nalgebra::Vector2<f32>,
    pub velocity: nalgebra::Vector2<f32>,
    pub energy: usize,
    pub sensor: crate::sensor::Sensor,
}

impl Cell {
    pub fn random(rng: &mut impl rand::RngCore) -> Self {
        let sensor_range = 0.5;
        let mut cell = Self {
            position: nalgebra::Point2::new(0.0, 0.0),
            direction: nalgebra::Vector2::new(0.0, 0.0),
            velocity: nalgebra::Vector2::new(0.0, 0.0),
            energy: rand::Rng::gen_range(rng, 1..=2),
            sensor: crate::sensor::Sensor::new(sensor_range),
        };
        cell.random_position(rng);
        cell
    }

    pub fn random_position(&mut self, rng: &mut impl rand::RngCore) {
        self.position.x = rng.gen();
        self.position.y = rng.gen();
    }

    pub fn get_speed(&self) -> f32 {
        0.001 + (1.0 / self.energy as f32) * 0.0005
    }
}
