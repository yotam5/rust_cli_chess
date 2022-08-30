use cte::chess;

// Todo: need to add test for conversions char to Piece, char to Color and char to PieceType

fn generate_algebraic_notation_arrays() -> ([char; 8], [char; 8])
{
    let letters = ['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h'];
    let numbers = ['1', '2', '3', '4', '5', '6', '7', '8'];
    (numbers, letters)
}

#[test]
fn same_owner_test()
{
    use chess::parse::parse_algebraic_notation;
    use chess::board::Board;
    let board = Board::new();
    let test_data = [
        ((b'a', b'1'), (b'a', b'2'), true), // white to white
        ((b'a', b'8'), (b'a', b'7'), true), // black to black
        ((b'a', b'2'), (b'a', b'3'), false), // white to empty
        ((b'a', b'7'), (b'a', b'6'), false), //black to empty
        ((b'a', b'5'), (b'a', b'4'), false), // emtpy to empty
        ((b'a', b'2'), (b'a', b'7'), false) // white to black
    ];

    for (src, dest, expected) in test_data
    {
        let src_pos = parse_algebraic_notation(&[src.0, src.1]).unwrap();
        let dest_pos = parse_algebraic_notation(&[dest.0, dest.1]).unwrap();
        assert_eq!(board.same_owner(&src_pos, &dest_pos), expected);
    }
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
        assert!(result.is_err());
        let result = parse_algebraic_notation(&[c as u8, n as u8]);
        assert!(result.is_ok());
    }
}



