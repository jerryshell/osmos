pub const TICKS_PER_SECOND: u32 = 60;
pub const SUBSTEPS: u32 = 2;

#[derive(Clone, Copy, Debug)]
pub struct SimConfig {
    pub seed: u64,
    pub motes: u32,
    pub pellets: u32,
    pub mote_mass: f32,
    pub initial_mass_spread: f32,
    pub pellet_mass: f32,
    pub max_speed: f32,
    pub collision_restitution: f32,
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
    pub pellet_reserve: f32,
    pub owner_immunity_ticks: u32,
    pub gravity_center_x: f32,
    pub gravity_center_y: f32,
    pub gravity_strength: f32,
    pub gravity_softening: f32,
    pub gravity_power: f32,
    pub gravity_max_accel: f32,
    pub gravity_well_radius: f32,
    pub orbit_speed_factor: f32,
}

impl Default for SimConfig {
    fn default() -> Self {
        Self {
            seed: 0x0D15_EA5E,
            motes: 64,
            pellets: 236,
            mote_mass: 1.0,
            initial_mass_spread: 0.75,
            pellet_mass: 0.12,
            max_speed: 0.25,
            collision_restitution: 0.8,
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
            pellet_reserve: 120.0,
            owner_immunity_ticks: 18,
            gravity_center_x: 0.5,
            gravity_center_y: 0.5,
            gravity_strength: 0.0,
            gravity_softening: 0.04,
            gravity_power: 2.0,
            gravity_max_accel: 0.12,
            gravity_well_radius: 0.045,
            orbit_speed_factor: 0.0,
        }
    }
}

impl SimConfig {
    pub fn validate(&self) {
        assert!(self.motes > 0, "at least one mote is required");
        assert!(self.mote_mass.is_finite() && self.mote_mass > 0.0);
        assert!(self.initial_mass_spread.is_finite() && self.initial_mass_spread >= 0.0);
        assert!(self.pellet_mass.is_finite() && self.pellet_mass > 0.0);
        assert!(self.max_speed.is_finite() && self.max_speed > 0.0);
        assert!((0.0..=1.0).contains(&self.collision_restitution));
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
        assert!(self.pellet_reserve.is_finite() && self.pellet_reserve >= 0.0);
        assert!(self.gravity_center_x.is_finite() && (0.0..=1.0).contains(&self.gravity_center_x));
        assert!(self.gravity_center_y.is_finite() && (0.0..=1.0).contains(&self.gravity_center_y));
        assert!(self.gravity_strength.is_finite());
        assert!(self.gravity_softening.is_finite() && self.gravity_softening > 0.0);
        assert!(self.gravity_power.is_finite() && self.gravity_power > 0.0);
        assert!(self.gravity_max_accel.is_finite() && self.gravity_max_accel > 0.0);
        assert!(self.gravity_well_radius.is_finite() && self.gravity_well_radius > 0.0);
        assert!(self.orbit_speed_factor.is_finite() && self.orbit_speed_factor >= 0.0);
    }
}
