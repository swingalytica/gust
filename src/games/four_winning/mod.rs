use wasm_bindgen::prelude::*;
use crate::shot::Shot;
use crate::game::GameState;

pub const COLS: usize = 7;
pub const ROWS: usize = 8;
pub const WIN_LENGTH: usize = 4;

/// A distance zone mapped to a column on the grid.
#[wasm_bindgen]
#[derive(Clone, Copy, Debug)]
pub struct Zone {
    pub min_distance: f64,
    pub max_distance: f64,
}

#[wasm_bindgen]
impl Zone {
    #[wasm_bindgen(constructor)]
    pub fn new(min_distance: f64, max_distance: f64) -> Zone {
        Zone { min_distance, max_distance }
    }
}

/// Default zones: 7 columns spanning 50m–225m in 25m steps.
pub fn default_zones() -> Vec<Zone> {
    vec![
        Zone::new(50.0, 75.0),
        Zone::new(75.0, 100.0),
        Zone::new(100.0, 125.0),
        Zone::new(125.0, 150.0),
        Zone::new(150.0, 175.0),
        Zone::new(175.0, 200.0),
        Zone::new(200.0, 225.0),
    ]
}

#[wasm_bindgen]
pub struct FourWinning {
    zones: Vec<Zone>,
    grid: [[Option<u8>; COLS]; ROWS],
    player_count: u8,
    current_player: u8,
    state: GameState,
}

#[wasm_bindgen]
impl FourWinning {
    /// Create a game with default zones and 2 players.
    #[wasm_bindgen(constructor)]
    pub fn new() -> FourWinning {
        FourWinning::with_config(default_zones(), 2)
    }

    /// Create a game with custom zones and player count.
    pub fn with_config(zones: Vec<Zone>, player_count: u8) -> FourWinning {
        FourWinning {
            zones,
            grid: [[None; COLS]; ROWS],
            player_count,
            current_player: 0,
            state: GameState::InProgress,
        }
    }

    /// Map a shot's distance to a column index, if within any zone.
    fn shot_to_col(&self, shot: Shot) -> Option<usize> {
        self.zones.iter().position(|z| {
            shot.distance >= z.min_distance && shot.distance < z.max_distance
        })
    }

    /// Drop a piece in the lowest available row in a column.
    fn drop_piece(&mut self, col: usize, player_id: u8) -> bool {
        for row in (0..ROWS).rev() {
            if self.grid[row][col].is_none() {
                self.grid[row][col] = Some(player_id);
                return true;
            }
        }
        false // Column is full
    }

    /// Check if the board has a winning line of WIN_LENGTH for any player.
    fn check_winner(&self) -> Option<u8> {
        let directions: &[(isize, isize)] = &[(0, 1), (1, 0), (1, 1), (1, -1)];

        for row in 0..ROWS {
            for col in 0..COLS {
                let Some(player) = self.grid[row][col] else { continue };

                for &(dr, dc) in directions {
                    let mut count = 1;
                    for step in 1..WIN_LENGTH {
                        let r = row as isize + dr * step as isize;
                        let c = col as isize + dc * step as isize;
                        if r < 0 || r >= ROWS as isize || c < 0 || c >= COLS as isize {
                            break;
                        }
                        if self.grid[r as usize][c as usize] != Some(player) {
                            break;
                        }
                        count += 1;
                    }
                    if count == WIN_LENGTH {
                        return Some(player);
                    }
                }
            }
        }
        None
    }

    /// Check if the board is full (draw).
    fn is_full(&self) -> bool {
        self.grid[0].iter().all(|cell| cell.is_some())
    }

    fn advance_turn(&mut self) {
        self.current_player = (self.current_player + 1) % self.player_count;
    }

    /// Get the current player index.
    pub fn current_player(&self) -> u8 {
        self.current_player
    }

    /// Returns true if the game is finished.
    pub fn is_finished(&self) -> bool {
        matches!(self.state, GameState::Finished { .. })
    }

    /// Returns the winner's player index, or u8::MAX if draw or not finished.
    pub fn winner(&self) -> u8 {
        match self.state {
            GameState::Finished { winner: Some(p) } => p,
            _ => u8::MAX,
        }
    }

    /// Add a shot for the current player.
    pub fn add_shot(&mut self, player_id: u8, shot: Shot) -> bool {
        if self.is_finished() || player_id != self.current_player {
            return false;
        }

        let Some(col) = self.shot_to_col(shot) else {
            // Out of bounds — not a valid shot, turn is NOT consumed
            return false;
        };

        if !self.drop_piece(col, player_id) {
            // Column full — not valid
            return false;
        }

        if let Some(winner) = self.check_winner() {
            self.state = GameState::Finished { winner: Some(winner) };
        } else if self.is_full() {
            self.state = GameState::Finished { winner: None };
        } else {
            self.advance_turn();
        }

        true
    }

    /// Skip the current player's turn (e.g. shot was out of all zones).
    pub fn skip_turn(&mut self, player_id: u8) {
        if self.is_finished() || player_id != self.current_player {
            return;
        }
        self.advance_turn();
    }

    /// Get a cell value: Some(player_id) or None.
    /// Returns u8::MAX for empty cells (WASM-friendly).
    pub fn get_cell(&self, row: usize, col: usize) -> u8 {
        match self.grid[row][col] {
            Some(p) => p,
            None => u8::MAX,
        }
    }
}