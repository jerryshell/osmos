use osmos_sim::{SimConfig, Snapshot, World};

pub struct Engine {
    config: SimConfig,
    world: World,
}

impl Engine {
    pub fn new(seed: u64) -> Self {
        Self::with_config(SimConfig {
            seed,
            ..SimConfig::default()
        })
    }

    pub fn with_config(config: SimConfig) -> Self {
        Self {
            config,
            world: World::new(config),
        }
    }

    pub fn step(&mut self, steps: u32) {
        for _ in 0..steps {
            self.world.step();
        }
    }

    pub fn reset(&mut self, seed: u64) {
        self.config.seed = seed;
        self.world = World::new(self.config);
    }

    pub fn snapshot(&self) -> Snapshot {
        self.world.snapshot()
    }

    pub fn tick(&self) -> u64 {
        self.world.tick()
    }

    pub fn config(&self) -> SimConfig {
        self.config
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn same_seed_produces_the_same_snapshot() {
        let mut first = Engine::new(11);
        let mut second = Engine::new(11);

        first.step(120);
        second.step(120);

        let first = first.snapshot();
        let second = second.snapshot();

        assert_eq!(first.mote_ids, second.mote_ids);
        assert_eq!(first.mote_state, second.mote_state);
        assert_eq!(first.pellet_state, second.pellet_state);
    }

    #[test]
    fn tick_advances_by_the_requested_steps() {
        let mut engine = Engine::new(3);
        engine.step(7);
        assert_eq!(engine.tick(), 7);
    }

    #[test]
    fn reset_restarts_from_the_seed() {
        let mut engine = Engine::new(5);
        let initial = engine.snapshot();
        engine.step(30);
        engine.reset(5);
        let reset = engine.snapshot();

        assert_eq!(initial.mote_state, reset.mote_state);
        assert_eq!(initial.pellet_state, reset.pellet_state);
    }
}
