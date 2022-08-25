use super::piece;
use std::fmt;
#[derive(Default)]

enum Square {
    Contains(piece::PieceType),
    #[default]
    Empty,
}

pub struct Board {
    board: [[Square; 8]; 8],
}

fn initialize_board(board_array: &mut [[Square; 8]; 8]) {
    let initial_game_position = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR";
    load_fen_string_to_board(board_array, &initial_game_position);
}

fn load_fen_string_to_board(board_array: &mut [[Square; 8]; 8], fen_string: &str) {
    for (line_number, line_fen_value) in fen_string.split("/").enumerate() {
        let mut current_line_index: u8 = 0;

        for fen_value in line_fen_value.chars() {
            if fen_value.is_numeric() {
                let fen_value = fen_value.to_digit(10).unwrap() as u8;
                for empty_index in current_line_index..fen_value {
                    board_array[line_number][usize::from(empty_index)] = Square::Empty;
                }
                current_line_index += fen_value - 1;
            } else if fen_value.is_ascii_alphabetic() {
                let piece: piece::PieceType = fen_value.into();
                board_array[line_number][usize::from(current_line_index)] = Square::Contains(piece);
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
}

impl<'a> fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:>2}", " ").unwrap();

        for c in 'a'..='h' {
            write!(f, "{:>2}", c).unwrap();
        }

        writeln!(f, "").unwrap();

        for (row_number, row_value) in self.board.iter().enumerate() {
            write!(f, "{:>2}", 8 - row_number).unwrap();

            for square in row_value {
                match square {
                    Square::Empty => write!(f, "{:>2}", "·").unwrap(),
                    Square::Contains(p) => {
                        write!(f, "{:>2}", p).unwrap();
                    }
                }
            }
            writeln!(f, "").unwrap();
        }
        write!(f, "")
    }
}
