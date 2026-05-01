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

export interface PlayerUpdate {
    pos?: number;
    name?: string;
    color?: string;
    shots?: number;
    points?: number;
}
"#;

#[wasm_bindgen]extern "C" {
    #[wasm_bindgen(typescript_type = "Player")]
    pub type PlayerJS;
    #[wasm_bindgen(typescript_type = "PlayerUpdate")]
    pub type PlayerUpdateJS;
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

#[derive(Deserialize)]
pub struct PlayerUpdate {
    pub pos: Option<i32>,
    pub name: Option<String>,
    pub color: Option<String>,
    pub shots: Option<i32>,
    pub points: Option<i64>
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

    #[wasm_bindgen]
    pub fn update_player(&mut self, id: String, update: PlayerUpdateJS) -> Result<(), JsValue> {
        let update: PlayerUpdate = serde_wasm_bindgen::from_value(update.into())
            .map_err(|e: Error| JsValue::from_str(&e.to_string()))?;

         if let Some(p) = self.players.iter_mut().find(|p: &&mut Player| p.id == id) {
            if let Some(pos) = update.pos { p.pos = pos; }
            if let Some(name) = update.name { p.name = name; }
            if let Some(color) = update.color { p.color = color; }
            if let Some(shots) = update.shots { p.shots = shots; }
            if let Some(points) = update.points { p.points = points; }
        }

        Ok(())
    }

    #[wasm_bindgen]
    pub fn remove_player(&mut self, id: String) {
        self.players.retain(|p: &Player| p.id != id);
    }

    #[wasm_bindgen]
    pub fn get_players(&self) -> Result<JsValue, JsValue> {
        to_value(&self.players).map_err(|e: Error| JsValue::from_str(&e.to_string()))
    }
}