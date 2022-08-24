#[derive(Debug, Eq, PartialEq)]

pub enum Color {
    Black,
    White,
}
pub enum PieceType {
    Pawn(Color),
    Knight(Color),
    Bishop(Color),
    Rook(Color),
    Queen(Color),
}

pub trait Piece {
    fn piece_type(&self) -> &PieceType;
    fn location(&self) -> &(usize, usize);
}
