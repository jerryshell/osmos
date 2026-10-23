use osmos_core::{Mass, MoteId, Vec2};

#[derive(Clone, Debug)]
pub struct Ejecta {
    pub id: u32,
    pub owner: MoteId,
    pub mass: Mass,
    pub position: Vec2,
    pub velocity: Vec2,
    pub age: u32,
    pub hue: f32,
    pub alive: bool,
}

impl Ejecta {
    pub fn new(
        id: u32,
        owner: MoteId,
        mass: Mass,
        position: Vec2,
        velocity: Vec2,
        hue: f32,
    ) -> Self {
        Self {
            id,
            owner,
            mass,
            position,
            velocity,
            age: 0,
            hue,
            alive: true,
        }
    }

    pub fn radius(&self) -> f32 {
        self.mass.radius()
    }
}
