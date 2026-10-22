use osmos_sim::SimConfig;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Scenario {
    Ambient,
    Sentient,
    Orbital,
    Repulsor,
}

impl Scenario {
    pub const fn from_id(id: u32) -> Self {
        match id {
            1 => Self::Sentient,
            2 => Self::Orbital,
            3 => Self::Repulsor,
            _ => Self::Ambient,
        }
    }

    pub const fn id(self) -> u32 {
        match self {
            Self::Ambient => 0,
            Self::Sentient => 1,
            Self::Orbital => 2,
            Self::Repulsor => 3,
        }
    }

    pub const fn label(self) -> &'static str {
        match self {
            Self::Ambient => "Ambient",
            Self::Sentient => "Sentient",
            Self::Orbital => "Orbital",
            Self::Repulsor => "Repulsor",
        }
    }

    pub fn config(self, base: SimConfig) -> SimConfig {
        match self {
            Self::Ambient => base,
            Self::Sentient => SimConfig {
                motes: 48,
                pellets: 100,
                ..base
            },
            Self::Orbital => SimConfig {
                motes: 64,
                pellets: 180,
                gravity_strength: 0.002,
                orbit_speed_factor: 1.0,
                ..base
            },
            Self::Repulsor => SimConfig {
                motes: 64,
                pellets: 180,
                gravity_strength: -0.002,
                orbit_speed_factor: 1.0,
                ..base
            },
        }
    }

    pub const fn hunters(self) -> u32 {
        match self {
            Self::Sentient => 4,
            _ => 0,
        }
    }

    pub const fn prey(self) -> u32 {
        match self {
            Self::Sentient => 12,
            _ => 0,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sentient_has_hunters_and_prey() {
        assert_eq!(Scenario::from_id(1), Scenario::Sentient);
        assert!(Scenario::Sentient.hunters() > 0);
        assert!(Scenario::Sentient.prey() > 0);
        assert_eq!(Scenario::Sentient.config(SimConfig::default()).motes, 48);
    }

    #[test]
    fn field_scenarios_are_signed() {
        assert_eq!(Scenario::from_id(2), Scenario::Orbital);
        assert_eq!(Scenario::from_id(3), Scenario::Repulsor);
        assert!(
            Scenario::Orbital
                .config(SimConfig::default())
                .gravity_strength
                > 0.0
        );
        assert!(
            Scenario::Repulsor
                .config(SimConfig::default())
                .gravity_strength
                < 0.0
        );
    }
}
