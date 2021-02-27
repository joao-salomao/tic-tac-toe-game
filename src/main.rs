use std::collections::HashMap;
use std::io;

#[derive(Debug)]
struct Board {
    table: HashMap<u8, bool>,
    winner: Option<bool>,
}

impl Board {
    fn new() -> Self {
        Self {
            winner: None,
            table: HashMap::with_capacity(9),
        }
    }

    fn mark_position(&mut self, position: u8, is_x: bool) {
        if self.position_is_valid(position) && self.can_mark_position(position) {
            self.table.insert(position, is_x);
            self.check_winner(is_x);
        }
    }

    fn position_is_valid(&self, position: u8) -> bool {
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
        match self.table.get(&position) {
            Some(marker) => *marker == value,
            _ => false,
        }
    }

    fn set_winner(&mut self, winner: bool) {
        self.winner = Some(winner);
    }

    fn get_winner(&self) -> Option<bool> {
        self.winner
    }
}

fn main() {
    println!("Type the player one name: ");
    let player_one = get_input();

    println!("Type the player two name: ");
    let player_two = get_input();

    let mut game_is_ended = false;
    let mut current_player = true;
    let mut board = Board::new();

    while game_is_ended != true {
        let current_player_name = get_current_player_name(current_player, &player_one, &player_two);
        println!("Type the position {}: ", &current_player_name);
        board.mark_position(get_position(), current_player);
        current_player = !current_player;

        if let Some(_) = board.get_winner() {
            println!("The player {} winner", &current_player_name);
            game_is_ended = true;
        }
    }

    dbg!(board);
}

fn get_current_player_name<'a>(
    current: bool,
    player_one: &'a String,
    player_two: &'a String,
) -> &'a String {
    match current {
        true => &player_one,
        _ => &player_two,
    }
}

fn get_position() -> u8 {
    get_input().parse::<u8>().expect("Error trying parse input")
}

fn get_input() -> String {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Error getting the player one name");
    input.replace("\r\n", "")
}
