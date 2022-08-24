use super::piece::{self, PieceType};


pub struct Pawn
{
    piece_type: piece::PieceType,
}

impl Pawn{
    pub fn new(piece_type: PieceType) -> Self
    {
        Pawn {
            piece_type,
        }
    }
}

impl piece::Piece for Pawn
{
    fn piece_type(&self) -> &piece::PieceType {
        &self.piece_type
    } 

    fn location(&self) -> &(usize, usize) {
        unimplemented!()
    }
}