use cte::chess;
use cte::chess::board_manager::BoardManager;
use cte::chess::parse::{parse_algebraic_notation, ChessMove};
use cte::chess::piece::{Color, PieceType, Position};
use cte::chess::piece_movement::{
    is_valid_bishop_move, is_valid_king_move, is_valid_knight_move, is_valid_pawn_move,
    is_valid_queen_move, is_valid_rook_move,
};
//this wont be saved
fn generate_algebraic_notation_arrays() -> ([char; 8], [char; 8]) {
    let letters = ['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h'];
    let numbers = ['1', '2', '3', '4', '5', '6', '7', '8'];
    (numbers, letters)
}

#[test]
fn promote_pawn() {
    // todo! need to fix parsed, shifting to the left for some reason
    let mut board = BoardManager::new_from_fen("rnbqkbnr/1Pppp2p/8/8/4P3/8/pP1PP1PP/RNBQKBNR");
    println!("promote pawn {}", &board);
    board. 
    //println!("{:?}", &board);
}

#[test]
fn same_owner_test() {
    let board = BoardManager::new();
    println!("{}", &board);
    let test_data = [
        ((b'a', b'1'), (b'a', b'2'), true),  // white to white
        ((b'a', b'8'), (b'a', b'7'), true),  // black to black
        ((b'a', b'2'), (b'a', b'3'), false), // white to empty
        ((b'a', b'7'), (b'a', b'6'), false), //black to empty
        ((b'a', b'5'), (b'a', b'4'), false), // emtpy to empty
        ((b'a', b'2'), (b'a', b'7'), false), // white to black
    ];
    for (src, dest, expected) in test_data {
        let src_pos = parse_algebraic_notation(&src.0, &src.1).unwrap();
        let dest_pos = parse_algebraic_notation(&dest.0, &dest.1).unwrap();
        assert_eq!(board.same_owner(&src_pos, &dest_pos), expected);
    }
}

#[test]
fn algebraic_notation_test() {
    use chess::parse::*;
    let (num_arr, char_arr) = generate_algebraic_notation_arrays();
    for (n, c) in num_arr.into_iter().zip(char_arr.into_iter()) {
        let result = parse_algebraic_notation(&(n as u8), &(c as u8));
        assert!(result.is_err());
        let result = parse_algebraic_notation(&(c as u8), &(n as u8));
        assert!(result.is_ok());
    }
}

#[test]
fn test_knight_moves() {
    let src_position = (b'e', b'4');
    let src_position = parse_algebraic_notation(&src_position.0, &src_position.1).unwrap();
    let valid_destinations = [
        (b'c', b'3'),
        (b'c', b'5'),
        (b'd', b'2'),
        (b'd', b'6'),
        (b'f', b'2'),
        (b'f', b'6'),
        (b'g', b'3'),
        (b'g', b'5'),
    ];

    move_validation_helper(&valid_destinations, PieceType::Knight, src_position, true);

    let invalid_destinations = [
        (b'f', b'4'),
        (b'f', b'3'),
        (b'd', b'3'),
        (b'd', b'3'),
        (b'd', b'4'),
        (b'd', b'5'),
        (b'e', b'5'),
    ];

    move_validation_helper(
        &invalid_destinations,
        PieceType::Knight,
        src_position,
        false,
    )
}

#[test]
fn test_pawn_moves() {
    let src_position = (b'b', b'2');
    let src_position = parse_algebraic_notation(&src_position.0, &src_position.1).unwrap();

    let valid_destinations = [(b'a', b'3'), (b'b', b'3'), (b'c', b'3'), (b'b', b'4')];

    let invalid_destinations = [(b'b', b'5'), (b'd', b'4'), (b'a', b'2'), (b'd', b'2')];

    move_validation_helper(&valid_destinations, PieceType::Pawn, src_position, true);

    move_validation_helper(&invalid_destinations, PieceType::Pawn, src_position, false);
}

#[test]
fn test_queen_moves() {
    let src_position = (b'f', b'3');
    let src_position = parse_algebraic_notation(&src_position.0, &src_position.1).unwrap();

    let valid_destinations = [
        (b'f', b'6'),
        (b'f', b'1'),
        (b'a', b'8'),
        (b'd', b'1'),
        (b'h', b'1'),
        (b'h', b'3'),
        (b'h', b'5'),
        (b'e', b'3'),
    ];

    move_validation_helper(&valid_destinations, PieceType::Queen, src_position, true);
}

#[test]
fn test_king_moves() {
    let src_position = (b'f', b'3');
    let src_position = parse_algebraic_notation(&src_position.0, &src_position.1).unwrap();

    let valid_destinations = [
        (b'f', b'4'),
        (b'f', b'2'),
        (b'e', b'4'),
        (b'e', b'2'),
        (b'g', b'2'),
        (b'g', b'3'),
        (b'g', b'4'),
        (b'e', b'3'),
    ];
    let invalid_destinations = [
        (b'f', b'5'),
        (b'f', b'1'),
        (b'd', b'3'),
        (b'h', b'3'),
        (b'd', b'5'),
        (b'h', b'1'),
        (b'h', b'5'),
        (b'd', b'1'),
    ];
    move_validation_helper(&valid_destinations, PieceType::King, src_position, true);

    move_validation_helper(&invalid_destinations, PieceType::King, src_position, false);
}

pub fn move_validation_helper(
    arr_of_moves: &[(u8, u8)],
    p_type: PieceType,
    source_position: Position,
    bv: bool,
) {
    use PieceType::*;
    for dest in arr_of_moves {
        let dest_position = parse_algebraic_notation(&dest.0, &dest.1).unwrap();
        let validation_result = match p_type {
            King => is_valid_king_move(&source_position, &dest_position),
            Queen => is_valid_queen_move(&source_position, &dest_position),
            Bishop => is_valid_bishop_move(&source_position, &dest_position),
            Rook => is_valid_rook_move(&source_position, &dest_position),
            Pawn => is_valid_pawn_move(&source_position, &dest_position),
            Knight => is_valid_knight_move(&source_position, &dest_position),
        };
        assert_eq!(validation_result, bv);
    }
}

#[test]
fn test_rook_moves() {
    let src_position = (b'e', b'4');
    let src_position = parse_algebraic_notation(&src_position.0, &src_position.1).unwrap();

    let valid_destinations = [(b'a', b'4'), (b'e', b'1'), (b'h', b'4'), (b'e', b'8')];
    move_validation_helper(&valid_destinations, PieceType::Rook, src_position, true);

    let invalid_destinations = [(b'b', b'1'), (b'h', b'1'), (b'a', b'8'), (b'h', b'8')];

    move_validation_helper(&invalid_destinations, PieceType::Rook, src_position, false);
}
