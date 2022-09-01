use std::ops::{Index, IndexMut};
use std::iter::Iterator;
use super::piece::Piece;

const BOARD_WIDTH: usize = 8;
const BOARD_HEIGHT: usize = 8;


pub type Square = Option<Piece>;

#[derive(Debug)]
pub struct Board<'a>([Square; 64]);

impl<'a> Board<'a>
{
    pub fn new() -> Self
    {

        Board([Square::None; 64])
    }

    fn valid_index((row, column): (usize, usize)) -> bool
    {
        row <= BOARD_WIDTH && column <= BOARD_HEIGHT
    }

}

impl<'a> Default for Board<'a>
{
    fn default() -> Self {
        Board::new()
    }
}

impl<'a> Index<(usize, usize)> for Board<'a>
{
    type Output = Square;

    fn index(&self, (row, column): (usize, usize)) -> &Self::Output
    {
        assert!(Board::valid_index((row, column)));
        &self.0[BOARD_WIDTH * row + column]
    }
}

impl<'a> Iterator for Board<'a>
{
    type Item = &'a [Square];


} 

impl<'a> Index<(usize, usize)> for &Board<'a>
{
    type Output = Square;

    fn index(&self, (row, column): (usize, usize)) -> &Self::Output
    {
        assert!(Board::valid_index((row, column)));
        &self.0[BOARD_WIDTH * row + column]
    }
}

impl<'a> IndexMut<(usize, usize)> for &Board<'a>
{
    fn index_mut(&mut self, (row, column): (usize, usize)) -> &mut Self::Output {
        assert!(Board::valid_index((row, column)));
        self.index_mut((row,column))
    }
}

impl<'a> IndexMut<(usize, usize)> for Board<'a>
{
    fn index_mut(&mut self, (row, column): (usize, usize)) -> &mut Self::Output {
        assert!(Board::valid_index((row, column)));
        &mut self.0[BOARD_WIDTH * row + column]
    }
}