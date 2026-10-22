pub const MOTE_STATE_STRIDE: usize = 7;
pub const PELLET_STATE_STRIDE: usize = 4;
pub const EJECTA_STATE_STRIDE: usize = 5;

pub const FLAG_ALIVE: u8 = 1;
pub const FLAG_TARGET_REACHED: u8 = 2;

#[derive(Clone, Debug, Default)]
pub struct Stats {
    pub generation: u32,
    pub tick: u64,
    pub alive: u32,
    pub pellets: u32,
    pub ejecta: u32,
    pub best_mass: f32,
    pub total_mass: f32,
}

#[derive(Clone, Debug, Default)]
pub struct Snapshot {
    pub tick: u64,
    pub mote_ids: Vec<u32>,
    pub mote_state: Vec<f32>,
    pub mote_role: Vec<u8>,
    pub mote_flags: Vec<u8>,
    pub pellet_state: Vec<f32>,
    pub ejecta_state: Vec<f32>,
    pub stats: Stats,
}

impl Snapshot {
    pub fn mote_count(&self) -> usize {
        self.mote_ids.len()
    }

    pub fn pellet_count(&self) -> usize {
        self.pellet_state.len() / PELLET_STATE_STRIDE
    }
}
