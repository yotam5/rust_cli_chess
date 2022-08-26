mod chess;
use chess::{board::Board};
use std::io::{self, Read, BufRead};


fn input_read() ->  [u8;2]
{
    let stdin = std::io::stdin();
    let mut input: [u8;2] = Default::default();
    let mut bstdin = std::io::BufReader::new(stdin.take(2));
    bstdin.read(&mut input).unwrap();
    input
}

fn main() {
    let mut board = Board::new();
    println!("{}",&board);
    println!("{:?}",input_read());
}
