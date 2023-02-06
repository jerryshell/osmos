#[wasm_bindgen::prelude::wasm_bindgen]
pub fn hello() -> String {
    "devzero".to_string()
}

#[derive(Default)]
#[wasm_bindgen::prelude::wasm_bindgen]
pub struct Simulator {
    simulator: osmos_sim::simulator::Simulator,
}

#[wasm_bindgen::prelude::wasm_bindgen]
impl Simulator {
    #[wasm_bindgen::prelude::wasm_bindgen(constructor)]
    pub fn new() -> Self {
        Default::default()
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

    pub fn step(&mut self) {
        self.simulator.step()
    }
}

#[derive(serde::Deserialize, serde::Serialize)]
pub struct Object {
    pub x: f32,
    pub y: f32,
    pub energy: usize,
    pub network_output: Vec<f32>,
}

impl From<&osmos_sim::object::Object> for Object {
    fn from(object: &osmos_sim::object::Object) -> Self {
        Self {
            x: object.cell.position.x,
            y: object.cell.position.y,
            energy: object.cell.energy,
            network_output: object.network_output.clone(),
        }
    }
}
