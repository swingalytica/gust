use crate::shot::Shot;

#[derive(Clone, Debug, PartialEq)]
pub enum GameState {
    WaitingForPlayers,
    InProgress,
    Finished { winner: Option<u8> },
}

#[derive(Clone, Debug)]
pub struct ShotResult {
    pub valid: bool,
    pub state: GameState,
}

pub trait Game {
    fn add_shot(&mut self, player_id: u8, shot: Shot) -> ShotResult;
    fn skip_turn(&mut self, player_id: u8);
    fn get_state(&self) -> GameState;
    fn current_player(&self) -> u8;
    fn is_finished(&self) -> bool;
    fn winner(&self) -> Option<u8>;
}