use std::error::Error;
use super::piece::{self, Position};
type MyResult<T> = Result<T, Box<dyn Error>>;

fn is_valid_algebric_notation(an_arr: &[u8; 2]) -> bool {
    let row = an_arr[0];
    let column = an_arr[1];

    if !(row.is_ascii_alphabetic() && row.is_ascii_lowercase() && column.is_ascii_digit()) {
        return false;
    }

    if !(['a', 'b', 'c', 'd', 'f', 'g'].contains(&(row as char))
        && (column as char).to_digit(10).unwrap() < 9_u32)
    {
        return false;
    }

    true
}

fn parse_algebric_notation(an_arr: &[u8; 2]) -> MyResult<piece::Position> {
    if is_valid_algebric_notation(an_arr) {
        let row = an_arr[0] - ('a' as u8);
        let column = (an_arr[1] as char).to_digit(10).unwrap();
        return Ok(Position::new(row as i8, column as i8));
    }
    Err("Invalid algebric notation for piece location")?


}