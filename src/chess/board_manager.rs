use std::collections::VecDeque;
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

#[derive(Debug, Copy, Clone)]
pub struct ChessMove {
    move_src: Position,
    move_dest: Position,
    piece_eaten: Option<Piece>,
}


pub struct KingsTracker {
    pub(super) white_king_pos: Position,
    pub(super) black_king_pos: Position,
}

impl BoardSizeInfo {
    pub fn row_count() -> usize {
        8_usize
    }
    pub fn column_count() -> usize {
        8_usize
    }
}

type Board = Array2d<Square>;

#[derive(Debug, Eq, PartialEq, Default, Copy, Clone)]
struct Square(Option<Piece>);

impl Square {
    pub fn new(piece_info: Piece) -> Self {
        Square(Some(piece_info))
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_none()
    }

    pub fn same_owner(&self, color: Color) -> bool {
        if !self.is_empty() {
            return self.0.unwrap().p_color == color;
        }
        false
    }
}

pub struct BoardManager {
    board: Board,
    turns_counter: usize,
    moves_tracker: VecDeque<ChessMove>,
    white_king_pos: Position,
    black_king_pos: Position,
}

impl fmt::Debug for BoardManager
{
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.debug_struct("BoardManager")
            .field("turns_counter", &self.turns_counter)
            .field("moves_tracker", &self.moves_tracker)
            .field("white king pos", &self.white_king_pos)
            .field("black king pos", &self.black_king_pos)
            .finish()
    }
}

impl GridIdx for Position {
    fn no_row(&self) -> usize {
        self.x as usize
    }

    fn no_column(&self) -> usize {
        self.y as usize
    }
}

impl Default for BoardManager {
    fn default() -> Self {
        let mut board =
            Board::filled_with_default(BoardSizeInfo::row_count(), BoardSizeInfo::column_count());

        let king_tracker = BoardManager::load_default_game_position(&mut board);

        BoardManager {
            board,
            turns_counter: 0,
            moves_tracker: VecDeque::with_capacity(80),
            white_king_pos: king_tracker.white_king_pos,
            black_king_pos: king_tracker.black_king_pos,
        }
    }
}

impl BoardManager {
    pub fn new() -> Self {
        BoardManager::default()
    }

    pub fn new_from_fen(fen_string: &str) -> Self
    {
        let mut board = Board::filled_with_default(BoardSizeInfo::row_count(),
                                                   BoardSizeInfo::column_count());
        let king_tracker = BoardManager::load_fen_string_to_board
            (&mut board, fen_string).unwrap();
        BoardManager {
            board,
            turns_counter: 0,
            moves_tracker: VecDeque::with_capacity(80),
            white_king_pos: king_tracker.white_king_pos,
            black_king_pos: king_tracker.black_king_pos,
        }
    }

    /// check if two positions have the same owner
    pub fn same_owner(&self, src: &Position, dest: &Position) -> bool {
        let square_src = &self.board[*src];
        let square_dest = &self.board[*dest];

        if let [Some(p_source), Some(p_dest)] = [&square_src.0, &square_dest.0] {
            return p_source.p_color == p_dest.p_color;
        }

        false
    }
    pub fn handle_move(&mut self, src: &Position, dest: &Position) -> MyResult<()> {
        self.perform_move(src, dest)
    }
    pub fn is_check(&self, king_color: Color) -> bool {
        /*
         MAYBE ADD CACHING
        */

        let king_pos = match king_color {
            Color::White => self.white_king_pos,
            Color::Black => self.black_king_pos,
        };
        for (row_index, row) in self.board.iter_rows().enumerate() {
            for (column_index, square) in row.iter().enumerate() {
                if !square.is_empty() && !square.same_owner(king_color) {
                    let src = Position::new(row_index as i8, column_index as i8);

                    let p_type = square.0.unwrap().p_type;
                    let is_valid = pm::is_valid_move(&p_type, &src, &king_pos);
                    let is_clear = self.check_dest_path_is_clear(&src, &king_pos);

                    if is_valid && is_clear {
                        return true;
                    }
                }
            }
        }
        false
    }

    /// check that the move is valid, if piece dest is legal movement if not interrupted by anything

    /// check that the movement path of the  piece is clear, not blocked
    pub fn check_dest_path_is_clear(&self, src: &Position, dest: &Position) -> bool {
        let velocity = Velocity::new(src, dest);
        let mut curr_pos = Position::new(src.x, src.y);

        for _ in 0..BoardSizeInfo::row_count() {
            curr_pos.x += velocity.x;
            curr_pos.y += velocity.y;

            let current_square = &self.board[curr_pos];

            // todo!: check if it invalidate eating an enemy
            if curr_pos == *dest && !self.same_owner(src, &curr_pos) {
                break;
            }

            if current_square.0.is_some() {
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
                match &square.0 {
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

        for (row_number, row_value) in self.board.iter_rows().rev().enumerate() {
            write!(f, "{:>2}", BoardSizeInfo::row_count() - row_number).unwrap();

            for square in row_value {
                match &square.0 {
                    Some(piece) => write!(f, "{:>2}", piece)?,
                    None => write!(f, "{:>2}", "·")?,
                }
            }

            writeln!(f)?;
        }
        Ok(())
    }
}

impl BoardManager
{
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
                        board[(BoardSizeInfo::row_count() - 1 - line_number, empty_index)] =
                            Square::default();
                    }
                    current_line_index += fen_value - 1;
                } else if fen_value.is_ascii_alphabetic() {
                    let p_type = fen_value.into();
                    let p_color = fen_value.into();
                    let p_position = Position::new(
                        (BoardSizeInfo::row_count() - 1 - line_number) as i8,
                        current_line_index as i8,
                    );
                    match (p_type, p_color) {
                        (King, Color::White) => white_king_pos = Some(p_position),
                        (King, Color::Black) => black_king_pos = Some(p_position),
                        _ => {}
                    }
                    board[(
                        BoardSizeInfo::row_count() - (line_number + 1),
                        current_line_index,
                    )] = Square::new(Piece::new(p_type, p_color));
                    current_line_index += 1;
                }
            }
        }

        if white_king_pos.is_none() {
            return Err("White King Not Found")?;
        }
        if black_king_pos.is_none() {
            return Err("Black King Not Found")?;
        }

        let white_king_pos = white_king_pos.unwrap();
        let black_king_pos = black_king_pos.unwrap();
        Ok(KingsTracker {
            white_king_pos,
            black_king_pos,
        })
    }

    fn validate_move(&mut self, src: &Position, dest: &Position) -> MyResult<Piece>
    {
        let piece_source = &self.board[*src].0;
        piece_source.ok_or("Illegal Move, Can't Move An Empty Square")?;

        let dest_is_invalid = self.same_owner(src, dest);

        if dest_is_invalid {
            Err("Can't Eat The Same Color")?;
        }

        let piece_source = piece_source.unwrap();
        let valid_move = pm::is_valid_move(&piece_source.p_type, src, dest);

        if !valid_move {
            Err("Piece Can't Move That Way")?;
        }

        if !self.check_dest_path_is_clear(src, dest) {
            Err("That Piece Movement Path Is Blocked")?;
        }

        self.do_move_regardless(src, dest);

        let is_check = self.is_check(piece_source.p_color);

        self.undo_move_regardless();

        if is_check
        {
            Err("Can't Make A Move That Danger The King")?;
        }
        Ok(piece_source)
    }

    fn perform_move(&mut self, src: &Position, dest: &Position) -> MyResult<()> {
        let piece_source = self.validate_move(src, dest)?;

        match (piece_source.p_type, piece_source.p_color) {
            (King, Color::White) => self.white_king_pos = *dest,
            (King, Color::Black) => self.black_king_pos = *dest,
            _ => {}
        }

        self.do_move_regardless(src, dest);
        self.turns_counter += 1;
        Ok(())
    }

    /// undo any last move that have been done by regardless
    fn undo_move_regardless(&mut self) {
        let last_move = self.moves_tracker.pop_back().unwrap();

        self.board[last_move.move_src].0 = last_move.piece_eaten;

        self.board.swap(&last_move.move_src, &last_move.move_dest);
    }

    /// make a move even if not legal
    fn do_move_regardless(&mut self, src: &Position, dest: &Position)
    {
        let chess_move = ChessMove {
            move_src: *src,
            move_dest: *dest,
            piece_eaten: self.board[*dest].0.take(),
        };
        self.board.swap(src, dest);
        self.moves_tracker.push_back(chess_move)
    }
}


/// format algebraic notation alphabetic
pub fn algebraic_notation_letters_formatted(f: &mut Formatter) {
    write!(f, "{:>2}", " ").unwrap();

    for c in 'a'..='h' {
        write!(f, "{:>2}", c).unwrap();
    }
}

impl fmt::Display for BoardManager {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        algebraic_notation_letters_formatted(f);

        if self.turns_counter % 2 == 0 {
            self.output_white_front(f)?;
        } else {
            self.output_black_front(f)?;
        }

        algebraic_notation_letters_formatted(f);
        write!(f, "")
    }
}
