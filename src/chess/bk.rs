use super::piece::Piece;
use std::iter::{IntoIterator, Iterator};
use std::ops::{Index, IndexMut};


#[derive(Debug)]
pub struct Board<T,>(T);

pub struct IterBoard<'a,T,const N: usize>
{
    inner: &'a Board<T,N>,
    index: usize,
}

impl<'a,T> Iterator for IterBoard<'a,T> {
    type Item = &'a Square;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index >= self.inner.0.len() {
            return None;
        }
        self.index += 1;
        self.inner.0.get(self.index - 1)
    }
}

impl<'a> IntoIterator for &'a Board {
    type Item = &'a Square;
    type IntoIter = IterBoard<'a>;

    fn into_iter(self) -> Self::IntoIter {
        IterBoard {
            inner: self,
            index: 0,
        }
    }
}

impl Default for Board {
    fn default() -> Self {
        Board([Square::None; 64])
    }
}

impl Board {
    pub fn new() -> Self {
        Board::default()
    }

    pub fn valid_index((row, column): (usize, usize)) -> bool {
        row <= BOARD_WIDTH && column <= BOARD_HEIGHT
    }

    pub fn iter<'a>(&'a self) -> IterBoard<'a> {
        IterBoard {
            inner: self,
            index: 0,
        }
    }
}

/// implement indexing the board using [(row,column)] notation
impl Index<(usize, usize)> for Board {
    type Output = Square;

    fn index(&self, (row, column): (usize, usize)) -> &Self::Output {
        assert!(Board::valid_index((row, column)));
        &self.0[BOARD_WIDTH * row + column]
    }
}

/// implement the same as Index but as mutable
impl IndexMut<(usize, usize)> for Board {
    fn index_mut(&mut self, (row, column): (usize, usize)) -> &mut Self::Output {
        assert!(Board::valid_index((row, column)));
        &mut self.0[BOARD_HEIGHT * row + column]
    }
}

impl Iterator for Board {
    type Item = Square;

    fn next(&mut self) -> Option<Self::Item> {
        Some(self.0[0])
    }
}

// implement the same as Index but for board reference
/*impl Index<(usize, usize)> for &Board {
    type Output = Square;

    fn index(&self, (row, column): (usize, usize)) -> &Self::Output {
        assert!(Board::valid_index((row, column)));
        &self.0[BOARD_WIDTH * row + column]
    }
}

impl IndexMut<(usize, usize)> for &Board {
    fn index_mut<'a>(&'a  mut self, (row, column): (usize, usize)) ->&'a mut Self::Output {
        assert!(Board::valid_index((row, column)));
        &mut self.0.index_mut(0)
    }
}*/
