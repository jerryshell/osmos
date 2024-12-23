pub struct Simulator {
    pub rng: rand::rngs::ThreadRng,
    pub object_count: usize,
    pub object_list: Vec<crate::object::Object>,
    pub step_count: usize,
    pub max_step_count_per_epoch: usize,
    pub min_object_count_per_epoch: usize,
    pub epoch_count: usize,
}

impl Default for Simulator {
    fn default() -> Self {
        let object_count = 300;
        let mut rng = rand::thread_rng();
        let object_list = (0..object_count)
            .map(|id| crate::object::Object::new(&mut rng, id))
            .collect();
        Self {
            rng,
            object_count,
            object_list,
            step_count: 0,
            max_step_count_per_epoch: 5000,
            min_object_count_per_epoch: 100,
            epoch_count: 0,
        }
    }
}

impl Simulator {
    pub fn step(&mut self) {
        crate::system::sensor::process(&mut self.object_list);
        crate::system::network::process(&mut self.object_list);
        crate::system::collision::process(&mut self.object_list);
        crate::system::movement::process(&mut self.rng, &mut self.object_list);
        crate::system::epoch::process(self);
    }
}
