//! BumbleBees - A Space Invaders Clone
//!
//! This is a classic arcade-style space shooter game built with Rust and ggez.
//! Features include:
//! - Progressive difficulty with wave-based gameplay
//! - Parallax scrolling background
//! - Highscore tracking with persistent storage
//! - Sound effects and background music

pub mod constants;
pub mod entities;
pub mod game_state;
pub mod highscore;
pub mod logger;
pub mod rendering;
pub mod systems;

pub use constants::*;
pub use entities::*;
pub use game_state::{GameState, MainState};
pub use highscore::{HighscoreEntry, HighscoreManager};
pub use logger::init as init_logger;
pub use rendering::draw_game;
pub use systems::*;
