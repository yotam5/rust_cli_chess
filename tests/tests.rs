use cte::chess;
use cte::chess::parse::parse_algebraic_notation;
use cte::chess::piece::{PieceType,Color, Position};
use cte::chess::piece_movement::{is_valid_king_move, is_valid_knight_move, is_valid_pawn_move, is_valid_queen_move, is_valid_rook_move};

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
    use chess::board_manager::BoardManager;
    let board = BoardManager::new();
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
        let result = parse_algebraic_notation(&[n as u8, c as u8]);
        assert!(result.is_err());
        let result = parse_algebraic_notation(&[c as u8, n as u8]);
        assert!(result.is_ok());
    }
}

#[test]
fn test_knight_moves()
{
    let src_position = (b'e', b'4');
    let src_position = parse_algebraic_notation(&[src_position.0, src_position.1]).unwrap();
    let valid_destinations = [
        (b'c', b'3'), (b'c', b'5'), (b'd', b'2'),
        (b'd', b'6'), (b'f', b'2'), (b'f', b'6'),
        (b'g', b'3'), (b'g', b'5')
    ];

    for dest in valid_destinations
    {
        let dest_position = parse_algebraic_notation(&[dest.0, dest.1]).unwrap();
        assert_eq!(is_valid_knight_move(&src_position, &dest_position), true);
    }
}

#[test]
fn test_pawn_moves()
{
    let src_position = (b'b', b'2');
    let src_position = parse_algebraic_notation(&[src_position.0, src_position.1]).unwrap();

    let valid_destinations = [
        (b'a', b'3'), (b'b', b'3'), (b'c', b'3'), (b'b', b'4')
    ];

    for dest in valid_destinations {
        // println!("dest fo: {:?}-{:?}",dest.0 as char, dest.1 as char);
        let dest_position = parse_algebraic_notation(&[dest.0, dest.1]).unwrap();
        println!("test: src: {:?} test: dest: {:?}",&src_position,&dest_position);
        assert!(is_valid_pawn_move(&src_position, &dest_position));
    }
}

#[test]
fn test_queen_moves()
{
    let src_position = (b'f', b'3');
    let src_position = parse_algebraic_notation(&[src_position.0, src_position.1]).unwrap();

    let valid_destinations = [
        (b'f', b'6'), (b'f', b'1'), (b'a', b'8'),
        (b'd', b'1'), (b'h', b'1'), (b'h', b'3'),
        (b'h', b'5'), (b'e', b'3')
    ];

    for dest in valid_destinations
    {
        let dest_position = parse_algebraic_notation(&[dest.0, dest.1]).unwrap();
        assert!(is_valid_queen_move(&src_position, &dest_position));
    }


}

#[test]
fn test_king_moves()
{
    let src_position = (b'f', b'3');
    let src_position = parse_algebraic_notation(&[src_position.0, src_position.1]).unwrap();

    let valid_destinations = [
        (b'f', b'4'), (b'f', b'2'), (b'e', b'4'),
        (b'e', b'2'), (b'g', b'2'), (b'g', b'3'),
        (b'g', b'4'), (b'e', b'3')
    ];
    let invalid_destinations = [
        (b'f',b'5'),(b'f',b'1'),(b'd',b'3'),
        (b'h',b'3'),(b'd',b'5'),(b'h',b'1'),
        (b'h',b'5'),(b'd',b'1')
    ];
    for dest in valid_destinations
    {
        let dest_position = parse_algebraic_notation(&[dest.0, dest.1]).unwrap();
        assert!(is_valid_king_move(&src_position, &dest_position));
    }
    for dest in invalid_destinations{
    }
}

pub fn move_validation_helper(arr_of_moves: &[(u8,u8)],p_type: PieceType, 
    p_source: Position,bv: bool)
{
    use PieceType::*;
        for dest in arr_of_moves
        {
            let dest_position = parse_algebraic_notation(&[dest.0, dest.1]).unwrap();
            match p_type
            {
                King => assert_eq!(is_valid_king_move(&p_source,&dest_position),bv),
                _ => unimplemented!()
            };
        }
}

#[test]
fn test_rook_moves()
{
    let src_position = (b'e', b'4');
    let src_position = parse_algebraic_notation(&[src_position.0, src_position.1]).unwrap();

    let valid_destinations = [
        (b'a', b'4'), (b'e', b'1'), (b'h', b'4'), (b'e', b'8')
    ];
    let invalid_destinations = [
        (b'a',)
    ] 
    for dest in valid_destinations
    {
        let dest_position = parse_algebraic_notation(&[dest.0, dest.1]).unwrap();
        assert!(is_valid_rook_move(&src_position, &dest_position));
    }
}



























