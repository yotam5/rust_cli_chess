use core::panic;
use std::convert::From;
use std::fmt;

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum Directions {
    Up,
    Down,
    Left,
    Right,
    UpLeft,
    UpRight,
    DownLeft,
    DownRight,
}


impl From<(i32, i32)> for Directions {
    fn from(item: (i32, i32)) -> Directions {
        match (item.0, item.1) {
            (0, -1) => Directions::Up,
            (0, 1) => Directions::Down,
            (1, 0) => Directions::Right,
            (-1, -1) => Directions::UpLeft,
            (1, -1) => Directions::UpRight,
            (1, 1) => Directions::DownRight,
            (-1, 1) => Directions::DownLeft,
            (-1, 0) => Directions::Left,
            _ => unreachable!(),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum Movment {}

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum Color {
    Black,
    White,
}

impl From<char> for Color {
    fn from(item: char) -> Self {
        if item.is_lowercase() {
            Color::Black
        } else {
            Color::White
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Copy)]
pub enum PieceType {
    Pawn(Color),
    Knight(Color),
    Bishop(Color),
    Rook(Color),
    Queen(Color),
    King(Color),
}

impl From<char> for PieceType {
    fn from(item: char) -> Self {
        let color: Color = item.into();
        match item {
            'p' | 'P' => PieceType::Pawn(color),
            'k' | 'K' => PieceType::King(color),
            'n' | 'N' => PieceType::Knight(color),
            'r' | 'R' => PieceType::Rook(color),
            'q' | 'Q' => PieceType::Queen(color),
            'b' | 'B' => PieceType::Bishop(color),
            _ => panic!("not a valid character for chess fen string"),
        }
    }
}

impl PieceType {
    pub fn move_to(&self) {
        unimplemented!()
    }
}

impl fmt::Display for PieceType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let result;
        match *self {
            Self::Queen(color) => result = if color == Color::Black { "♛" } else { "♕" },

            Self::Pawn(color) => result = if color == Color::Black { "♟" } else { "♙" },

            Self::Bishop(color) => result = if color == Color::Black { "♝" } else { "♗" },

            Self::King(color) => result = if color == Color::Black { "♚" } else { "♔" },

            Self::Knight(color) => result = if color == Color::Black { "♞" } else { "♘" },

            Self::Rook(color) => result = if color == Color::Black { "♜" } else { "♖" },
        }
        f.pad(result)
    }
}
