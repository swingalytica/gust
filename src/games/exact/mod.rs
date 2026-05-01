mod consts;
mod generate_game_board;

use serde::{Deserialize, Serialize};
use serde_wasm_bindgen::{to_value, Error};
use wasm_bindgen::prelude::*;
use generate_game_board::{rows, ExactGameBoardCell};

#[wasm_bindgen(typescript_custom_section)]
const TS_TYPES: &'static str = r#"
export interface Player {
    id: string;
    pos: number;
    name: string;
    color: string;
    shots: number;
    points: number;
}
"#;

#[wasm_bindgen]extern "C" {
    #[wasm_bindgen(typescript_type = "Player")]
    pub type PlayerJS;
}

#[derive(Deserialize, Serialize)]
pub struct Player {
    pub id: String,
    pub pos: i32,
    pub name: String,
    pub color: String,
    pub shots: i32,
    pub points: i64
}

#[wasm_bindgen]
pub struct Exact {
    players: Vec<Player>,
}

#[wasm_bindgen]
impl Exact {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Exact {
        Exact {
            players: Vec::new(),
        }
    }

    #[wasm_bindgen]
    pub fn get_game_board(&self) -> JsValue {
        let game_board: Vec<ExactGameBoardCell> = rows();
        to_value(&game_board).unwrap_or_else(|e: Error| JsValue::from_str(&e.to_string()))
    }

    #[wasm_bindgen]
    pub fn add_player(&mut self, player: PlayerJS) -> Result<(), JsValue> {
        let player: Player = serde_wasm_bindgen::from_value(player.into())
            .map_err(|e: Error| JsValue::from_str(&e.to_string()))?;
        self.players.push(player);
        Ok(())
    }
}