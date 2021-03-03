pub trait Error {
    fn message(&self) -> &str;
}

#[derive(Debug)]
pub enum GameStatusError {
    Finished,
    GameOver,
}

impl Error for GameStatusError {
    fn message(&self) -> &str {
        match self {
            Self::GameOver => "Game over",
            Self::Finished => "Game finished",
        }
    }
}

#[derive(Debug)]
pub enum PositionError {
    PositionInvalid,
    PositionAlreadyMarked,
}

impl Error for PositionError {
    fn message(&self) -> &str {
        match self {
            Self::PositionInvalid => "Position must be between 1 and 9",
            Self::PositionAlreadyMarked => "The position was already marked",
        }
    }
}
