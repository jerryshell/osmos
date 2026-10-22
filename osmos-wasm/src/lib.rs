use js_sys::{Float32Array, Object, Reflect, Uint32Array, Uint8Array};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct Engine {
    inner: osmos_evo::Engine,
}

#[wasm_bindgen]
impl Engine {
    #[wasm_bindgen(constructor)]
    pub fn new(seed: u32) -> Self {
        Self {
            inner: osmos_evo::Engine::new(seed as u64),
        }
    }

    pub fn step(&mut self, steps: u32) {
        self.inner.step(steps);
    }

    pub fn reset(&mut self, seed: u32) {
        self.inner.reset(seed as u64);
    }

    pub fn snapshot(&self) -> JsValue {
        let snapshot = self.inner.snapshot();
        let result = Object::new();

        set(&result, "tick", JsValue::from_f64(snapshot.tick as f64));
        set(
            &result,
            "moteIds",
            Uint32Array::from(snapshot.mote_ids.as_slice()).into(),
        );
        set(
            &result,
            "moteState",
            Float32Array::from(snapshot.mote_state.as_slice()).into(),
        );
        set(
            &result,
            "moteRole",
            Uint8Array::from(snapshot.mote_role.as_slice()).into(),
        );
        set(
            &result,
            "moteFlags",
            Uint8Array::from(snapshot.mote_flags.as_slice()).into(),
        );
        set(
            &result,
            "pelletState",
            Float32Array::from(snapshot.pellet_state.as_slice()).into(),
        );

        let stats = Object::new();
        set(
            &stats,
            "tick",
            JsValue::from_f64(snapshot.stats.tick as f64),
        );
        set(
            &stats,
            "alive",
            JsValue::from_f64(snapshot.stats.alive as f64),
        );
        set(
            &stats,
            "pellets",
            JsValue::from_f64(snapshot.stats.pellets as f64),
        );
        set(
            &stats,
            "bestMass",
            JsValue::from_f64(snapshot.stats.best_mass as f64),
        );
        set(
            &stats,
            "totalMass",
            JsValue::from_f64(snapshot.stats.total_mass as f64),
        );
        set(&result, "stats", stats.into());

        result.into()
    }
}

fn set(object: &Object, key: &str, value: JsValue) {
    Reflect::set(object, &JsValue::from_str(key), &value).expect("snapshot object is writable");
}
