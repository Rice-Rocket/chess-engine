use std::ops::{Index, IndexMut};

use crate::board::coord::Coord;

pub struct SquareValues<T>([T; 64]);

impl<T> Index<usize> for SquareValues<T> {
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
    }
}

impl<T> IndexMut<usize> for SquareValues<T> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.0[index]
    }
}

impl<T> Index<Coord> for SquareValues<T> {
    type Output = T;

    fn index(&self, index: Coord) -> &Self::Output {
        &self.0[index.index()]
    }
}

impl<T> IndexMut<Coord> for SquareValues<T> {
    fn index_mut(&mut self, index: Coord) -> &mut Self::Output {
        &mut self.0[index.index()]
    }
}


pub type SquareEvaluations = SquareValues<i32>;
