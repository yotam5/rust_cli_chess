use std::io::{self, BufReader, Read, Write};

use chess::board_manager::BoardManager;
use chess::parse;
use chess::piece::Position;

use crate::parse::parse_algebraic_notation;

pub mod chess;

fn input_read() -> ([u8; 2], [u8; 2]) {
    let stdin = io::stdin();
    let mut square_from: [u8; 2] = Default::default();
    let mut square_to: [u8; 2] = Default::default();

    io::stdout().flush().unwrap();
    let mut bstdin = BufReader::new(stdin.take(5));
    let _ = bstdin.read(&mut square_from).unwrap();
    let _ = bstdin.read(&mut square_to).unwrap();
    io::stdout().flush().unwrap();

    (square_from, square_to)
}

fn clear_screen() {
    print!("\x1B[2J\x1B[1;1H");
}

fn ask_input() -> ([u8; 2], [u8; 2]) {
    println!("enter move:");
    input_read()
}

fn get_user_move() -> (Position, Position) {
    let mut move_info = ask_input();
    let mut square_from = parse_algebraic_notation(&move_info.0);
    let mut square_to = parse_algebraic_notation(&move_info.1);

    while square_to.is_err() || square_from.is_err()
    {
        if square_to.is_err() { println!("{:?}", &square_to); }
        if square_from.is_err() { println!("{:?}", &square_from); }
        move_info = ask_input();
        square_from = parse_algebraic_notation(&move_info.0);
        square_to = parse_algebraic_notation(&move_info.1);
    }
    (square_from.unwrap(), square_to.unwrap())
}

pub fn run_game()
{
    let mut board = BoardManager::new();
    //println!("{:?}", &board);
    let mut game_end = false;

    println!("{}", &board);
    while !game_end {
        let move_info = get_user_move();
        let move_result = board.handle_move(&move_info.0, &move_info.1);
        if move_result.is_err(){
            println!("{:?}",&move_result);
            continue;
        }
        clear_screen();
        println!("{}", &board);
    }
}