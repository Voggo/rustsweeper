pub mod board;
pub mod game;
pub mod menu;
pub mod tui;

// You can also re-export important types for easier access
pub use board::Board;
pub use game::{GameConfig, GameState};
