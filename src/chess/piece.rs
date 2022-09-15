use core::panic;
use std::convert::From;
use std::fmt;

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub struct Piece {
    pub p_type: PieceType,
    pub p_color: Color,
}

impl Piece {
    pub fn new(p_type: PieceType, p_color: Color) -> Self {
        Piece { p_type, p_color }
    }
}

impl fmt::Display for Piece {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.pad(&format!("{}", piece_to_char(&self.p_type, &self.p_color)))
    }
}

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
    Pawn,
    Knight,
    Bishop,
    Rook,
    Queen,
    King,
}

impl From<char> for PieceType {
    fn from(item: char) -> Self {
        match item {
            'p' | 'P' => PieceType::Pawn,
            'k' | 'K' => PieceType::King,
            'n' | 'N' => PieceType::Knight,
            'r' | 'R' => PieceType::Rook,
            'q' | 'Q' => PieceType::Queen,
            'b' | 'B' => PieceType::Bishop,
            _ => panic!("not a valid character for chess fen string"),
        }
    }
}

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub struct Position {
    pub x: i8,
    pub y: i8,
}

impl Position {
    pub fn new(x: i8, y: i8) -> Self {
        Position { x, y }
    }
}

impl From<char> for Piece {
    fn from(item: char) -> Self {
        let color = item.into();
        let piece_type = item.into();
        Piece::new(piece_type, color)
    }
}

pub fn piece_to_char(p_type: &PieceType, p_color: &Color) -> char {
    match p_type {
        PieceType::Queen => {
            if p_color == &Color::Black {
                '♛'
            } else {
                '♕'
            }
        }

        PieceType::Pawn => {
            if p_color == &Color::Black {
                '♟'
            } else {
                '♙'
            }
        }

        PieceType::Bishop => {
            if p_color == &Color::Black {
                '♝'
            } else {
                '♗'
            }
        }

        PieceType::King => {
            if p_color == &Color::Black {
                '♚'
            } else {
                '♔'
            }
        }

        PieceType::Knight => {
            if p_color == &Color::Black {
                '♞'
            } else {
                '♘'
            }
        }

        PieceType::Rook => {
            if p_color == &Color::Black {
                '♜'
            } else {
                '♖'
            }
        }
    }
}
