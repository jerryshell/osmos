#[derive(Clone)]
pub struct Cell {
    pub position: nalgebra::Point2<f32>,
    pub acceleration: nalgebra::Vector2<f32>,
    pub velocity: nalgebra::Vector2<f32>,
    pub velocity_max_magnitude: f32,
    pub energy: usize,
    pub sensor: crate::sensor::Sensor,
    pub network: osmos_nn::network::Network,
    pub network_output: Vec<f32>,
}

impl Cell {
    pub fn random(rng: &mut rand::rngs::ThreadRng) -> Self {
        let x = rand::Rng::gen_range(rng, 0.0..=1.0);
        let y = rand::Rng::gen_range(rng, 0.0..=1.0);
        let position = nalgebra::Point2::new(x, y);
        let acceleration = nalgebra::Vector2::new(0.0, 0.0);
        let velocity = nalgebra::Vector2::new(0.0, 0.0);
        let sensor = crate::sensor::Sensor::new(0.5);
        let network = osmos_nn::network::Network::random(rng, &[4, 10, 2]);
        Self {
            position,
            acceleration,
            velocity,
            velocity_max_magnitude: 0.005,
            energy: rand::Rng::gen_range(rng, 1..=5),
            sensor,
            network,
            network_output: vec![],
        }
    }

    pub fn step(&mut self, rng: &mut rand::rngs::ThreadRng, cell_list: &[Cell]) {
        self.process_network(cell_list);
        self.process_move(rng);
    }

    pub fn process_network(&mut self, cell_list: &[Cell]) {
        let rate = 0.0005;
        let sensor_output = self.sensor.process(self, cell_list);
        let mut nn_output = self.network.feed(sensor_output);
        nn_output = nn_output.iter().map(|n| sigmoid(*n) - 0.5).collect();
        self.network_output = nn_output.clone();
        self.acceleration = nalgebra::Vector2::new(
            nalgebra::clamp(nn_output[0] / 100.0, -rate, rate),
            nalgebra::clamp(nn_output[1] / 100.0, -rate, rate),
        );
    }

    pub fn process_move(&mut self, rng: &mut rand::rngs::ThreadRng) {
        // let rate = 0.0005;
        // self.acceleration = nalgebra::Vector2::new(
        //     rand::Rng::gen_range(rng, -rate..=rate),
        //     rand::Rng::gen_range(rng, -rate..=rate),
        // );

        self.velocity += self.acceleration;
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

pub fn sigmoid(x: f32) -> f32 {
    1.0 / (1.0 + std::f32::consts::E.powf(-x))
}
