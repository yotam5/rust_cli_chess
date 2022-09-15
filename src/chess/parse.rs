use std::error::Error;

use crate::chess::piece::{Piece, PieceType};

use super::board_manager::BoardSizeInfo;
use super::piece::Position;

//todo! read input string and convert to ChessMove and handle
// castling + pawn promption

// NOTE: CAPLOCK + K is like mouse press
type MyResult<T> = Result<T, Box<dyn Error>>;

pub struct AlgebraicNotation
{
    square_source: u8,
    square_dest: u8,
    promotion: Option<u8>,
}


/// chess move is only the action in the turn
/// for example Pawn prompted: e7e8q
/// castling is just like movingthe king to the rook
pub struct ChessMove {
    piece_source: Position,
    piece_dest: Position,
    prompted: Option<Piece>,
}

/// chess turn is used to redo the turn
/// and save the previous turns in the game,
/// after they have been applied
pub struct ChessTurn {
    chess_move: ChessMove,
    piece_eaten: Option<Piece>,
}

pub fn is_valid_uci_piece_character(piece_char: &char) -> bool
{
    ['r', 'k', 'b', 'q', 'n', 'p'].contains(piece_char)
}

pub fn is_valid_algebraic_notation_row(row: &char) -> bool
{
    ['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h'].contains(row)
}

pub fn is_valid_algebraic_notation_column(column: &char) -> bool
{
    let column = column.to_digit(10);
    if column.is_none() {
        return false;
    }
    let column = column.unwrap();
    column <= (BoardSizeInfo::row_count() as u32)
}

/// check if algebraic notation is valid
pub fn is_valid_algebraic_notation(an_arr: &[u8; 2]) -> bool {
    let row = an_arr[0];
    let column = an_arr[1];

    if !(row.is_ascii_alphabetic() && row.is_ascii_lowercase() && column.is_ascii_digit()) {
        return false;
    }

    let valid_row = is_valid_algebraic_notation_row(&(row as char));
    if !valid_row {
        return false;
    }

    let valid_column = is_valid_algebraic_notation_column(&(column as char));
    if !valid_column {
        return false;
    }

    true
}

/// need to parse string to a move,
/// for example castling: e1g1 and promotion e7e8q
/// todo! array of 5 letters, and to add support for
/// piece pawn promotion and castling

/// parse algebraic notation to Position on board note the subtraction of 1
/// for board array indexing that starts with 0
pub fn parse_algebraic_notation(an_arr: &[u8; 2]) -> MyResult<Position> {
    if is_valid_algebraic_notation(an_arr) {
        let column = an_arr[0] - b'a';
        let row = (an_arr[1] as char).to_digit(10).unwrap() - 1;
        return Ok(Position::new(row as i8, column as i8));
    }
    Err("Invalid algebraic notation for piece location")?
}
