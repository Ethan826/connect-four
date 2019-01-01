use std::{error::Error, fmt};

mod dimensions;
mod game;
mod space;

pub use self::dimensions::Dimensions;
pub use self::game::Game;
pub use self::space::Space;

#[derive(Debug)]
pub enum GameError {
    InvalidGameDefinition,
    InvalidGameState,
}

impl Error for GameError {}

impl fmt::Display for GameError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            GameError::InvalidGameDefinition => write!(f, "Invalid game definition."),
            GameError::InvalidGameState => write!(f, "Invalid game state."),
        }
    }
}
