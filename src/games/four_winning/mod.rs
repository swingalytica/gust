use wasm_bindgen::prelude::*;
use serde::{Serialize, Deserialize};
use serde_wasm_bindgen::{to_value, from_value};

static FOUR_WINNING_DISTANCE_ROWS: i32 = 8;
static FOUR_WINNING_DISTANCE_COLS: i32 = 5;
static FOUR_WINNING_LATERAL_DEVIATION_COLS: i32 = 2;

#[derive(Serialize, Deserialize)]
pub struct GameBoardCell {
    pub text: String,
    pub color: String,
    pub player_id: String
}

fn generate_game_board(range: [i32; 2], measurement_unit: String) -> Vec<Vec<GameBoardCell>> {
    let mut board: Vec<Vec<GameBoardCell>> = Vec::new();
    let [min, max] = range;
    let distance_cells: i32 = FOUR_WINNING_DISTANCE_ROWS * FOUR_WINNING_DISTANCE_COLS;
    let range_size: i32 = max - min;
    let increment_per_cell: i32 = range_size / distance_cells;

    let mut cell_index: i32 = 0;

    for row in 0..FOUR_WINNING_DISTANCE_ROWS {
        let mut current_row: Vec<GameBoardCell> = Vec::new();

        for col in 0..FOUR_WINNING_DISTANCE_COLS {
            if col == 0 || col == 8 {
                // Leteral deviation cells
                let deviation_value: i32 = (row / FOUR_WINNING_LATERAL_DEVIATION_COLS) + 1;

                current_row.push(GameBoardCell { text: format!("{} {}", deviation_value, measurement_unit), color: String::from("transparent"), player_id: String::new() });
            } else {
                let a: i32 = { f32::round((min + cell_index * increment_per_cell) as f32) } as i32;
                let b: i32 = { f32::round((min + (cell_index + 1) * increment_per_cell) as f32) } as i32;
                current_row.push(GameBoardCell { text: format!("{}-{}", a, b), color: String::from("transparent"), player_id: String::from("") });
                cell_index += 1;
            }
        }
        board.push(current_row);
    }
    board
}

#[wasm_bindgen]
pub struct FourWinning {
}

#[wasm_bindgen]
impl FourWinning {
    #[wasm_bindgen(constructor)]
    pub fn new() -> FourWinning {
        FourWinning {}
    }

    #[wasm_bindgen]
    pub fn generate_game_board(&self, range: &JsValue, measurement_unit: String) -> JsValue {
        let range_vec: Vec<i32> = from_value(range.clone()).expect("range should be an array of two integers");
        assert!(range_vec.len() == 2, "range must have exactly two elements");
        let board = generate_game_board([range_vec[0], range_vec[1]], measurement_unit);
        to_value(&board).unwrap()
    }
}
