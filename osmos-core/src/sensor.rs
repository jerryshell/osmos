#[derive(Clone)]
pub struct Sensor {
    pub range: f32,
    pub data_list: Vec<f32>,
}

impl Sensor {
    pub fn new(range: f32) -> Self {
        Self {
            range,
            data_list: vec![],
        }
    }
}
