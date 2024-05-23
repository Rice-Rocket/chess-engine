use std::ops::{Add, AddAssign, Index, IndexMut, BitAnd, BitAndAssign};

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


impl<T> SquareValues<T> {
    pub fn map<U, F: Fn(T) -> U>(self, f: F) -> SquareValues<U> {
        SquareValues(self.0.map(f))
    }
}

impl<T: Default + Copy> SquareValues<T> {
    pub fn new() -> Self {
        Self([T::default(); 64])
    }

    pub fn zip<U: Default + Copy>(self, rhs: SquareValues<U>) -> SquareValues<(T, U)> {
        let mut zipped = [(T::default(), U::default()); 64];

        for (i, z) in zipped.iter_mut().enumerate() {
            z.0 = self.0[i];
            z.1 = rhs.0[i];
        }

        SquareValues(zipped)
    }

    pub fn enumerate(self) -> SquareValues<(usize, T)> {
        let mut enumerate = [(0, T::default()); 64];

        for (i, (i1, v)) in enumerate.iter_mut().enumerate() {
            *v = self.0[i];
            *i1 = i;
        }

        SquareValues(enumerate)
    }
}

impl<T: Default + Copy> Default for SquareValues<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T: Add<T, Output = T> + Clone> SquareValues<T> {
    pub fn count(&self) -> T {
        self.0.iter().cloned().reduce(|a, x| a + x).unwrap()
    }
}


macro_rules! impl_bb_ops {
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

            impl BitAnd<BitBoard> for $t<$prim> {
                type Output = $t<$prim>;

                fn bitand(self, rhs: BitBoard) -> Self::Output {
                    self.enumerate().map(|(sqr, v)| v * rhs.square_value(sqr as i8) as $prim)
                }
            }
        )*
    }
}

impl_bb_ops!(SquareValues; i8, i16, i32, i64, u8, u16, u32, u64);


pub type SquareEvaluations = SquareValues<i32>;
pub type SquareCounts = SquareValues<u8>;
