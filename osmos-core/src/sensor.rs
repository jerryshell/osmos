pub struct Sensor {
    pub range: f64,
    pub data_list: Vec<f64>,
}

impl Sensor {
    pub fn new(range: f64) -> Self {
        Self {
            range,
            data_list: vec![],
        }
    }
}
