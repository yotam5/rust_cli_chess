mod chess;
use chess::{board::Board};
use chess::parse;
use std::io::{self, Read, BufRead};


fn input_read() ->  [u8;2]
{
    let stdin = io::stdin();
    let mut input: [u8;2] = Default::default();
    let mut bstdin = io::BufReader::new(stdin.take(2));
    bstdin.read(&mut input).unwrap();
    input
}

fn main() {
    let mut board = Board::new();
    println!("{}",&board);
    let input = input_read();
    let parsed = parse::parse_algebraic_notation(&input).unwrap();
    println!("{:?}",parse::parse_algebraic_notation(&input));

    let opt = Some(3_i32);


    board.empty_at(&parsed);
}
