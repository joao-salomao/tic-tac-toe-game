use super::player::Player;
use std::collections::HashMap;

const WINNER_POSITIONS: [(u8, u8, u8); 8] = [
    (1, 2, 3),
    (4, 5, 6),
    (7, 8, 9),
    (1, 4, 7),
    (2, 5, 8),
    (3, 6, 9),
    (1, 5, 9),
    (3, 5, 7),
];

#[derive(Debug)]
pub struct Board {
    table: HashMap<u8, Player>,
}

impl Board {
    pub fn new() -> Self {
        Self {
            table: HashMap::with_capacity(9),
        }
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
            Some(player) => player.get_marker(),
            _ => " ",
        }
    }

    pub fn mark_position(&mut self, position: u8, player: Player) {
        if Self::position_is_valid(position) && self.can_mark_position(position) {
            self.table.insert(position, player);
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

    pub fn player_won(&mut self, player: Player) -> bool {
        for positions in WINNER_POSITIONS.iter() {
            if self.positions_were_marked_by_player(positions.0, positions.1, positions.2, player) {
                return true;
            }
        }

        false
    }

    fn positions_were_marked_by_player(
        &self,
        position_one: u8,
        position_two: u8,
        position_three: u8,
        player: Player,
    ) -> bool {
        self.position_has_value(position_one, player)
            && self.position_has_value(position_two, player)
            && self.position_has_value(position_three, player)
    }

    fn position_has_value(&self, position: u8, player: Player) -> bool {
        match self.get_position_value(position) {
            Some(value) => *value == player,
            _ => false,
        }
    }

    fn get_position_value(&self, position: u8) -> Option<&Player> {
        self.table.get(&position)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_mark_position() {
        let mut board = Board::new();

        board.mark_position(1, Player::One);
        board.mark_position(2, Player::One);
        board.mark_position(3, Player::One);
        board.mark_position(4, Player::Two);
        board.mark_position(5, Player::Two);
        board.mark_position(6, Player::Two);

        assert_eq!(*board.get_position_value(1).unwrap(), Player::One);
        assert_eq!(*board.get_position_value(2).unwrap(), Player::One);
        assert_eq!(*board.get_position_value(3).unwrap(), Player::One);
        assert_eq!(*board.get_position_value(4).unwrap(), Player::Two);
        assert_eq!(*board.get_position_value(5).unwrap(), Player::Two);
        assert_eq!(*board.get_position_value(6).unwrap(), Player::Two);

        let position_value_to_bool = |v| match board.get_position_value(v) {
            Some(_) => true,
            _ => false,
        };

        assert_eq!(position_value_to_bool(7), false);
        assert_eq!(position_value_to_bool(8), false);
        assert_eq!(position_value_to_bool(9), false);
    }
}
