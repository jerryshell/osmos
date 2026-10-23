use osmos_core::{ControlNetwork, LineageId, Mass, MoteId, Vec2, SENSOR_INPUTS};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MoteRole {
    Ai,
    Hunter,
    Prey,
}

impl MoteRole {
    pub const fn as_u8(self) -> u8 {
        match self {
            Self::Ai => 0,
            Self::Hunter => 1,
            Self::Prey => 2,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MoteState {
    Alive,
    Absorbed,
    Starved,
}

impl MoteState {
    pub const fn is_alive(self) -> bool {
        matches!(self, Self::Alive)
    }
}

#[derive(Clone, Debug)]
pub struct Mote {
    pub id: MoteId,
    pub lineage: LineageId,
    pub role: MoteRole,
    pub mass: Mass,
    pub position: Vec2,
    pub velocity: Vec2,
    pub state: MoteState,
    pub absorbed_pellets: u32,
    pub absorbed_motes: u32,
    pub ejected_mass: f32,
    pub pending_ejecta: f32,
    pub controller: Option<ControlNetwork>,
    pub sensor: [f32; SENSOR_INPUTS],
    pub thrust: Vec2,
}

impl Mote {
    pub fn new(
        id: MoteId,
        lineage: LineageId,
        role: MoteRole,
        mass: Mass,
        position: Vec2,
        velocity: Vec2,
    ) -> Self {
        Self {
            id,
            lineage,
            role,
            mass,
            position,
            velocity,
            state: MoteState::Alive,
            absorbed_pellets: 0,
            absorbed_motes: 0,
            ejected_mass: 0.0,
            pending_ejecta: 0.0,
            controller: None,
            sensor: [0.0; SENSOR_INPUTS],
            thrust: Vec2::ZERO,
        }
    }

    pub fn radius(&self) -> f32 {
        self.mass.radius()
    }

    pub fn is_alive(&self) -> bool {
        self.state.is_alive()
    }

    pub fn hue(&self) -> f32 {
        (self.lineage.get() as f32 * 0.618_034).fract()
    }
}
