use std::fmt;

use array2ds::array2d::Array2d;
use array2ds::array2d::GridIdx;
use super::piece::{Color, Piece, PieceType, Position};
use super::piece_movement as pm;
use super::piece_movement::Velocity;

struct BoardSizeInfo();

impl BoardSizeInfo
{
    pub fn row_count() -> usize { 8_usize }
    pub fn column_count() -> usize { 8_usize }
}

type Board = Array2d<Square>;

#[derive(Debug)]
struct Square
{
    square_color: Color,
    piece_on_square: Option<Piece>,
}

impl Square
{
    pub fn new_empty() -> Self
    {
        Square {
            square_color: Color::White,
            piece_on_square: None,
        }
    }

    pub fn new_contains(piece_info: Piece) -> Self
    {
        Square {
            square_color: Color::White,
            piece_on_square: Some(piece_info),
        }
    }
}

impl Default for Square
{
    fn default() -> Self {
        Square::new_empty()
    }
}

pub struct BoardManager {
    board: Board,
    turns_counter: usize,
}

impl GridIdx for Position {
    fn no_row(&self) -> usize {
        self.x as usize
    }

    fn no_column(&self) -> usize {
        self.y as usize
    }
}

/// load starting position for the chess game
fn initialize_board(board_array: &mut Board) {
    let initial_game_position = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR";
    load_fen_string_to_board(board_array, initial_game_position);
}

/// load fen string to the board
fn load_fen_string_to_board(board_array: &mut Board, fen_string: &str) {
    for (line_number, line_fen_value) in fen_string.split('/').enumerate() {
        let mut current_line_index: usize = 0;

        for fen_value in line_fen_value.chars() {
            if fen_value.is_numeric() {
                let fen_value = fen_value.to_digit(10).unwrap() as usize;
                for empty_index in current_line_index..fen_value {
                    board_array[(BoardSizeInfo::row_count() - 1 - line_number, empty_index)]
                        = Square::new_empty();
                }
                current_line_index += fen_value - 1;
            } else if fen_value.is_ascii_alphabetic() {
                board_array
                    [(BoardSizeInfo::row_count() - (line_number + 1), current_line_index)] =
                    Square::new_contains(
                        Piece::new(
                            fen_value.into(),
                            fen_value.into(),
                            Position::new
                                ((BoardSizeInfo::row_count() - 1 - line_number) as isize,
                                 current_line_index as isize),
                        ));
                current_line_index += 1;
            }
        }
    }
}

impl Default for BoardManager
{
    fn default() -> Self {
        let mut board = Board::filled_with_default(
            BoardSizeInfo::row_count(),
            BoardSizeInfo::column_count());

        initialize_board(&mut board);
        BoardManager { board, turns_counter: 0 }
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
        [&square_src.piece_on_square, &square_dest.piece_on_square]
        {
            return p_source.p_color == p_dest.p_color;
        }

        false
    }

    /// handle a chess move and  return bool if performed or not
    pub fn handle_move(&mut self, src: &Position, dest: &Position) -> bool
    {
        let piece_source = &self.board[*src].piece_on_square;

        let source_is_valid = piece_source.is_some();
        let dest_is_valid = self.same_owner(src, dest);

        if !source_is_valid || dest_is_valid {
            return false;
        }

        let piece_source = &piece_source.as_ref().unwrap();

        let is_valid_move = BoardManager::is_valid_move(&piece_source.p_type, src, dest);
        println!("mov valid: {}", &is_valid_move);
        if !(is_valid_move && self.check_dest_path_is_clear(src, dest))
        {
            return false;
        }

        self.turns_counter += 1;
        self.board.swap(src, dest);
        true
    }

    /// check that the move is valid, if piece dest is legal movement if not interrupted by anything
    pub fn is_valid_move(piece_type: &PieceType, src: &Position, dest: &Position) -> bool
    {
        match piece_type {
            PieceType::Knight => pm::is_valid_knight_move(src, dest),
            PieceType::Bishop => pm::is_valid_bishop_move(src, dest),
            PieceType::Queen => pm::is_valid_queen_move(src, dest),
            PieceType::Rook => pm::is_valid_rook_move(src, dest),
            PieceType::Pawn => pm::is_valid_pawn_move(src, dest),
            PieceType::King => pm::is_valid_king_move(src, dest),
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

            if current_square.piece_on_square.is_some()
            {
                return false;
            }
        }

        true
    }

    /// display the black in the front
    pub fn output_black_front(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f)?;

        for (row_number, row_value) in self.board.iter_rows().enumerate() {
            write!(f, "{:>2}", row_number + 1)?;

            for square in row_value.iter() {
                match &square.piece_on_square
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
    pub fn output_white_front(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f)?;

        for (row_number, row_value) in self.board.iter_rows().rev().enumerate()
        {
            write!(f, "{:>2}", BoardSizeInfo::row_count() - row_number).unwrap();

            for square in row_value
            {
                match &square.piece_on_square
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
pub fn algebraic_notation_letters_formatted(f: &mut fmt::Formatter)
{
    write!(f, "{:>2}", " ").unwrap();

    for c in 'a'..='h' {
        write!(f, "{:>2}", c).unwrap();
    }
}

impl fmt::Display for BoardManager {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
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