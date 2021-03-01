mod core;

use crate::core::Board;
use crate::core::Game;
use std::io;

fn main() {
    println!("Type the player one name: ");
    let player_one = get_input();

    println!("Type the player two name: ");
    let player_two = get_input();

    let mut game = Game::new(player_one, player_two);

    while game.get_is_finished() != true {
        println!("Type the position {}: ", game.get_current_player_name());

        game.set_position(get_position());
        game.board.show();

        if let Some(winner) = game.get_winner_name() {
            println!("Player {} won !", winner);
        }
    }

    dbg!(game);
}

fn get_position() -> u8 {
    loop {
        let position = match get_input().parse::<u8>() {
            Ok(value) => value,
            Err(_) => 0,
        };

        if !Board::position_is_valid(position) {
            println!("The provided position is invalid. Try again.");
            continue;
        }

        return position;
    }
}

fn get_input() -> String {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Error getting user input");
    input.replace("\r\n", "")
}
