#[derive(Debug)]
pub enum Error {
    PositionInvalid,
    PositionAlreadyMarked,
    GameFinished,
    GameOver,
}

impl Error {
    pub fn message(&self) -> &str {
        match self {
            Self::GameOver => "Game over",
            Self::GameFinished => "Game finished",
            Self::PositionInvalid => "Position must be between 1 and 9",
            Self::PositionAlreadyMarked => "The position was already marked",
        }
    }
}
