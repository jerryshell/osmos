use js_sys::{Float32Array, Object, Reflect, Uint32Array, Uint8Array};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct Engine {
    inner: osmos_evo::Engine,
}

#[wasm_bindgen]
impl Engine {
    #[wasm_bindgen(constructor)]
    pub fn new(seed: u32, scenario: u32) -> Self {
        Self {
            inner: osmos_evo::Engine::with_scenario(
                seed as u64,
                osmos_evo::Scenario::from_id(scenario),
            ),
        }
    }

    pub fn step(&mut self, steps: u32) {
        self.inner.step(steps);
    }

    pub fn reset(&mut self, seed: u32) {
        self.inner.reset(seed as u64);
    }

    pub fn inspect(&self, id: u32) -> JsValue {
        let Some(detail) = self.inner.inspect(id) else {
            return JsValue::NULL;
        };
        let result = Object::new();
        set(&result, "id", JsValue::from_f64(detail.id as f64));
        set(&result, "mass", JsValue::from_f64(detail.mass as f64));
        set(
            &result,
            "sensor",
            Float32Array::from(detail.sensor.as_slice()).into(),
        );
        set(
            &result,
            "thrustX",
            JsValue::from_f64(detail.thrust[0] as f64),
        );
        set(
            &result,
            "thrustY",
            JsValue::from_f64(detail.thrust[1] as f64),
        );
        set(
            &result,
            "absorbedPellets",
            JsValue::from_f64(detail.absorbed_pellets as f64),
        );
        set(
            &result,
            "absorbedMotes",
            JsValue::from_f64(detail.absorbed_motes as f64),
        );
        set(
            &result,
            "ejectedMass",
            JsValue::from_f64(detail.ejected_mass as f64),
        );
        result.into()
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
            "moteFlags",
            Uint8Array::from(snapshot.mote_flags.as_slice()).into(),
        );
        set(
            &result,
            "pelletState",
            Float32Array::from(snapshot.pellet_state.as_slice()).into(),
        );
        set(
            &result,
            "ejectaState",
            Float32Array::from(snapshot.ejecta_state.as_slice()).into(),
        );
        set(
            &result,
            "wellState",
            Float32Array::from(snapshot.well_state.as_slice()).into(),
        );

        let stats = Object::new();
        set(
            &stats,
            "generation",
            JsValue::from_f64(snapshot.stats.generation as f64),
        );
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
            "ejecta",
            JsValue::from_f64(snapshot.stats.ejecta as f64),
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

#[wasm_bindgen(js_name = newEngineWithChampion)]
pub fn new_engine_with_champion(seed: u32, scenario: u32, champion: &[f32]) -> Engine {
    Engine {
        inner: osmos_evo::Engine::with_scenario_champion(
            seed as u64,
            osmos_evo::Scenario::from_id(scenario),
            osmos_sim::SimConfig::default(),
            champion.to_vec(),
        ),
    }
}

#[wasm_bindgen(js_name = genomeLength)]
pub fn genome_length() -> usize {
    osmos_evo::GENOME_LENGTH
}
