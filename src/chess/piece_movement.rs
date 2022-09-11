use super::piece::Position;

/// TODO: after finding the piece velocity direction return it converted to direction enum
/// then travers to there and check if its not blocked by other pieces or so
#[derive(Debug)]
/// struct to contain piece velocity, movement vector.
/// for example a Knight vector may be x: -2 y: -1 scalar: 1
/// if it move left down
pub struct Velocity
{
    pub x: isize,
    pub y: isize,
    pub scalar: isize,
}

impl Velocity
{
    pub fn new(src: &Position, dest: &Position) -> Velocity
    {
        let x = dest.x - src.x;
        let y = dest.y - src.y;
        let scalar = Velocity::gcd(x, y);
        Velocity
        {
            x: x / scalar,
            y: y / scalar,
            scalar,
        }
    }

    /// find the greatest common divisor
    pub fn gcd(x: isize, y: isize) -> isize
    {
        if y == 0 { return x.abs(); }
        Velocity::gcd(y, x % y)
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

/// check if pawn move is valid, using the fact that pawn can move
/// diagonally 1 square to eat, 1 square vertically or 2 if moved the first time
pub fn is_valid_pawn_move(src: &Position, dest: &Position) -> bool
{
    let v = Velocity::new(src, dest);
    let vx_abs = v.x.abs();
    let vy_abs = v.y.abs();
    let scalar_abs = v.scalar.abs();

    // move 2 times if moved for the first time
    if scalar_abs == 2 {
        // the rows that pawn begin with, relative index 1 is index 0
        return [1, 6].contains(&src.x);
    } else if scalar_abs == 1
    {
        // pawn absolute movement vectors, refer to direction for better understanding
        // moves either 1 on x and y or only on y
        if [(1, 1), (1, 0)].contains(&(vx_abs, vy_abs))
        {
            return true;
        }
        return false;
    }
    false
}

/// check if rook move is valid as it can move only in strait lines
pub fn is_valid_rook_move(src: &Position, dest: &Position) -> bool
{
    src.x == dest.x || src.y == dest.y
}

/// check if bishop is valid as it can move only diagonally
pub fn is_valid_bishop_move(src: &Position, dest: &Position) -> bool
{
    ((src.y - dest.y).abs() / (src.x - dest.x).abs()) == 1
}

/// check if queen move is valid as it can move both as Bishop and Rook
pub fn is_valid_queen_move(src: &Position, dest: &Position) -> bool
{
    is_valid_rook_move(src, dest) || is_valid_bishop_move(src, dest)
}

/// check if king move is valid as it can move like Queen but only one square
pub fn is_valid_king_move(src: &Position, dest: &Position) -> bool
{
    (src.x - dest.x).abs() <= 1 && (src.y - dest.y) <= 1
}