use std::error::Error;
use super::piece::{Position};
// NOTE: CAPLOCK + K is like mouse press

type MyResult<T> = Result<T, Box<dyn Error>>;

/// check if algebraic notation is valid
pub fn is_valid_algebraic_notation(an_arr: &[u8; 2]) -> bool {
    let row = an_arr[0];
    let column = an_arr[1];

    if !(row.is_ascii_alphabetic() && row.is_ascii_lowercase() && column.is_ascii_digit()) {
        return false;
    }

    if !(['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h'].contains(&(row as char))
        && (column as char).to_digit(10).unwrap() <= 8_u32)
    {
        return false;
    }

    true
}

/// parse algebraic notation to Position on board note the subtraction of 1
/// for board array indexing that starts with 0
pub fn parse_algebraic_notation(an_arr: &[u8; 2]) -> MyResult<Position> {
    if is_valid_algebraic_notation(an_arr) {
        let row = an_arr[0] - b'a';
        let column = (an_arr[1] as char).to_digit(10).unwrap() - 1;
        return Ok(Position::new(column as isize, row as isize));
    }
    Err("Invalid algebraic notation for piece location")?
}






