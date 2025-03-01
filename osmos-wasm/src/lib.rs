#[wasm_bindgen::prelude::wasm_bindgen]
#[derive(Default)]
pub struct Simulator {
    simulator: osmos_sim::simulator::Simulator,
}

#[wasm_bindgen::prelude::wasm_bindgen]
impl Simulator {
    #[wasm_bindgen::prelude::wasm_bindgen(constructor)]
    pub fn new() -> Self {
        Self::default()
    }

    #[wasm_bindgen::prelude::wasm_bindgen(js_name = getObjectList)]
    pub fn get_object_list(&self) -> wasm_bindgen::JsValue {
        let object_list = self
            .simulator
            .object_list
            .iter()
            .map(Object::from)
            .collect::<Vec<Object>>();
        serde_wasm_bindgen::to_value(&object_list).unwrap()
    }

    #[wasm_bindgen::prelude::wasm_bindgen(js_name = getStepCount)]
    pub fn get_step_count(&self) -> usize {
        self.simulator.step_count
    }

    #[wasm_bindgen::prelude::wasm_bindgen(js_name = getEpochCount)]
    pub fn get_epoch_count(&self) -> usize {
        self.simulator.epoch_count
    }

    pub fn step(&mut self) {
        self.simulator.step()
    }
}

#[derive(serde::Deserialize, serde::Serialize)]
pub struct Object {
    pub id: usize,
    pub x: f32,
    pub y: f32,
    pub energy: usize,
}

impl From<&osmos_sim::object::Object> for Object {
    fn from(sim_object: &osmos_sim::object::Object) -> Self {
        Self {
            id: sim_object.id,
            x: sim_object.cell.position.x,
            y: sim_object.cell.position.y,
            energy: sim_object.cell.energy,
        }
    }
}
