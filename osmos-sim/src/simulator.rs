pub struct Simulator {
    pub rng: rand::rngs::ThreadRng,
    pub object_list: Vec<crate::object::Object>,
}

impl Default for Simulator {
    fn default() -> Self {
        let mut rng = rand::thread_rng();
        let object_list = (0..400)
            .map(|_| crate::object::Object::new(&mut rng))
            .collect();
        Self { rng, object_list }
    }
}

impl Simulator {
    pub fn step(&mut self) {
        crate::system::sensor::process(&mut self.object_list);
        crate::system::network::process(&mut self.object_list);
        crate::system::movement::process(&mut self.object_list);
        crate::system::collision::process(&mut self.object_list);
    }
}
