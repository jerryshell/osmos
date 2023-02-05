#[wasm_bindgen::prelude::wasm_bindgen]
pub fn hello() -> String {
    "devzero".to_string()
}

#[wasm_bindgen::prelude::wasm_bindgen]
pub struct World {
    world: osmos_core::world::World,
}

impl Default for World {
    fn default() -> Self {
        Self {
            world: osmos_core::world::World::random(),
        }
    }
}

#[wasm_bindgen::prelude::wasm_bindgen]
impl World {
    #[wasm_bindgen::prelude::wasm_bindgen(constructor)]
    pub fn new() -> Self {
        Default::default()
    }

    pub fn cell_list(&self) -> wasm_bindgen::JsValue {
        let cell_list = self
            .world
            .cell_list
            .iter()
            .map(Cell::from)
            .collect::<Vec<Cell>>();
        serde_wasm_bindgen::to_value(&cell_list).unwrap()
    }

    pub fn step(&mut self) {
        self.world.step()
    }
}

#[derive(serde::Deserialize, serde::Serialize)]
pub struct Cell {
    pub x: f32,
    pub y: f32,
    pub energy: usize,
}

impl From<&osmos_core::cell::Cell> for Cell {
    fn from(value: &osmos_core::cell::Cell) -> Self {
        Self {
            x: value.position.x,
            y: value.position.y,
            energy: value.energy,
        }
    }
}
