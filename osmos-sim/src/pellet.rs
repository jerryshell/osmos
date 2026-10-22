use osmos_core::{Mass, PelletId, Vec2};

#[derive(Clone, Debug)]
pub struct Pellet {
    pub id: PelletId,
    pub mass: Mass,
    pub position: Vec2,
    pub alive: bool,
}

impl Pellet {
    pub fn new(id: PelletId, mass: Mass, position: Vec2) -> Self {
        Self {
            id,
            mass,
            position,
            alive: true,
        }
    }

    pub fn radius(&self) -> f32 {
        self.mass.radius()
    }
}
