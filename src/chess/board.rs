use std::fmt;
use crate::chess::piece;

use super::piece::{Position, Piece};
use super::piece_movement as pm;

type Square = Option<Piece>;

pub struct Board {
    board: [[Square; 8]; 8],
}

/// load starting position for the chess game
fn initialize_board(board_array: &mut [[Square; 8]; 8]) {
    let initial_game_position = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR";
    load_fen_string_to_board(board_array, initial_game_position);
}

/// load fen string to the board
fn load_fen_string_to_board(board_array: &mut [[Square; 8]; 8], fen_string: &str) {
    for (line_number, line_fen_value) in fen_string.split('/').enumerate() {
        let mut current_line_index: usize = 0;

        for fen_value in line_fen_value.chars() {
            if fen_value.is_numeric() {
                let fen_value = fen_value.to_digit(10).unwrap() as usize;
                for empty_index in current_line_index..fen_value {
                    board_array[line_number][empty_index] = None;
                }
                current_line_index += fen_value - 1;
            } else if fen_value.is_ascii_alphabetic() {
                board_array[line_number][current_line_index] = Some(Piece::new(
                    fen_value.into(),
                    fen_value.into(),
                    Position::new(line_number as isize, current_line_index as isize),
                ));
                current_line_index += 1;
            }
        }
    }
}

impl Board {
    pub fn new() -> Self {
        let mut board_array: [[Square; 8]; 8] = Default::default();
        initialize_board(&mut board_array);
        Board { board: board_array }
    }

    /// in normal chess you index positions col row
    fn square_at(&self, dest: &Position) -> &Square
    {
        &self.board[dest.y as usize][dest.x as usize]
    }

    fn square_at_mut(&mut self, dest: &Position) -> &mut Square
    {
        &mut self.board[dest.y as usize][dest.x as usize]
    }

    pub fn same_owner(&self, src: &Position, dest: &Position) -> bool
    {
        let square_src = self.square_at(src);
        let square_dest = self.square_at(dest);

        if let (Some(ss), Some(sd)) = (square_src, square_dest)
        {
            return ss.p_color == sd.p_color;
        }

        false
    }

    pub fn handle_move(&mut self, src: &Position, dest: &Position) -> bool
    {
        if self.square_at(src).is_some()
            && !self.same_owner(src, dest) //note replace king and rook need fix
        {
            let piece_type = &self.square_at(src).as_ref().unwrap().p_type;
            match piece_type
            {
                piece::PieceType::Knight =>
                    {
                        let res = pm::is_valid_knight_move(src, dest);
                        if res {
                            println!("valid move");
                        }
                    }

                _ => unimplemented!()
            }
        }

        true
    }


//fn get_value_at(&self) ->
}

pub fn algebraic_notation_letters_formatted(f: &mut fmt::Formatter)
{
    write!(f, "{:>2}", " ").unwrap();

    for c in 'a'..='h' {
        write!(f, "{:>2}", c).unwrap();
    }
}

impl Default for Board
{
    fn default() -> Self {
        Board::new()
    }
}

impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        algebraic_notation_letters_formatted(f);
        writeln!(f)?;

        for (row_number, row_value) in self.board.iter().enumerate() {
            write!(f, "{:>2}", 8 - row_number).unwrap();

            for square in row_value {
                if square.is_none() { write!(f, "{:>2}", "·")? } else if square.is_some() {
                    write!(f, "{:>2}", square.as_ref().unwrap()).unwrap();
                }
            }

            writeln!(f)?;
        }


        write!(f, "{:>2}", " ")?;
        for c in 'a'..='h' {
            write!(f, "{:>2}", c)?;
        }
        write!(f, "")
    }
}
