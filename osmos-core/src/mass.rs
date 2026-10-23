pub const RADIUS_SCALE: f32 = 0.012;

#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
pub struct Mass(f32);

impl Mass {
    pub fn new(value: f32) -> Self {
        assert!(value.is_finite(), "mass must be finite");
        assert!(value > 0.0, "mass must be positive");
        Self(value)
    }

    pub const fn get(self) -> f32 {
        self.0
    }

    pub fn radius(self) -> f32 {
        RADIUS_SCALE * self.0.sqrt()
    }
}

impl std::ops::Add for Mass {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        Self::new(self.0 + rhs.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn radius_comes_from_mass() {
        let one = Mass::new(1.0);
        let four = Mass::new(4.0);

        assert!((one.radius() - RADIUS_SCALE).abs() < 1e-6);
        assert!((four.radius() - RADIUS_SCALE * 2.0).abs() < 1e-6);
    }
}
