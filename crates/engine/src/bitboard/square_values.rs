use std::ops::{Add, AddAssign, Index, IndexMut};

use crate::board::coord::Coord;

use super::bb::BitBoard;

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


macro_rules! impl_add_bb {
    ($t:tt; $($prim:ty),*) => {
        $(
            impl Add<BitBoard> for $t<$prim> {
                type Output = $t<$prim>;

                fn add(self, rhs: BitBoard) -> Self::Output {
                    let mut res = [0; 64];
                    for ((i, v), r) in self.0.into_iter().enumerate().zip(&mut res) {
                        *r = v + ((rhs >> i) & 1).0 as $prim;
                    }
                    $t(res)
                }
            }

            impl AddAssign<BitBoard> for $t<$prim> {
                fn add_assign(&mut self, rhs: BitBoard) {
                    for (i, v) in self.0.iter_mut().enumerate() {
                        *v = *v + ((rhs >> i) & 1).0 as $prim;
                    }
                }
            }
        )*
    }
}

impl_add_bb!(SquareValues; i8, i16, i32, i64, u8, u16, u32, u64);


pub type SquareEvaluations = SquareValues<i32>;
pub type SquareCounts = SquareValues<u8>;
