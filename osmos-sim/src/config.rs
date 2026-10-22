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
    pub episode_ticks: u64,
    pub sensor_range: f32,
    pub max_eject_rate: f32,
    pub min_mass: f32,
    pub eject_speed: f32,
    pub target_mass: f32,
    pub ejecta_enabled: bool,
    pub ejecta_ttl_ticks: u32,
    pub ejecta_packet: f32,
    pub max_ejecta: usize,
    pub max_pellets: usize,
    pub owner_immunity_ticks: u32,
}

impl Default for SimConfig {
    fn default() -> Self {
        Self {
            seed: 0x0D15_EA5E,
            motes: 64,
            pellets: 236,
            mote_mass: 1.0,
            pellet_mass: 0.12,
            max_speed: 0.25,
            wall_restitution: 0.8,
            collision_restitution: 0.8,
            absorb_ratio: 2.0,
            episode_ticks: 2400,
            sensor_range: 0.25,
            max_eject_rate: 0.005,
            min_mass: 0.05,
            eject_speed: 2.0,
            target_mass: 4.0,
            ejecta_enabled: true,
            ejecta_ttl_ticks: 480,
            ejecta_packet: 0.01,
            max_ejecta: 512,
            max_pellets: 300,
            owner_immunity_ticks: 18,
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
        assert!(self.episode_ticks > 0);
        assert!(self.sensor_range.is_finite() && self.sensor_range > 0.0);
        assert!(self.max_eject_rate.is_finite() && self.max_eject_rate > 0.0);
        assert!(self.min_mass.is_finite() && self.min_mass > 0.0);
        assert!(self.eject_speed.is_finite() && self.eject_speed > 0.0);
        assert!(self.target_mass.is_finite() && self.target_mass > self.mote_mass);
        assert!(self.ejecta_ttl_ticks > 0);
        assert!(self.ejecta_packet.is_finite() && self.ejecta_packet > 0.0);
        assert!(self.max_ejecta > 0);
        assert!(self.max_pellets > 0);
    }
}
