use std::collections::HashMap;

#[derive(Debug)]
pub struct Board {
    table: HashMap<u8, bool>,
    winner: Option<bool>,
}

impl Board {
    pub fn new() -> Self {
        Self {
            winner: None,
            table: HashMap::with_capacity(9),
        }
    }

    pub fn get_winner(&self) -> Option<bool> {
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

    pub fn mark_position(&mut self, position: u8, is_x: bool) {
        if Self::position_is_valid(position) && self.can_mark_position(position) {
            self.table.insert(position, is_x);
            self.check_winner(is_x);
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

    fn check_winner(&mut self, is_x: bool) {
        let one = self.position_has_value(1, is_x);
        let two = self.position_has_value(2, is_x);
        let three = self.position_has_value(3, is_x);
        let four = self.position_has_value(4, is_x);
        let five = self.position_has_value(5, is_x);
        let six = self.position_has_value(6, is_x);
        let seven = self.position_has_value(7, is_x);
        let eight = self.position_has_value(8, is_x);
        let nine = self.position_has_value(9, is_x);

        if one && two && three {
            self.set_winner(is_x);
        } else if four && five && six {
            self.set_winner(is_x);
        } else if seven && eight && nine {
            self.set_winner(is_x);
        } else if one && four && seven {
            self.set_winner(is_x);
        } else if two && five && eight {
            self.set_winner(is_x);
        } else if three && six && nine {
            self.set_winner(is_x);
        } else if one && five && nine {
            self.set_winner(is_x);
        } else if three && five && seven {
            self.set_winner(is_x);
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

    fn set_winner(&mut self, winner: bool) {
        self.winner = Some(winner);
    }
}
