pub struct Sensor {
    pub range: f32,
    // [up, right, down, left]
    pub data_list: [f32; 4],
}

impl Sensor {
    pub fn new(range: f32) -> Self {
        Self {
            range,
            data_list: [0.0; 4],
        }
    }
}
