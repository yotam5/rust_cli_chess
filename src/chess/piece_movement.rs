use super::piece::Position;


/// check if knight move is valid, using distance from it source to dest for 
/// all of the valid moves that have the constancy
pub fn is_valid_knight_move(src: &Position, dest: &Position) -> bool
{
    ((src.x - dest.x) * (src.y - dest.y)).abs() == 2
}

pub fn is_valid_rook_move(src: &Position, dest: &Position) -> bool
{
    src.x == dest.x || src.y == dest.y
}

pub fn is_valid_bishop_move(src: &Position, dest: &Position) -> bool
{
    ((src.y - dest.y).abs()/(src.x - dest.x).abs()) == 1
}

pub fn is_valid_queen_move(src: &Position, dest: &Position) -> bool
{
    is_valid_bishop_move(src, dest) || is_valid_rook_move(src, dest)
}

pub fn is_valid_king_move(src: &Position, dest: &Position) -> bool
{
    (src.x - dest.x).abs() <= 1 && (src.y - dest.y) <= 1
}