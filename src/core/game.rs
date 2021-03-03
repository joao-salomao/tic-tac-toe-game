use super::board::Board;
use super::error::Error;
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

    pub fn play(&mut self, position: u8) -> Result<(), Error> {
        if self.get_is_finished() {
            return Err(Error::GameFinished);
        }

        if self.is_game_over() {
            return Err(Error::GameOver);
        }

        if let Err(e) = self.set_position(position) {
            return Err(e);
        }

        self.increment_plays();

        if self.is_winner(self.current_player) {
            self.set_is_finished(true);
            self.set_winner(self.current_player);
        } else if self.is_game_over() {
            self.set_is_finished(true);
        } else {
            self.update_current_player();
        }

        Ok(())
    }

    fn is_game_over(&self) -> bool {
        self.get_plays() >= 9
    }

    fn set_position(&mut self, position: u8) -> Result<(), Error> {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn game_should_be_finished() {
        let mut game = create_game();
        assert_eq!(game.get_is_finished(), false);
        win_game(&mut game, Player::One);
        assert!(game.get_is_finished());
    }

    #[test]
    fn should_get_winner_name() {
        let mut game_one = create_game();
        win_game(&mut game_one, Player::One);
        assert_eq!(
            *game_one.get_winner_name().unwrap(),
            *game_one.get_player_name(Player::One)
        );

        let mut game_two = create_game();
        win_game(&mut game_two, Player::Two);
        assert_eq!(
            *game_two.get_winner_name().unwrap(),
            *game_two.get_player_name(Player::Two)
        );
    }

    #[test]
    fn should_play() {
        let mut game = create_game();
        assert!(game.play(1).is_ok());
        assert!(game.play(1).is_err());
        assert!(game.play(2).is_ok());
        assert!(game.play(2).is_err());
        assert!(game.play(4).is_ok());
        assert!(game.play(4).is_err());
        assert!(game.play(3).is_ok());
        assert!(game.play(3).is_err());
        assert!(game.play(7).is_ok());
        assert!(game.play(7).is_err());
    }

    #[test]
    fn should_not_play_on_finished_game() {
        let mut game = create_game();
        win_game(&mut game, Player::One);
        assert!(game.play(5).is_err());
        assert!(game.play(6).is_err());
        assert!(game.play(8).is_err());
        assert!(game.play(9).is_err());

        let mut game = create_game();
        win_game(&mut game, Player::Two);
        assert!(game.play(6).is_err());
        assert!(game.play(8).is_err());
        assert!(game.play(9).is_err());
    }

    fn win_game(game: &mut Game, winner: Player) {
        match winner {
            Player::One => {
                game.play(1).unwrap();
                game.play(2).unwrap();
                game.play(4).unwrap();
                game.play(3).unwrap();
                game.play(7).unwrap();
            }
            Player::Two => {
                game.play(2).unwrap();
                game.play(1).unwrap();
                game.play(3).unwrap();
                game.play(4).unwrap();
                game.play(5).unwrap();
                game.play(7).unwrap();
            }
        }
    }

    #[test]
    fn should_get_current_player_name() {
        let mut game = create_game();
        assert_eq!(
            *game.get_current_player_name(),
            *game.get_player_name(Player::One),
        );
        game.play(1).unwrap();
        assert_eq!(
            *game.get_current_player_name(),
            *game.get_player_name(Player::Two),
        );
        game.play(2).unwrap();
        assert_eq!(
            *game.get_current_player_name(),
            *game.get_player_name(Player::One),
        );
    }

    fn create_game() -> Game {
        Game::new(String::from("One"), String::from("Two"))
    }
}
