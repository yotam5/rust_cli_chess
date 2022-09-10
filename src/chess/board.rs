use super::piece::Piece;
use std::iter::{IntoIterator, Iterator};
use std::ops::{Index, IndexMut};

//TODO!: generic board array that have method that return
/// pointer to slices for each row

///TODO!:
/// iter -> to get iterator
/// into_iter -> to get iterator, for loop &Board
/// Index, IndexMut
/// Iterator + next

#[derive(Debug)]

/// T - The type of that the array contains
/// RTV - Row Times Column (row * column), for 1d to 2d array conversion
pub struct Board<T, const RTC: usize>([T;RTC]);

pub struct IterBoard<T, const RTC: usize>
{
    type Item = 1
}


