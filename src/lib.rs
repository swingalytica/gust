pub mod shot;
pub mod game;
pub mod games {
    pub mod four_winning;
}
 
pub use shot::Shot;
pub use game::{Game, GameState, ShotResult};
pub use games::four_winning::FourWinning;