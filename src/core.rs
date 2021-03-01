use std::collections::HashMap;

#[derive(Debug)]
pub struct Game {
    pub board: Board,
    is_finished: bool,
    current_player: Player,
    winner: Option<Player>,
    players: HashMap<Player, String>,
}

impl Game {
    pub fn new(player_one_name: String, player_two_name: String) -> Self {
        let mut players = HashMap::new();
        players.insert(Player::One, player_one_name);
        players.insert(Player::Two, player_two_name);

        Self {
            players,
            winner: None,
            board: Board::new(),
            is_finished: false,
            current_player: Player::One,
        }
    }

    pub fn set_position(&mut self, position: u8) {
        self.board.mark_position(position, self.current_player);

        if self.board.player_won(self.current_player) {
            self.winner = Some(self.current_player);
            self.is_finished = true;
            return;
        }

        match self.current_player {
            Player::One => self.current_player = Player::Two,
            _ => self.current_player = Player::One,
        }
    }

    pub fn get_is_finished(&self) -> bool {
        self.is_finished
    }

    pub fn get_winner_name(&self) -> Option<&String> {
        match self.winner {
            Some(player) => Some(self.get_player_name(player)),
            _ => None,
        }
    }

    pub fn get_current_player_name(&self) -> &String {
        self.get_player_name(self.current_player)
    }

    fn get_player_name(&self, player: Player) -> &String {
        self.players.get(&player).unwrap()
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum Player {
    One,
    Two,
}

impl Player {
    fn get_marker(&self) -> &str {
        match *self == Self::One {
            true => "X",
            _ => "O",
        }
    }
}

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

    fn mark_position(&mut self, position: u8, player: Player) {
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

    fn player_won(&mut self, player: Player) -> bool {
        let one = self.position_has_value(1, player);
        let two = self.position_has_value(2, player);
        let three = self.position_has_value(3, player);
        let four = self.position_has_value(4, player);
        let five = self.position_has_value(5, player);
        let six = self.position_has_value(6, player);
        let seven = self.position_has_value(7, player);
        let eight = self.position_has_value(8, player);
        let nine = self.position_has_value(9, player);

        if one && two && three {
            return true;
        } else if four && five && six {
            return true;
        } else if seven && eight && nine {
            return true;
        } else if one && four && seven {
            return true;
        } else if two && five && eight {
            return true;
        } else if three && six && nine {
            return true;
        } else if one && five && nine {
            return true;
        } else if three && five && seven {
            return true;
        }

        false
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
