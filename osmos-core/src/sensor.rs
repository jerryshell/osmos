pub struct Sensor {
    pub range: f64,
    pub data_list: [f64; 4],
}

impl Sensor {
    pub fn new(range: f64) -> Self {
        Self {
            range,
            data_list: [0.0; 4],
        }
    }
}
