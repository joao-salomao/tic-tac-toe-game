use std::collections::HashMap;

#[derive(Debug, Copy, Clone)]
pub enum Player {
    One,
    Two,
}

impl Player {
    pub fn from_bool(value: bool) -> Self {
        match value {
            true => Self::One,
            _ => Self::Two,
        }
    }

    fn parse_to_bool(player: Self) -> bool {
        match player {
            Self::One => true,
            _ => false,
        }
    }
}

#[derive(Debug)]
pub struct Board {
    table: HashMap<u8, bool>,
    winner: Option<Player>,
}

impl Board {
    pub fn new() -> Self {
        Self {
            winner: None,
            table: HashMap::with_capacity(9),
        }
    }

    pub fn get_winner(&self) -> Option<Player> {
        self.winner
    }

    pub fn show(&self) {
        let board = format!(
            "| {} | {} | {} |\n| {} | {} | {} |\n| {} | {} | {} |\n",
            self.get_position_marker(1),
            self.get_position_marker(2),
            self.get_position_marker(3),
            self.get_position_marker(4),
            self.get_position_marker(5),
            self.get_position_marker(6),
            self.get_position_marker(7),
            self.get_position_marker(8),
            self.get_position_marker(9)
        );
        println!("{}", board);
    }

    fn get_position_marker(&self, position: u8) -> &str {
        match self.get_position_value(position) {
            Some(value) => {
                if *value {
                    "X"
                } else {
                    "O"
                }
            }
            _ => " ",
        }
    }

    pub fn mark_position(&mut self, position: u8, player: Player) {
        if Self::position_is_valid(position) && self.can_mark_position(position) {
            self.table.insert(position, Player::parse_to_bool(player));
            self.check_winner(player);
        }
    }

    pub fn position_is_valid(position: u8) -> bool {
        position > 0 && position < 10
    }

    fn can_mark_position(&self, position: u8) -> bool {
        match self.table.get(&position) {
            Some(_) => false,
            _ => true,
        }
    }

    fn check_winner(&mut self, player: Player) {
        let player_bool_value = Player::parse_to_bool(player);
        let one = self.position_has_value(1, player_bool_value);
        let two = self.position_has_value(2, player_bool_value);
        let three = self.position_has_value(3, player_bool_value);
        let four = self.position_has_value(4, player_bool_value);
        let five = self.position_has_value(5, player_bool_value);
        let six = self.position_has_value(6, player_bool_value);
        let seven = self.position_has_value(7, player_bool_value);
        let eight = self.position_has_value(8, player_bool_value);
        let nine = self.position_has_value(9, player_bool_value);

        if one && two && three {
            self.set_winner(player);
        } else if four && five && six {
            self.set_winner(player);
        } else if seven && eight && nine {
            self.set_winner(player);
        } else if one && four && seven {
            self.set_winner(player);
        } else if two && five && eight {
            self.set_winner(player);
        } else if three && six && nine {
            self.set_winner(player);
        } else if one && five && nine {
            self.set_winner(player);
        } else if three && five && seven {
            self.set_winner(player);
        }
    }

    fn position_has_value(&self, position: u8, value: bool) -> bool {
        match self.get_position_value(position) {
            Some(marker) => *marker == value,
            _ => false,
        }
    }

    fn get_position_value(&self, position: u8) -> Option<&bool> {
        self.table.get(&position)
    }

    fn set_winner(&mut self, winner: Player) {
        self.winner = Some(winner);
    }
}
