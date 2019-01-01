use std::fmt;

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
