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
#[derive(Debug, Copy, Clone)]
pub struct ChessMove {
    pub piece_source: Position,
    pub piece_dest: Position,
    pub prompted: Option<PieceType>,
}

impl ChessMove
{
    pub fn new(ps: Position, pd: Position, pr: Option<PieceType>) -> Self
    {
        ChessMove {
            piece_source: ps,
            piece_dest: pd,
            prompted: pr,
        }
    }
}

/// chess turn is used to redo the turn
/// and save the previous turns in the game,
/// after they have been applied
#[derive(Debug, Copy, Clone)]
pub struct ChessTurn {
    pub chess_move: ChessMove,
    pub piece_eaten: Option<Piece>,
}

pub fn is_valid_uci_piece_character(piece_char: &u8) -> bool
{
    ['r', 'k', 'b', 'q', 'n', 'p'].contains(&(*piece_char as char))
}

pub fn is_valid_promotion(piece_char: &u8) -> bool
{
    is_valid_uci_piece_character(piece_char) && (*piece_char as char) != 'k'
}

pub fn is_valid_algebraic_notation_column(column: &char) -> bool
{
    ['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h'].contains(column)
}

pub fn is_valid_algebraic_notation_row(row: &char) -> bool
{
    let column = row.to_digit(10);
    if column.is_none() {
        return false;
    }
    let column = column.unwrap();
    column <= (BoardSizeInfo::row_count() as u32)
}

/// check if algebraic notation is valid
pub fn is_valid_algebraic_notation(column: &u8, row: &u8) -> bool {
    is_valid_algebraic_notation_column(&(*column as char))
        && is_valid_algebraic_notation_row(&(*row as char))
}

/// need to parse string to a move,
/// for example castling: e1g1 and promotion e7e8q
/// todo! array of 5 letters, and to add support for
/// piece pawn promotion and castling

/// parse algebraic notation to Position on board note the subtraction of 1
/// for board array indexing that starts with 0
pub fn parse_algebraic_notation(col: &u8, row: &u8) -> MyResult<Position> {
    if is_valid_algebraic_notation(col, row) {
        let column = col - b'a';
        let row = (*row as char).to_digit(10).unwrap() - 1;
        println!("pos: {}-{}",row as i8, column as i8);
        return Ok(Position::new(row as i8, column as i8));
    }
    Err("Invalid algebraic notation for piece location")?
}
