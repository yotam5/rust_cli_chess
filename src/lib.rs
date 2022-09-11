use std::io::{self, BufReader, Read};

use chess::board_manager::BoardManager;
use chess::parse;
use chess::piece::Position;

pub mod chess;

fn input_read() -> [u8; 2] {
    let stdin = io::stdin();
    let mut input: [u8; 2] = Default::default();

    let mut bstdin = BufReader::new(stdin.take(4));
    let _ = bstdin.read(&mut input).unwrap();
    input
}

fn clear_screen() {
    print!("\x1B[2J\x1B[1;1H");
}

fn ask_input() -> [u8; 2] {
    println!("selected position: [row,col]");
    input_read()
}

fn get_move() -> Position {
    let mut parsed = parse::parse_algebraic_notation(&ask_input());
    while parsed.is_err() {
        parsed = parse::parse_algebraic_notation(&ask_input());
    }
    parsed.unwrap()
}

pub fn run_game()
{
    let mut board = BoardManager::new();
    let mut game_end = false;
    while !game_end {
        println!("{}", &board);
        let src = get_move();
        let dest = get_move();
        board.handle_move(&src, &dest);
        clear_screen();
    }
}