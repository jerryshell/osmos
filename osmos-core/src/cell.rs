const BASE_SPEED: f32 = 2.0;

pub struct Cell {
    pub position: nalgebra::Point2<f32>,
    pub direction: nalgebra::Vector2<f32>,
    pub velocity: nalgebra::Vector2<f32>,
    pub energy: usize,
    pub sensor: crate::sensor::Sensor,
}

impl Cell {
    pub fn random(rng: &mut impl rand::RngCore, max_x: f32, max_y: f32) -> Self {
        let sensor_range = nalgebra::Vector2::new(max_x, max_y).magnitude() * 0.5;
        let mut cell = Self {
            position: nalgebra::Point2::new(0.0, 0.0),
            direction: nalgebra::Vector2::new(0.0, 0.0),
            velocity: nalgebra::Vector2::new(0.0, 0.0),
            energy: rand::Rng::gen_range(rng, 1..=3),
            sensor: crate::sensor::Sensor::new(sensor_range),
        };
        cell.random_position(rng, max_x, max_y);
        cell
    }

    pub fn random_position(&mut self, rng: &mut impl rand::RngCore, max_x: f32, max_y: f32) {
        let random_x = rand::Rng::gen_range(rng, 0.0..=max_x);
        let random_y = rand::Rng::gen_range(rng, 0.0..=max_y);
        self.position.x = random_x;
        self.position.y = random_y;
    }

    pub fn get_speed(&self) -> f32 {
        BASE_SPEED + (BASE_SPEED / self.energy as f32)
    }
}
