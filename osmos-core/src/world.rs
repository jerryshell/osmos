pub struct World {
    pub rng: rand::rngs::ThreadRng,
    pub cell_list: Vec<crate::cell::Cell>,
}

impl World {
    pub fn random() -> Self {
        let mut rng = rand::thread_rng();
        let cell_list = (0..50)
            .map(|_| crate::cell::Cell::random(&mut rng))
            .collect();
        Self { rng, cell_list }
    }

    pub fn step(&mut self) {
        let c = self.cell_list.clone();
        self.cell_list
            .iter_mut()
            .for_each(|cell| cell.step(&mut self.rng, &c));
    }
}
