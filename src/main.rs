mod core;

use crate::core::Board;
use crate::core::Player;
use std::io;

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
        board.mark_position(get_position(), Player::from_bool(current_player));
        current_player = !current_player;
        board.show();

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
