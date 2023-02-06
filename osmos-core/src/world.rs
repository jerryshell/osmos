pub struct World {
    pub rng: rand::rngs::ThreadRng,
    pub cell_list: Vec<crate::cell::Cell>,
}

impl World {
    pub fn random() -> Self {
        let mut rng = rand::thread_rng();
        let cell_list = (0..100)
            .map(|_| crate::cell::Cell::random(&mut rng))
            .collect();
        Self { rng, cell_list }
    }

    pub fn step(&mut self) {
        crate::system::sensor::process(&mut self.cell_list);
        self.cell_list.iter_mut().for_each(|cell| cell.step());
        crate::system::movement::process(&mut self.cell_list);
    }
}
