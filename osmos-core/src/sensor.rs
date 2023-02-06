#[derive(Clone)]
pub struct Sensor {
    pub range: f32,
}

impl Sensor {
    pub fn new(range: f32) -> Self {
        Self { range }
    }
}
