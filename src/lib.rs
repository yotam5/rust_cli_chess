use std::io::{self, BufRead, BufReader, Read, Write};
use std::ops::Index;

use crate::parse::{is_valid_promotion, is_valid_uci_piece_character, parse_algebraic_notation};
use chess::board_manager::BoardManager;
use chess::parse;
use chess::parse::ChessMove;
use chess::piece::PieceType;

type MyResult<T> = Result<T, dyn std::error::Error>;

pub mod chess;

fn input_user_move() -> String {
    let mut input = String::with_capacity(4);
    io::stdout().flush().unwrap();
    let stdin = io::stdin();
    let mut bstdin = BufReader::new(stdin.take(6));
    bstdin.read_line(&mut input).unwrap();
    let _ = input.pop();
    input
}

fn parse_chess_move(chess_move: &str) -> Result<ChessMove, Box<dyn std::error::Error>> {
    let mut chess_move_chunks = chess_move.as_bytes().chunks(2);
    if chess_move_chunks.len() < 2 || chess_move_chunks.len() > 3 {
        Err("Chess Move Is Two Squares And Optional Promotion")?;
    }

    let source_square = chess_move_chunks.next().unwrap();
    let source_square = parse_algebraic_notation(source_square.index(0), source_square.index(1))?;

    let dest_square = chess_move_chunks.next().unwrap();
    let dest_square = parse_algebraic_notation(dest_square.index(0), dest_square.index(1))?;

    let promotion_chunk = chess_move_chunks.next();
    let mut promotion_type = None;

    if promotion_chunk.is_some() {
        let promotion_chunk = promotion_chunk.unwrap();
        if !is_valid_promotion(&promotion_chunk[0]) {
            Err("Not A Valid Promotion Piece")?;
        }
        let piece_char = promotion_chunk[0] as char;
        promotion_type = Some(piece_char.into());
    }

    Ok(ChessMove::new(source_square, dest_square, promotion_type))
}

fn clear_screen() {
    print!("\x1B[2J\x1B[1;1H");
}

fn get_user_move() -> ChessMove {
    let mut move_info = input_user_move();
    let mut parsed_move_info = parse_chess_move(&move_info);

    while parsed_move_info.is_err() {
        println!("{:?}", &parsed_move_info);

        move_info = input_user_move();

        parsed_move_info = parse_chess_move(&move_info);
    }
    parsed_move_info.unwrap()
}

pub fn run_game() {
    let mut board = BoardManager::new();
    //println!("{:?}", &board);
    let mut game_end = false;
    println!("{:?}", std::mem::size_of::<parse::AlgebraicNotation>());
    println!("{}", &board);
    while !game_end {
        let move_info = get_user_move();
        let move_result = board.handle_move(&move_info);
        if move_result.is_err() {
            println!("Illegal move: {:?}", &move_result);
            continue;
        }
        //clear_screen();
        println!("{}", &board);
    }
}
