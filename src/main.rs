mod chess;
use chess::{board::Board};

fn main() {
    let mut board = Board::new();
    println!("{}",&board);
}
