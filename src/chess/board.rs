use super::piece::Piece;
use std::iter::{Iterator,IntoIterator};
use std::ops::{Index, IndexMut};

const BOARD_WIDTH: usize = 8;
const BOARD_HEIGHT: usize = 8;

pub type Square = Option<Piece>;

#[derive(Debug)]
pub struct Board {
    board_array: [Square; BOARD_HEIGHT  * BOARD_HEIGHT], 
    index: usize,
}

pub struct IterBoard<'a>
{
    inner: &'a ,
    index: usize,
}

impl<'a> Iterator for IterBoard<'a> {
    type Item = &'a Square;

    fn next(&mut self) -> Option<Self::Item> {
       if self.index >= self.inner.len()
       {
        return None
       } 

       self.inner
       
    }
}


impl Default for Board {
    fn default() -> Self {
        Board {
            board_array: [Square::None; 64],
            index: 0,
        }
    }
}

impl Board {
    pub fn new() -> Self {
        Board::default()
    }

    fn valid_index((row, column): (usize, usize)) -> bool {
        row <= BOARD_WIDTH && column <= BOARD_HEIGHT
    }
}

/// implement indexing the board using [(row,column)] notation
impl Index<(usize, usize)> for Board {
    type Output = Square;

    fn index(&self, (row, column): (usize, usize)) -> &Self::Output {
        assert!(Board::valid_index((row, column)));
        &self.board_array[BOARD_WIDTH * row + column]
    }
}

impl Iterator for Board {
    type Item = Square;

    fn next(&mut self) -> Option<Self::Item> {
        Some(self.board_array[0])
    }
}

/// implement the same as Index but for board reference
impl Index<(usize, usize)> for &Board {
    type Output = Square;

    fn index(&self, (row, column): (usize, usize)) -> &Self::Output {
        assert!(Board::valid_index((row, column)));
        &self.board_array[BOARD_WIDTH * row + column]
    }
}

/*impl IndexMut<(usize, usize)> for &Board {
    fn index_mut(&'a mut self, (row, column): (usize, usize)) -> &'a mut Self::Output {
        assert!(Board::valid_index((row, column)));
        //self.index_mut((row, column)) ///!ERROR: RCURSIVE BRUH

}
*/

/// implement the same as Index but as mutable
impl IndexMut<(usize, usize)> for Board {
    fn index_mut(&mut self, (row, column): (usize, usize)) -> &mut Self::Output {
        assert!(Board::valid_index((row, column)));
        &mut self.board_array[BOARD_HEIGHT * row + column]
    }
}
