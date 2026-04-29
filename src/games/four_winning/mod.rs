use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct FourWinning {
    
}

#[wasm_bindgen]
impl FourWinning {
    #[wasm_bindgen(constructor)]
    pub fn new() -> FourWinning {
        FourWinning {
            // Initialize fields here if needed
        }
    }
}