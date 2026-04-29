use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(Clone, Copy, Debug)]
pub struct Shot {
    pub distance: f64,
    pub lateral_deviation: f64,
}

#[wasm_bindgen]
impl Shot {
    #[wasm_bindgen(constructor)]
    pub fn new(distance: f64, lateral_deviation: f64) -> Shot {
        Shot { distance, lateral_deviation }
    }
}