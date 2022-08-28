use std::io::{self, BufRead, Read};

use chess::board::Board;
use chess::parse;

mod chess;

fn input_read() -> [u8; 2]
{
    let stdin = io::stdin();
    let mut input: [u8; 2] = Default::default();
    let mut bstdin = io::BufReader::new(stdin.take(2));
    let _ = bstdin.read(&mut input).unwrap();
    input
}

fn main() {
    let mut board = Board::new();
    println!("{}", &board);
    let input = input_read();
    let parsed = parse::parse_algebraic_notation(&input).unwrap();
    println!("{:?}", parse::parse_algebraic_notation(&input));
    board.handle_move(&parsed,&parsed);
}
