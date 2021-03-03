mod core;

use crate::core::Board;
use crate::core::Error;
use crate::core::Game;
use std::io;

fn main() {
    println!("Type the player one name: ");
    let player_one = get_player_name();

    println!("Type the player two name: ");
    let player_two = get_player_name();

    let mut game = Game::new(player_one, player_two);

    while game.get_is_finished() != true {
        println!("Type the position {}: ", game.get_current_player_name());

        let position = get_position(&game.board);
        game.play(position).unwrap();
        game.board.show();

        let winner = game.get_winner_name();
        if winner == None && game.get_is_finished() {
            println!("Game over");
        } else if let Some(winner_name) = winner {
            println!("Player {} won !", winner_name);
        }
    }

    dbg!(game);
}

fn get_position(board: &Board) -> u8 {
    match get_input().parse::<u8>() {
        Ok(value) => match board.validate_position(value) {
            Ok(_) => return value,
            Err(e) => {
                println!("{}", e.message());
                return get_position(&board);
            }
        },
        Err(_) => {
            println!("The provided position is invalid. Try again.");
            return get_position(&board);
        }
    }
}

fn get_player_name() -> String {
    let failed_validation_message = "The provided name is invalid. Try again.";
    let validator = |input: &String| !input.is_empty();
    get_validated_input(failed_validation_message, validator)
}

fn get_validated_input(failed_validation_message: &str, validator: fn(&String) -> bool) -> String {
    let input = get_input();
    if validator(&input) {
        return input;
    } else {
        println!("{}", &failed_validation_message);
        return get_validated_input(&failed_validation_message, validator);
    }
}

fn get_input() -> String {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Error getting user input");
    input.trim().replace("\r\n", "")
}
