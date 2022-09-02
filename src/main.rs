use chess::board::Board;
use chess::parse;
use chess::piece::Position;
use cte::chess::board_manager::BoardManager;
use cte::chess::piece::Piece;
use cte::chess::{self, board::Square};
use std::io::{self, BufReader, Read};

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

fn main() {
    let mut board = BoardManager::new();
    let mut bb = Board::new();

    println!("jesus");
    println!("{:?}", &bb);
    let piece = Piece::new(
        chess::piece::PieceType::Bishop,
        chess::piece::Color::Black,
        Position::new(1, 2),
    );
    bb[(1, 1)] = Square::Some(piece);

    println!();
    println!("{:?}", &bb);

    println!("\n now for a normal loop\n");
    for i in &bb {
        println!("{:?}", i);
    }

    /*for row in bb.iter() {
        println!("{:?}", &row);
    }*/
    /*loop {
            //println!("{}", &board);
            println!("selected piece position");
            let move_src = get_move();
            println!("selected piece destination");
            let move_dest = get_move();
            //board.handle_move(&move_src, &move_dest);
    //        clear_screen();
        }*/
}
