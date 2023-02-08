use crate::object_ga;

pub struct Simulator {
    pub rng: rand::rngs::ThreadRng,
    pub object_count: usize,
    pub object_list: Vec<crate::object::Object>,
    pub step_count: usize,
    pub max_step_count_per_epoch: usize,
}

impl Default for Simulator {
    fn default() -> Self {
        let object_count = 400;
        let mut rng = rand::thread_rng();
        let object_list = (0..object_count)
            .map(|_| crate::object::Object::new(&mut rng))
            .collect();
        Self {
            rng,
            object_count,
            object_list,
            step_count: 0,
            max_step_count_per_epoch: 500,
        }
    }
}

impl Simulator {
    pub fn step(&mut self) {
        self.step_count += 1;

        crate::system::sensor::process(&mut self.object_list);
        crate::system::network::process(&mut self.object_list);
        crate::system::movement::process(&mut self.object_list);
        crate::system::collision::process(&mut self.object_list);

        if self.step_count >= self.max_step_count_per_epoch {
            self.evolve();
            self.step_count = 0;
        }
    }

    pub fn selection(&mut self) -> usize {
        osmos_ga::selection(&mut self.rng, &self.object_list)
    }

    fn evolve(&mut self) {
        let new_object_list = (0..self.object_count)
            .map(|_| {
                let parent_a_index = self.selection();
                let parent_b_index = self.selection();
                let parent_a = &self.object_list[parent_a_index];
                let parent_b = &self.object_list[parent_b_index];

                let mut child_gene_data_list =
                    osmos_ga::crossover(&mut self.rng, parent_a, parent_b);

                osmos_ga::mutation(&mut self.rng, 0.01, 0.3, &mut child_gene_data_list);

                let child_network = object_ga::build_network_from_gene_data_list(
                    &[4, 16, 4],
                    &child_gene_data_list,
                );

                crate::object::Object::from_network(&mut self.rng, child_network)
            })
            .collect::<Vec<crate::object::Object>>();
        self.object_list = new_object_list;
    }
}

#[cfg(test)]
mod tests {
    mod selection {
        #[test]
        fn test() {
            let mut sim = crate::simulator::Simulator::default();
            for _ in 0..10 {
                sim.step();
                sim.selection();
            }
        }
    }
}
