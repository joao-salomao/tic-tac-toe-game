use super::board::Board;
use super::board::PositionError;
use super::player::Player;
use std::collections::HashMap;

#[derive(Debug)]
pub struct Game {
    pub board: Board,
    plays: u8,
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
            plays: 0,
            winner: None,
            board: Board::new(),
            is_finished: false,
            current_player: Player::One,
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

    pub fn play(&mut self, position: u8) -> Result<(), PositionError> {
        if let Err(e) = self.set_position(position) {
            return Err(e);
        }

        self.increment_plays();

        if self.is_winner(self.current_player) {
            self.set_is_finished(true);
            self.set_winner(self.current_player);
        } else if self.get_plays() == 9 {
            self.set_is_finished(true);
        } else {
            self.update_current_player();
        }

        Ok(())
    }

    fn set_position(&mut self, position: u8) -> Result<(), PositionError> {
        self.board.mark_position(position, self.current_player)
    }

    fn increment_plays(&mut self) {
        self.plays += 1;
    }

    fn is_winner(&mut self, player: Player) -> bool {
        self.board.player_won(player)
    }

    fn set_is_finished(&mut self, value: bool) {
        self.is_finished = value;
    }

    fn set_winner(&mut self, player: Player) {
        self.winner = Some(player);
    }

    fn get_plays(&self) -> u8 {
        self.plays
    }

    fn update_current_player(&mut self) {
        match self.current_player {
            Player::One => self.current_player = Player::Two,
            _ => self.current_player = Player::One,
        }
    }
}
