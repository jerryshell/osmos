pub struct Simulator {
    pub max_x: f64,
    pub max_y: f64,
    pub rng: rand::rngs::ThreadRng,
    pub object_count: usize,
    pub object_list: Vec<crate::object::Object>,
    pub step_count: usize,
    pub max_step_count_per_epoch: usize,
    pub epoch_count: usize,
}

impl Simulator {
    pub fn new(max_x: f64, max_y: f64) -> Self {
        let object_count = 300;
        let mut rng = rand::thread_rng();
        let object_list = (0..object_count)
            .map(|id| crate::object::Object::new(&mut rng, id, max_x, max_y))
            .collect();
        Self {
            max_x,
            max_y,
            rng,
            object_count,
            object_list,
            step_count: 0,
            max_step_count_per_epoch: 2000,
            epoch_count: 0,
        }
    }

    pub fn step(&mut self) {
        crate::system::sensor::process(&mut self.object_list);
        crate::system::network::process(&mut self.object_list);
        crate::system::collision::process(&mut self.object_list);
        crate::system::movement::process(&mut self.rng, &mut self.object_list);
        crate::system::epoch::process(self);
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test() {
        let mut sim = crate::simulator::Simulator::new(1.0, 1.0);
        for _ in 0..5 {
            sim.step();
            crate::ga::evolve::evolve(&mut sim);
        }
    }
}
