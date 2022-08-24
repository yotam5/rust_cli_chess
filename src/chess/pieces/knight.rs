use super::piece::{self, PieceType};

pub struct Knight {
    piece_type: piece::PieceType,
}

impl Knight {
    pub fn new(piece_type: PieceType) -> Self {
        Knight { piece_type }
    }
}

impl piece::Piece for Knight {
    fn piece_type(&self) -> &piece::PieceType {
        &self.piece_type
    }

    fn location(&self) -> &(usize, usize) {
        unimplemented!()
    }
}
