use super::piece;


/// check if knight move is valid, using distance from it source to dest for 
/// all of the valid moves that have the constancy
pub fn is_valid_kight_move(src:&piece::Position, dest: &piece::Position) -> bool 
{
    let valid_option_a= (src.x - dest.x).abs() == 1 && (src.y - dest.y).abs() == 2;
    let valid_option_b = (src.x - dest.y).abs() == 2 && (src.y - dest.y).abs() == 1;

    valid_option_a || valid_option_b
}