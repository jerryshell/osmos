pub const TICKS_PER_SECOND: u32 = 60;
pub const SUBSTEPS: u32 = 2;

#[derive(Clone, Copy, Debug)]
pub struct SimConfig {
    pub seed: u64,
    pub motes: u32,
    pub pellets: u32,
    pub mote_mass: f32,
    pub pellet_mass: f32,
    pub max_speed: f32,
    pub wall_restitution: f32,
    pub collision_restitution: f32,
    pub absorb_ratio: f32,
}

impl Default for SimConfig {
    fn default() -> Self {
        Self {
            seed: 0x0D15_EA5E,
            motes: 64,
            pellets: 236,
            mote_mass: 1.0,
            pellet_mass: 0.12,
            max_speed: 0.2,
            wall_restitution: 0.9,
            collision_restitution: 0.8,
            absorb_ratio: 1.15,
        }
    }
}

impl SimConfig {
    pub fn validate(&self) {
        assert!(self.motes > 0, "at least one mote is required");
        assert!(self.mote_mass.is_finite() && self.mote_mass > 0.0);
        assert!(self.pellet_mass.is_finite() && self.pellet_mass > 0.0);
        assert!(self.max_speed.is_finite() && self.max_speed > 0.0);
        assert!((0.0..=1.0).contains(&self.wall_restitution));
        assert!((0.0..=1.0).contains(&self.collision_restitution));
        assert!(self.absorb_ratio >= 1.0);
    }
}
