mod consts;
mod generate_game_board;

use serde_wasm_bindgen::{to_value, Error};
use wasm_bindgen::prelude::*;
use generate_game_board::{rows, ExactGameBoardCell};

#[wasm_bindgen]
pub struct Exact {
    
}

#[wasm_bindgen]
impl Exact {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Exact {
        Exact {
            
        }
    }

    #[wasm_bindgen]
    pub fn get_game_board(&self) -> JsValue {
        let game_board: Vec<ExactGameBoardCell> = rows();
        to_value(&game_board).unwrap_or_else(|e: Error| JsValue::from_str(&e.to_string()))
    }
}