use super::{Dimensions, GameError, Space};
use std::fmt;

#[derive(Debug)]
pub struct Game {
    state: Vec<Vec<Space>>,
    number_for_win: usize,
    dimensions: Dimensions,
}

#[derive(Debug, Copy, Clone)]
pub enum Player {
    AI,
    Opponent,
}

#[derive(Debug, Copy, Clone)]
struct RunData {
    ai: usize,
    opponent: usize,
}

struct LongestRuns {
    horizontal: RunData,
    vertical: RunData,
    diagonal: RunData,
}

impl LongestRuns {
    fn winner(&self, number_for_win: usize) -> Result<Option<Player>, GameError> {
        let ai_wins = self.horizontal.ai >= number_for_win
            || self.vertical.ai >= number_for_win
            || self.diagonal.ai >= number_for_win;

        let opponent_wins = self.horizontal.opponent >= number_for_win
            || self.vertical.opponent >= number_for_win
            || self.diagonal.opponent >= number_for_win;

        match (ai_wins, opponent_wins) {
            (false, false) => Ok(None),
            (true, false) => Ok(Some(Player::AI)),
            (false, true) => Ok(Some(Player::Opponent)),
            (true, true) => Err(GameError::InvalidGameState),
        }
    }
}

impl Game {
    /// Return a new `Game` given a state for that game and the number of pieces needed to
    /// constitute a win (the "four" part of connect four).
    ///
    /// # Errors
    ///
    /// Returns an error if the `number_for_win` argument is larger than any dimension.
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

    /// Return a new empty `Game` given dimensions for that game and the number of pieces needed to
    /// constitute a win (the "four" part of connect four).
    ///
    /// # Errors
    ///
    /// Returns an error if the `number_for_win` argument is larger than any dimension.
    pub fn new_empty(dimensions: Dimensions, number_for_win: usize) -> Result<Self, GameError> {
        Game::new(
            vec![vec![Space::Empty; dimensions.width]; dimensions.height],
            number_for_win,
        )
    }

    /// Fulfills the roles of determining if we are at a leaf node in the game graph, and if so
    /// what is the value of the position.
    ///
    /// The leaf node functionality is represented by whether the return value is a `None` or
    /// `Some` value. We could be at a leaf node either because the game is won, lost, or drawn, or
    /// because the `depth` argument is 0, meaning we are at a leaf node by fiat.
    ///
    /// If the game is over, the value of the position is evaluated based on a winning state
    /// (maximum `isize` value), a losing state (minimum `isize` value), or a draw (0). If the
    /// position is not a game-over state, apply a heuristic, currently based on the AI's number of
    /// pieces in a row relative to the `number_for_win` argument and the opponent's status
    /// regarding the same.
    pub fn evaluate(&self, depth: usize) -> Result<Option<isize>, GameError> {
        let longest_runs = self.find_longest_runs();
        let winner = self.winner(&longest_runs)?;

        match winner {
            Some(Player::AI) => Ok(Some(isize::max_value())),
            Some(Player::Opponent) => Ok(Some(isize::min_value())),
            None => {
                if depth == 0 {
                    Ok(Some(self.score_incomplete_game(&longest_runs)))
                } else {
                    Ok(None)
                }
            }
        }
    }

    fn find_longest_runs(&self) -> LongestRuns {
        // We need some way of computing the diagonals.

        LongestRuns {
            vertical: RunData { ai: 0, opponent: 1 },
            horizontal: RunData { ai: 0, opponent: 1 },
            diagonal: RunData { ai: 0, opponent: 1 },
        }
    }

    fn winner(&self, longest_runs: &LongestRuns) -> Result<Option<Player>, GameError> {
        longest_runs.winner(self.number_for_win)
    }

    fn score_incomplete_game(&self, longest_runs: &LongestRuns) -> isize {
        100
    }
}

impl fmt::Display for Game {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for row in &self.state {
            writeln!(f)?;
            for (i, space) in row.iter().enumerate() {
                if i > 0 && i < self.dimensions.width {
                    write!(f, " ")?;
                }
                write!(f, "{}", space)?;
            }
        }
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
