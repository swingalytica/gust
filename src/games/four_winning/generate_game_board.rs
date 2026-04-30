use serde::{Serialize, Deserialize};
use super::consts;

#[derive(Serialize, Deserialize, Clone)]
pub struct GameBoardCell {
    pub col: String,
    pub row: i32,
    pub text: String,
    pub color: String,
    pub player_id: String
}

pub fn generate_game_board(min: i32, max: i32, measurement_unit: String) -> Vec<Vec<GameBoardCell>> {
    let mut board: Vec<Vec<GameBoardCell>> = Vec::new();

    let distance_cells: i32 = consts::FOUR_WINNING_DISTANCE_ROWS * consts::FOUR_WINNING_DISTANCE_COLS; // Distance cells: 8 rows * 5 cols = 40 cells
    let range_size: i32 = max - min;
    let increment_per_cell: f32 = range_size as f32 / distance_cells as f32;
    let total_cols: i32 = consts::FOUR_WINNING_DISTANCE_COLS + consts::FOUR_WINNING_LATERAL_DEVIATION_COLS;

    let mut cell_index: i32 = 0;

    for row in 0..consts::FOUR_WINNING_DISTANCE_ROWS {
        let mut current_row: Vec<GameBoardCell> = Vec::new();

        for col in 0..total_cols {
            if col == 0 || col == total_cols - 1 {
                // Leteral deviation cells
                let deviation_value: i32 = (row / consts::FOUR_WINNING_LATERAL_DEVIATION_COLS) + 1;

                current_row.push(GameBoardCell {
                    col: String::new(), 
                    row: 0, 
                    text: format!("{} {}", deviation_value, measurement_unit), 
                    color: String::from("transparent"), 
                    player_id: String::new() });
            } else {
                let a: i32 = f32::round(min as f32 + cell_index as f32 * increment_per_cell) as i32;
                let b: i32 = f32::round(min as f32 + (cell_index + 1) as f32 * increment_per_cell) as i32;

                let col_id: char = (b'a' + col as u8 - 1) as char;
                let row_id: i32 = row + 1;
                current_row.push(GameBoardCell { 
                    col: col_id.to_string(), 
                    row: row_id, 
                    text: format!("{}-{}", a, b), 
                    color: String::from("transparent"), 
                    player_id: String::from("") });
                cell_index += 1;
            }
        }
        board.push(current_row);
    }
    board
}