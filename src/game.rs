use super::{Dimensions, GameError, Space};
use std::fmt;

#[derive(Debug)]
pub struct Game {
    state: Vec<Vec<Space>>,
    number_for_win: usize,
    dimensions: Dimensions,
}

impl Game {
    pub fn new(state: Vec<Vec<Space>>, number_for_win: usize) -> Result<Self, GameError> {
        let dimensions = Dimensions {
            height: state.len(),
            width: state[0].len(),
        };

        if dimensions.width < number_for_win && dimensions.height < number_for_win {
            Err(GameError::InvalidGameDefinition)
        } else {
            Ok(Game {
                number_for_win,
                dimensions,
                state,
            })
        }
    }

    pub fn new_empty(dimensions: Dimensions, number_for_win: usize) -> Result<Self, GameError> {
        Game::new(
            vec![vec![Space::Empty; dimensions.width]; dimensions.height],
            number_for_win,
        )
    }
}

impl fmt::Display for Game {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for row in &self.state {
            write!(f, "\n")?;
            for (i, space) in row.iter().enumerate() {
                if i > 0 && i < self.dimensions.width {
                    write!(f, " ")?;
                }
                write!(f, "{}", space)?;
            }
        }
        write!(f, "\n")?;
        Ok(())
    }
}

// =================================================================================================
// Tests
// =================================================================================================

#[test]
fn test_new_empty_game_valid() {
    assert!(Game::new_empty(
        Dimensions {
            width: 7,
            height: 6,
        },
        4,
    )
    .is_ok());
}

#[test]
fn test_new_empty_game_invalid() {
    assert!(Game::new_empty(
        Dimensions {
            width: 3,
            height: 3,
        },
        4,
    )
    .is_err());
}
