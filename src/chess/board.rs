use std::ops::{Index, IndexMut};
use super::piece::Piece;

const BOARD_WIDTH: usize = 8;
const BOARD_HEIGHT: usize = 8;


pub type Square = Option<Piece>;

pub struct Board([Square; 64]);

impl Board
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

impl Default for Board
{
    fn default() -> Self {
        Board::new()
    }
}

impl Index<(usize, usize)> for Board
{
    type Output = Square;

    fn index(&self, (row, column): (usize, usize)) -> &Self::Output
    {
        assert!(Board::valid_index((row, column)));
        &self.0[BOARD_WIDTH * row + column]
    }
}

impl Index<(usize, usize)> for &Board
{
    type Output = Square;

    fn index(&self, (row, column): (usize, usize)) -> &Self::Output
    {
        self.index((row,column))
    }
}

impl IndexMut<(usize, usize)> for &Board
{
    fn index_mut(&mut self, (row, column): (usize, usize)) -> &mut Self::Output {
        self.index_mut((row,column))
    }
}

impl IndexMut<(usize, usize)> for Board
{
    fn index_mut(&mut self, (row, column): (usize, usize)) -> &mut Self::Output {
        assert!(Board::valid_index((row, column)));
        &mut self.0[BOARD_WIDTH * row + column]
    }
}