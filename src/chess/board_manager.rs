use std::fmt;
use super::piece_movement::{Directions,Velocity};
use super::piece::{Position, Piece, PieceType};
use super::piece_movement as pm;
use super::board::{Board,Square};

pub struct BoardManager<'a> {
    pub board: &'a Board<'a>,
    turns_counter: usize,
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
                    board_array[(7 - line_number, empty_index)] = None;
                }
                current_line_index += fen_value - 1;
            } else if fen_value.is_ascii_alphabetic() {
                board_array
                    [(7 - line_number, current_line_index)] = Some(Piece::new(
                    fen_value.into(),
                    fen_value.into(),
                    Position::new((7 - line_number) as isize, current_line_index as isize),
                ));
                current_line_index += 1;
            }
        }
    }
}

impl<'a> BoardManager<'a> {
    pub fn new() -> Self {
        let mut board= Board::new();
        initialize_board(&mut board);
        BoardManager { board, turns_counter: 0 }
    }

    pub fn same_owner(&self, src: &Position, dest: &Position) -> bool
    {
        let square_src = self.board[src.into()];
        let square_dest = self.board[(*dest).into()];

        if let (Some(ss), Some(sd)) = (square_src, square_dest)
        {
            return ss.p_color == sd.p_color;
        }

        false
    }

    pub fn handle_move(&mut self, src: &Position, dest: &Position) -> bool
    {

        /*if self.square_at(src).is_some()
            && !self.same_owner(src, dest) //note replace king and rook need fix
        {
            let piece_type = &self.square_at(src).as_ref().unwrap().p_type;
            if !Board::is_valid_move(piece_type, src, dest) || !self.check_dest_path_is_clear(src, dest)
            {
                return false;
            }
            println!("path is clear {}", self.check_dest_path_is_clear(src, dest));
            self.turns_counter += 1;

            //mem::swap(k,w);
            return true;
        }*/
        false
    }

    pub fn is_valid_move(piece_type: &PieceType, src: &Position, dest: &Position) -> bool
    {
        return match piece_type {
            PieceType::Knight => pm::is_valid_knight_move(src, dest),
            PieceType::Bishop => pm::is_valid_bishop_move(src, dest),
            PieceType::Queen => pm::is_valid_queen_move(src, dest),
            PieceType::Rook => pm::is_valid_rook_move(src, dest),
            _ => unimplemented!()
        };
    }

    fn valid_position_on_board(pos: &Position) -> bool
    {
        pos.x >= 0 && pos.y >= 0
    }

    pub fn check_dest_path_is_clear(&self, src: &Position, dest: &Position) -> bool
    {
        let velocity = Velocity::new(src, dest);
        let mut curr_pos = Position::new(src.x, src.y);

        for _ in 0..8
        {
            curr_pos.x += velocity.x;
            curr_pos.y += velocity.y;
            //assert!(Board::valid_position_on_board(&curr_pos));
            if self.board[curr_pos.into()].is_some(){
                return false;
            }

            if &curr_pos == dest
            {
                break;
            }
        }

        true
    }

    pub fn output_black_front(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f)?;
        //param - (row_number % 8) + 1)
        let black_front = |row_number: &usize| -> usize{
            row_number + 1
        };

        /*for (row_number, row_value) in self.board.iter().enumerate() {
            write!(f, "{:>2}", black_front(&row_number)).unwrap();

            for square in row_value {
                if square.is_none() { write!(f, "{:>2}", "·")? } else if square.is_some() {
                    write!(f, "{:>2}", square.as_ref().unwrap()).unwrap();
                }
            }

            writeln!(f)?;
        }*/
        Ok(())
    }

    pub fn output_white_front(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f)?;
        //param - (row_number % 8) + 1)
        /*for (row_number, row_value) in self.board.iter().enumerate().rev() {
            write!(f, "{:>2}", row_number + 1).unwrap();

            for square in row_value {
                if square.is_none() { write!(f, "{:>2}", "·")? } else if square.is_some() {
                    write!(f, "{:>2}", square.as_ref().unwrap()).unwrap();
                }
            }

            writeln!(f)?;
        }*/
        Ok(())
    }
}


pub fn algebraic_notation_letters_formatted(f: &mut fmt::Formatter)
{
    write!(f, "{:>2}", " ").unwrap();

    for c in 'a'..='h' {
        write!(f, "{:>2}", c).unwrap();
    }
}

impl<'a> fmt::Display for BoardManager<'a> {
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