use std::{error::Error, fmt};

/*
 * max_value(s, α, β)
 *   if terminal(s) return U(s)
 *   v = -∞
 *   for c in next_states(s)
 *     v' = min_value(c, α, β)
 *     if v' > v, v = v'
 *     if v' ≥ β, return v
 *     if v' > α, α = v'
 *   return v
 *
 * min_value(s, α, β)
 *   if terminal(s) return U(s)
 *   v = ∞
 *   for c in next_states(s)
 *     v' = max_value(c, α, β)
 *     if v' < v, v = v'
 *     if v' ≤ α, return v
 *     if v' < β, β = v'
 *   return v
 */

#[derive(Debug)]
struct Dimensions {
    pub height: usize,
    pub width: usize,
}

#[derive(Debug)]
struct Game {
    state: Vec<Vec<Space>>,
    number_for_win: usize,
    dimensions: Dimensions,
}

#[derive(Debug, Clone, Copy)]
pub enum Space {
    Empty,
    Red,
    Yellow,
}

impl fmt::Display for Space {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Space::Empty => ".",
                Space::Red => "R",
                Space::Yellow => "Y",
            }
        )
    }
}

#[derive(Debug)]
pub enum GameError {
    InvalidGameDefinition,
}

impl Error for GameError {}

impl fmt::Display for GameError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Invalid game definition.")
    }
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

fn main() {
    println!("Hello, world!");
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
