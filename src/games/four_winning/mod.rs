mod generate_game_board;

use serde_wasm_bindgen::to_value;
use wasm_bindgen::prelude::*;
use generate_game_board::generate_game_board;

#[wasm_bindgen]
pub struct FourWinning {}

#[wasm_bindgen]
impl FourWinning {
    #[wasm_bindgen(constructor)]
    pub fn new() -> FourWinning {
        FourWinning {}
    }

    #[wasm_bindgen]
    pub fn generate_game_board(&self, min: i32, max: i32, measurement_unit: String) -> Result<JsValue, JsValue> {
        let board: Vec<Vec<generate_game_board::GameBoardCell>> = generate_game_board(min, max, measurement_unit);
        to_value(&board).map_err(|e| JsValue::from_str(&e.to_string()))
    }
}
