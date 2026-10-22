use osmos_sim::SimConfig;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Scenario {
    Ambient,
    Sentient,
}

impl Scenario {
    pub const fn from_id(id: u32) -> Self {
        match id {
            1 => Self::Sentient,
            _ => Self::Ambient,
        }
    }

    pub const fn id(self) -> u32 {
        match self {
            Self::Ambient => 0,
            Self::Sentient => 1,
        }
    }

    pub const fn label(self) -> &'static str {
        match self {
            Self::Ambient => "Ambient",
            Self::Sentient => "Sentient",
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
        }
    }

    pub const fn hunters(self) -> u32 {
        match self {
            Self::Ambient => 0,
            Self::Sentient => 4,
        }
    }

    pub const fn prey(self) -> u32 {
        match self {
            Self::Ambient => 0,
            Self::Sentient => 12,
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
}
