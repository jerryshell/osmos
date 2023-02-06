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
            velocity_max_magnitude: 0.001,
            energy: rand::Rng::gen_range(rng, 1..=5),
            sensor,
            network,
            network_output: vec![],
        }
    }

    pub fn step(&mut self, rng: &mut rand::rngs::ThreadRng, cell_list: &[Cell]) {
        self.process_network(cell_list);
    }

    pub fn process_network(&mut self, cell_list: &[Cell]) {
        let sensor_output = self.sensor.process(self, cell_list);
        let mut nn_output = self.network.feed(sensor_output);
        // 0.0~1.0 => -0.5~0.5
        nn_output = nn_output.iter().map(|n| sigmoid(*n) - 0.5).collect();
        self.network_output = nn_output.clone();
        // -0.5~0.5 => -0.0005~0.0005
        self.acceleration = nalgebra::Vector2::new(nn_output[0] / 1000.0, nn_output[1] / 1000.0);
    }
}

pub fn sigmoid(x: f32) -> f32 {
    1.0 / (1.0 + std::f32::consts::E.powf(-x))
}
