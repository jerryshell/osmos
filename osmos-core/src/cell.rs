#[derive(Clone)]
pub struct Cell {
    pub position: nalgebra::Point2<f32>,
    pub acceleration: nalgebra::Vector2<f32>,
    pub velocity: nalgebra::Vector2<f32>,
    pub velocity_max_magnitude: f32,
    pub energy: usize,
    pub sensor: crate::sensor::Sensor,
    pub sensor_data_list: Vec<f32>,
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
        let network = osmos_nn::network::Network::random(rng, &[4, 10, 5]);
        Self {
            position,
            acceleration,
            velocity,
            velocity_max_magnitude: 0.001,
            energy: rand::Rng::gen_range(rng, 1..=5),
            sensor,
            sensor_data_list: vec![0.0, 0.0, 0.0, 0.0],
            network,
            network_output: vec![],
        }
    }
}
