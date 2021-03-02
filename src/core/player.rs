#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum Player {
    One,
    Two,
}

impl Player {
    pub fn get_marker(&self) -> &str {
        match *self == Self::One {
            true => "X",
            _ => "O",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_return_player_marker() {
        assert_eq!("X", Player::One.get_marker());
        assert_eq!("O", Player::Two.get_marker());
        assert!("C" != Player::One.get_marker());
        assert!("V" != Player::Two.get_marker());
    }
}
