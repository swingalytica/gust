mod generate_game_board;
mod consts;

use serde::{Deserialize, Serialize};
use serde_wasm_bindgen::to_value;
use wasm_bindgen::prelude::*;
use generate_game_board::generate_game_board;
use serde_wasm_bindgen::Error;

#[wasm_bindgen(typescript_custom_section)]
const TS_TYPES: &'static str = r#"
export interface Player {
    id: string;
    pos: number;
    name: string;
    color: string;
    data: string[];
}

export interface PlayerUpdate {
    pos?: number;
    name?: string;
    color?: string;
    data?: string[];
}
"#;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(typescript_type = "Player")]
    pub type PlayerJs;

    #[wasm_bindgen(typescript_type = "PlayerUpdate")]
    pub type PlayerUpdateJs;
}

#[derive(Deserialize, Serialize)]
pub struct Player {
    pub id: String,
    pub pos: i32,
    pub name: String,
    pub color: String,
    pub data: Vec<String>,
}

#[derive(Deserialize)]
pub struct PlayerUpdate {
    pub pos: Option<i32>,
    pub name: Option<String>,
    pub color: Option<String>,
    pub data: Option<Vec<String>>,
}

#[wasm_bindgen]
pub struct FourWinning {
    players: Vec<Player>,
    board: Vec<Vec<generate_game_board::GameBoardCell>>,
    current_player_id: String,
    game_ended: bool,
    start_time: String,
    unit: String,
}

#[wasm_bindgen]
impl FourWinning {
    #[wasm_bindgen(constructor)]
    pub fn new() -> FourWinning {
        FourWinning {
            players: Vec::new(),
            board: Vec::new(),
            current_player_id: String::new(),
            game_ended: false,
            start_time: String::new(),
            unit: String::new(),
        }
    }

    #[wasm_bindgen]
    pub fn generate_game_board(&mut self, min: i32, max: i32, measurement_unit: String) -> Result<JsValue, JsValue> {
        self.unit = measurement_unit.clone();
        let board: Vec<Vec<generate_game_board::GameBoardCell>> = generate_game_board(min, max, measurement_unit);
        self.board = board.clone();
        to_value(&board).map_err(|e: Error| JsValue::from_str(&e.to_string()))
    }

    #[wasm_bindgen]
    pub fn add_player(&mut self, player: PlayerJs) -> Result<(), JsValue> {
        let player: Player = serde_wasm_bindgen::from_value(player.into())
            .map_err(|e: Error| JsValue::from_str(&e.to_string()))?;
        self.players.push(player);
        Ok(())
    }

    pub fn update_player(&mut self, id: String, update: PlayerUpdateJs) -> Result<(), JsValue> {
        let update: PlayerUpdate = serde_wasm_bindgen::from_value(update.into())
            .map_err(|e: Error| JsValue::from_str(&e.to_string()))?;
    
        if let Some(p) = self.players.iter_mut().find(|p: &&mut Player| p.id == id) {
            if let Some(pos) = update.pos { p.pos = pos; }
            if let Some(name) = update.name { p.name = name; }
            if let Some(color) = update.color { p.color = color; }
            if let Some(data) = update.data { p.data = data; }
        }

        Ok(())
    }

    pub fn remove_player(&mut self, id: String) {
        self.players.retain(|p: &Player| p.id != id);
    }

    pub fn get_players(&self) -> Result<JsValue, JsValue> {
        to_value(&self.players).map_err(|e: Error| JsValue::from_str(&e.to_string()))
    }

    pub fn start(&mut self, start_time: String) {
        self.start_time = start_time;
        self.current_player_id = self.players[0].id.clone();
        self.game_ended = false;
    }

    pub fn click_cell(&mut self, coord: String) -> Result<JsValue, JsValue> {
        let cell: &generate_game_board::GameBoardCell = self.board.iter()
            .flatten()
            .find(|c: &&generate_game_board::GameBoardCell| c.col == coord)
            .ok_or_else(|| JsValue::from_str("Cell not found"))?;

        if self.game_ended || !cell.player_id.is_empty() || cell.text.contains(&self.unit) {
            return Ok(JsValue::null());
        }


        Ok(JsValue::null())
    }

    #[wasm_bindgen]
    pub fn handle_click(&mut self, coord: String) -> Result<JsValue, JsValue> {
        let current_player_id: String = self.current_player_id.clone();

        let current_player_color: String = self.players.iter()
            .find(|p: &&Player| p.id == current_player_id)
            .ok_or_else(|| JsValue::from_str("Current player not found"))?
            .color.clone();

        let mut found: bool = false;
        for row in self.board.iter_mut() {
            for cell in row.iter_mut() {
                let cell_coord = format!("{}{}", cell.col, cell.row);
                if cell_coord == coord {
                    found = true;
                    if cell.player_id.is_empty() {
                        cell.player_id = current_player_id.clone();
                        cell.color = current_player_color.clone();
                        if let Some(player) = self.players.iter_mut().find(|p: &&mut Player| p.id == current_player_id) {
                            player.data.push(cell.text.clone());
                        }
                    }
                    break;
                }
            }
            if found { break; }
        }

        Ok(JsValue::null())
    }
}
