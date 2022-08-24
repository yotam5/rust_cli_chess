use super::pieces::piece;
use std::fmt;

#[derive(Default)]

enum Square<'a> {
    Contains(&'a dyn piece::Piece),
    #[default]
    Empty,
}

pub struct Board<'a> {
    board: [[Square<'a>; 8]; 8],
}

fn create_initial_board(board_array: &mut [[Square; 8]; 8]) {}

impl<'a> Board<'a> {
    pub fn new() -> Self {
        let mut board_array: [[Square; 8]; 8] = Default::default();
        create_initial_board(&mut board_array);
        Board { board: board_array }
    }
}

impl<'a> fmt::Display for Board<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:>2}", " ").unwrap();

        for c in 'a'..='h' {
            write!(f, "{:>2}", c).unwrap();
        }

        writeln!(f, "").unwrap();

        for (row_number, row_value) in self.board.iter().enumerate() {
            write!(f, "{:>2}", row_number + 1).unwrap();

            for square in row_value {
                match square {
                    Square::Empty => write!(f, "{:>2}", "♙").unwrap(),
                    Square::Contains(n) => {
                        unreachable!()
                    }
                }
            }
            writeln!(f, "").unwrap();
        }
        write!(f, "")
    }
}
