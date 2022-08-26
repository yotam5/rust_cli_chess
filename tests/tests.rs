use cte::chess;

// Todo: need to add test for conversions char to Piece, char to Color and char to PieceType

fn generate_algebraic_notation_arrays() -> ([char; 8], [char; 8])
{
    let letters = ['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h'];
    let numbers = ['1', '2', '3', '4', '5', '6', '7', '8'];
    (numbers, letters)
}

#[test]
fn algebraic_notation_test()
{
    use chess::parse::*;
    let (num_arr, char_arr) = generate_algebraic_notation_arrays();
    for (n, c) in num_arr.into_iter().zip(char_arr.into_iter())
    {
        let k = &[n as u8, c as u8];
        let result = parse_algebraic_notation(&[n as u8, c as u8]);
        assert_eq!(result.is_err(), true);
        let result = parse_algebraic_notation(&[c as u8, n as u8]);
        assert_eq!(result.is_ok(), true);
    }
}

