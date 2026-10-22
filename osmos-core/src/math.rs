#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Vec2 {
    pub x: f32,
    pub y: f32,
}

impl Vec2 {
    pub const ZERO: Self = Self { x: 0.0, y: 0.0 };

    pub const fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }

    pub fn length_squared(self) -> f32 {
        self.x * self.x + self.y * self.y
    }

    pub fn length(self) -> f32 {
        self.length_squared().sqrt()
    }

    pub fn dot(self, other: Self) -> f32 {
        self.x * other.x + self.y * other.y
    }

    pub fn distance_squared(self, other: Self) -> f32 {
        (self - other).length_squared()
    }

    pub fn normalize_or_zero(self) -> Self {
        let length = self.length();
        if length > 1e-6 {
            self * (1.0 / length)
        } else {
            Self::ZERO
        }
    }

    pub fn clamp_length(self, max_length: f32) -> Self {
        let length = self.length();
        if length > max_length {
            self * (max_length / length)
        } else {
            self
        }
    }
}

/// Wrap a position into the unit torus.
pub fn wrap_position(position: Vec2) -> Vec2 {
    Vec2::new(wrap_coordinate(position.x), wrap_coordinate(position.y))
}

fn wrap_coordinate(value: f32) -> f32 {
    if value < 0.0 {
        value + 1.0
    } else if value >= 1.0 {
        value - 1.0
    } else {
        value
    }
}

/// Return the shortest signed delta on the unit torus.
pub fn wrap_delta(delta: Vec2) -> Vec2 {
    Vec2::new(wrap_component(delta.x), wrap_component(delta.y))
}

fn wrap_component(value: f32) -> f32 {
    if value > 0.5 {
        value - 1.0
    } else if value < -0.5 {
        value + 1.0
    } else {
        value
    }
}

impl std::ops::Add for Vec2 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        Self::new(self.x + rhs.x, self.y + rhs.y)
    }
}

impl std::ops::AddAssign for Vec2 {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl std::ops::Sub for Vec2 {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self {
        Self::new(self.x - rhs.x, self.y - rhs.y)
    }
}

impl std::ops::SubAssign for Vec2 {
    fn sub_assign(&mut self, rhs: Self) {
        self.x -= rhs.x;
        self.y -= rhs.y;
    }
}

impl std::ops::Mul<f32> for Vec2 {
    type Output = Self;

    fn mul(self, rhs: f32) -> Self {
        Self::new(self.x * rhs, self.y * rhs)
    }
}

impl std::ops::MulAssign<f32> for Vec2 {
    fn mul_assign(&mut self, rhs: f32) {
        self.x *= rhs;
        self.y *= rhs;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn vector_math_is_consistent() {
        let a = Vec2::new(3.0, 4.0);
        assert!((a.length() - 5.0).abs() < 1e-6);
        assert!((a.dot(Vec2::new(1.0, 0.0)) - 3.0).abs() < 1e-6);
        assert_eq!(Vec2::ZERO.normalize_or_zero(), Vec2::ZERO);
        assert!((a.normalize_or_zero().length() - 1.0).abs() < 1e-6);
    }

    #[test]
    fn positions_and_deltas_wrap() {
        assert_eq!(wrap_position(Vec2::new(1.25, -0.25)), Vec2::new(0.25, 0.75));
        let delta = wrap_delta(Vec2::new(0.75, -0.6));
        assert!((delta.x + 0.25).abs() < 1e-6);
        assert!((delta.y - 0.4).abs() < 1e-6);
        assert_eq!(wrap_delta(Vec2::new(-0.5, 0.5)), Vec2::new(-0.5, 0.5));
    }
}
