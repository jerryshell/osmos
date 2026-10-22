use osmos_core::Vec2;

use crate::config::SimConfig;

/// A fixed radial force field. Positive strength attracts, negative repels.
///
/// The field is acceleration, not force. `strength` is scaled by inverse
/// squared distance with a softening radius, then capped by `max_accel`.
#[derive(Clone, Copy, Debug)]
pub struct GravityWell {
    pub center: Vec2,
    pub strength: f32,
    pub softening: f32,
    pub max_accel: f32,
    pub radius: f32,
}

impl GravityWell {
    pub fn from_config(config: &SimConfig) -> Self {
        Self {
            center: Vec2::new(config.gravity_center_x, config.gravity_center_y),
            strength: config.gravity_strength,
            softening: config.gravity_softening,
            max_accel: config.gravity_max_accel,
            radius: config.gravity_well_radius,
        }
    }

    pub fn is_active(self) -> bool {
        self.strength.abs() > f32::EPSILON
    }

    pub fn acceleration(self, position: Vec2) -> Vec2 {
        if !self.is_active() {
            return Vec2::ZERO;
        }

        let delta = self.center - position;
        let distance_squared = delta.length_squared();
        let denominator = distance_squared + self.softening * self.softening;
        let magnitude = (self.strength / denominator).clamp(-self.max_accel, self.max_accel);
        delta.normalize_or_zero() * magnitude
    }

    pub fn kind(self) -> f32 {
        if self.strength > 0.0 {
            1.0
        } else if self.strength < 0.0 {
            -1.0
        } else {
            0.0
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn well(strength: f32) -> GravityWell {
        GravityWell {
            center: Vec2::new(0.5, 0.5),
            strength,
            softening: 0.04,
            max_accel: 0.12,
            radius: 0.045,
        }
    }

    #[test]
    fn attractor_pulls_toward_the_center() {
        let acceleration = well(0.002).acceleration(Vec2::new(0.7, 0.5));
        assert!(acceleration.x < 0.0);
        assert!(acceleration.y.abs() < 1e-6);
        assert!(acceleration.length() <= 0.12 + 1e-6);
    }

    #[test]
    fn repulsor_pushes_away_from_the_center() {
        let acceleration = well(-0.002).acceleration(Vec2::new(0.7, 0.5));
        assert!(acceleration.x > 0.0);
        assert!(acceleration.length() <= 0.12 + 1e-6);
    }

    #[test]
    fn inactive_field_has_no_acceleration() {
        assert_eq!(well(0.0).acceleration(Vec2::new(0.2, 0.8)), Vec2::ZERO);
        assert_eq!(well(0.0).kind(), 0.0);
    }
}
