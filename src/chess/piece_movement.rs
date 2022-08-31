use super::piece::Position;

/// TODO: after finding the piece velocity direction return it converted to direction enum
/// then travers to there and check if its not blocked by other pieces or so
#[derive(Debug)]
pub struct Velocity
{
    pub x: isize,
    pub y: isize,
}

impl Velocity
{
    pub fn new(src: &Position, dest: &Position) -> Velocity
    {
        let x = dest.x - src.x;
        let y = dest.y - src.y;
        let mut scalar = Velocity::gcd(x, y);
        Velocity
        {
            x: x / scalar,
            y: y / scalar,
        }
    }

    pub fn gcd(x: isize, y: isize) -> isize
    {
        return if y == 0 { x.abs() } else {
            Velocity::gcd(y, x % y)
        };
    }
}

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

impl From<Velocity> for Directions
{
    fn from(item: Velocity) -> Self {
        match (item.x, item.y) {
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
    ((src.y - dest.y).abs() / (src.x - dest.x).abs()) == 1
}

pub fn is_valid_queen_move(src: &Position, dest: &Position) -> bool
{
    is_valid_rook_move(src, dest) || is_valid_bishop_move(src, dest)
}

pub fn is_valid_king_move(src: &Position, dest: &Position) -> bool
{
    (src.x - dest.x).abs() <= 1 && (src.y - dest.y) <= 1
}