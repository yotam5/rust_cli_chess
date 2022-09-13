use std::error::Error;
use std::fmt;
use std::fmt::Formatter;

use array2ds::array2d::Array2d;
use array2ds::array2d::GridIdx;

use crate::chess::piece::PieceType::King;

use super::piece::{Color, Piece, PieceType, Position};
use super::piece_movement as pm;
use super::piece_movement::Velocity;

type MyResult<T> = Result<T, Box<dyn Error>>;

pub struct BoardSizeInfo();

pub struct KingsTracker
{
    pub(super) white_king_pos: Position,
    pub(super) black_king_pos: Position,
}

impl BoardSizeInfo
{
    pub fn row_count() -> usize { 8_usize }
    pub fn column_count() -> usize { 8_usize }
}

type Board = Array2d<Square>;

#[derive(Debug, Eq, PartialEq, Default)]
struct Square(Option<Piece>);


impl Square
{
    pub fn new(piece_info: Piece) -> Self
    {
        Square(Some(piece_info))
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_none()
    }
}


#[derive(Debug)]
pub struct BoardManager {
    board: Board,
    turns_counter: usize,
    white_king_pos: Position,
    black_king_pos: Position,
}

impl GridIdx for Position {
    fn no_row(&self) -> usize {
        self.x as usize
    }

    fn no_column(&self) -> usize {
        self.y as usize
    }
}


impl Default for BoardManager
{
    fn default() -> Self {
        let mut board = Board::filled_with_default(
            BoardSizeInfo::row_count(),
            BoardSizeInfo::column_count());

        let king_tracker = BoardManager::load_default_game_position(&mut board);

        BoardManager {
            board,
            turns_counter: 0,
            white_king_pos: king_tracker.white_king_pos,
            black_king_pos: king_tracker.black_king_pos,
        }
    }
}

impl BoardManager {
    pub fn new() -> Self {
        BoardManager::default()
    }

    /// check if two positions have the same owner
    pub fn same_owner(&self, src: &Position, dest: &Position) -> bool
    {
        let square_src = &self.board[*src];
        let square_dest = &self.board[*dest];

        if let [Some(p_source), Some(p_dest)] =
        [&square_src.0, &square_dest.0]
        {
            return p_source.p_color == p_dest.p_color;
        }

        false
    }

    /// load starting position for the chess game
    fn load_default_game_position(board: &mut Board) -> KingsTracker {
        let initial_game_position = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR";
        BoardManager::load_fen_string_to_board(board, initial_game_position).unwrap()
    }

    /// load fen string to the board
    fn load_fen_string_to_board(board: &mut Board, fen_string: &str) -> MyResult<KingsTracker> {
        let mut black_king_pos = None;
        let mut white_king_pos = None;
        for (line_number, line_fen_value) in fen_string.split('/').enumerate() {
            let mut current_line_index: usize = 0;

            for fen_value in line_fen_value.chars() {
                if fen_value.is_numeric() {
                    let fen_value = fen_value.to_digit(10).unwrap() as usize;
                    for empty_index in current_line_index..fen_value {
                        board[(BoardSizeInfo::row_count() - 1 - line_number, empty_index)]
                            = Square::default();
                    }
                    current_line_index += fen_value - 1;
                } else if fen_value.is_ascii_alphabetic() {
                    let p_type = fen_value.into();
                    let p_color = fen_value.into();
                    let p_position = Position::new(
                        (BoardSizeInfo::row_count() - 1 - line_number) as i8,
                        current_line_index as i8);
                    match (p_type, p_color)
                    {
                        (King, Color::White) => white_king_pos = Some(p_position),
                        (King, Color::Black) => black_king_pos = Some(p_position),
                        _ => {}
                    }
                    board
                        [(BoardSizeInfo::row_count() - (line_number + 1), current_line_index)] =
                        Square::new(
                            Piece::new(
                                p_type,
                                p_color,
                            ));
                    current_line_index += 1;
                }
            }
        }

        if white_king_pos.is_none() {
            return Err("White King Not Found")?;
        }
        if black_king_pos.is_none()
        {
            return Err("Black King Not Found")?;
        }

        let white_king_pos = white_king_pos.unwrap();
        let black_king_pos = black_king_pos.unwrap();
        Ok(KingsTracker {
            white_king_pos,
            black_king_pos,
        })
    }

    /// handle a chess move and  return bool if performed or not
    pub fn handle_move(&mut self, src: &Position, dest: &Position) -> bool
    {
        let mn = self.perform_move(src,dest).unwrap();
        false
    }

    fn perform_move(&mut self, src: &Position, dest: &Position) -> MyResult<Option<Piece>> {
        let piece_source = &self.board[*src].0;
        piece_source.ok_or("Illegal Move, Can't Move An Empty Square")?;

        let dest_is_invalid = self.same_owner(src, dest);
        if dest_is_invalid {
            Err("Can't Eat The Same Color")?;
        }

        let piece_source = piece_source.unwrap();
        let valid_move = BoardManager::is_valid_move(
            &piece_source.p_type, src, dest);

        if !valid_move {
            Err("Piece Can't Move That Way")?;
        }

        if !self.check_dest_path_is_clear(src, dest) {
            Err("That Piece Movement Path Is Blocked")?;
        }

        let captured = self.board[*dest].0.take();
        self.board.swap(src, dest);
        self.turns_counter += 1;
        Ok(captured)
    }

    fn undo_move(&mut self, src: &Position, dest: &Position) {
        self.board.swap(src, dest);
    }

    pub fn is_check(&self, src: &Position, dest: &Position) -> bool
    {
        /*
         perform the move on the board, save the previous state of both places
         check if the king of the color that performed the move is in danger,
         undo the move, or to find another more efficient way instread of coying the Piece
         each call or so
         */
        false
    }


    /// check that the move is valid, if piece dest is legal movement if not interrupted by anything
    pub fn is_valid_move(piece_type: &PieceType, src: &Position, dest: &Position) -> bool
    {
        use PieceType::*;
        match piece_type {
            Knight => pm::is_valid_knight_move(src, dest),
            Bishop => pm::is_valid_bishop_move(src, dest),
            Queen => pm::is_valid_queen_move(src, dest),
            Rook => pm::is_valid_rook_move(src, dest),
            Pawn => pm::is_valid_pawn_move(src, dest),
            King => pm::is_valid_king_move(src, dest),
        }
    }

    /// check that the movement path of the  piece is clear, not blocked
    pub fn check_dest_path_is_clear(&self, src: &Position, dest: &Position) -> bool
    {
        let velocity = Velocity::new(src, dest);
        let mut curr_pos = Position::new(src.x, src.y);

        for _ in 0..BoardSizeInfo::row_count()
        {
            curr_pos.x += velocity.x;
            curr_pos.y += velocity.y;

            let current_square = &self.board[curr_pos];

            // todo!: check if it invalidate eating an enemy
            if &curr_pos == dest && !self.same_owner(src, &curr_pos)
            {
                break;
            }

            if current_square.0.is_some()
            {
                return false;
            }
        }

        true
    }

    /// display the black in the front
    pub fn output_black_front(&self, f: &mut Formatter) -> fmt::Result {
        writeln!(f)?;

        for (row_number, row_value) in self.board.iter_rows().enumerate() {
            write!(f, "{:>2}", row_number + 1)?;

            for square in row_value.iter() {
                match &square.0
                {
                    Some(piece) => write!(f, "{:>2}", piece)?,
                    None => write!(f, "{:>2}", "·")?,
                }
            }

            writeln!(f)?;
        }
        Ok(())
    }

    /// display white in the front
    pub fn output_white_front(&self, f: &mut Formatter) -> fmt::Result {
        writeln!(f)?;

        for (row_number, row_value) in self.board.iter_rows().rev().enumerate()
        {
            write!(f, "{:>2}", BoardSizeInfo::row_count() - row_number).unwrap();

            for square in row_value
            {
                match &square.0
                {
                    Some(piece) => write!(f, "{:>2}", piece)?,
                    None => write!(f, "{:>2}", "·")?,
                }
            }

            writeln!(f)?;
        }
        Ok(())
    }
}

/// format algebraic notation alphabetic
pub fn algebraic_notation_letters_formatted(f: &mut Formatter)
{
    write!(f, "{:>2}", " ").unwrap();

    for c in 'a'..='h' {
        write!(f, "{:>2}", c).unwrap();
    }
}

impl fmt::Display for BoardManager {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        algebraic_notation_letters_formatted(f);

        if self.turns_counter % 2 == 0
        {
            self.output_white_front(f)?;
        } else {
            self.output_black_front(f)?;
        }

        algebraic_notation_letters_formatted(f);
        write!(f, "")
    }
}