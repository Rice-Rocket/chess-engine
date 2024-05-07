use std::ops::Index;

use crate::board::Board;


pub struct State<'a> {
    pub board: &'a Board,
    pub color: Color,
}

impl<'a> State<'a> {
    pub fn flip(self) -> Self {
        Self {
            color: self.color.flip(),
            ..self
        }
    }
}


pub struct Color(usize);

impl Color {
    pub fn flip(self) -> Self {
        Self(1 - self.0)
    }
}


impl<T> Index<Color> for Vec<T> {
    type Output = T;

    fn index(&self, index: Color) -> &Self::Output {
        &self[index.0]
    }
}


/// Executes the given function on all squares, returning the sum.
/// 
/// Syntax: {`function`}: {`args`}
///
/// Note that `args` does not include the square.
#[macro_export]
macro_rules! sum_sqrs {
    ($f:ident: $($arg:expr,)*) => {
        {
            let mut sum = 0i32;
            
            for sqr in Coord::iter_squares() {
                sum += $f($($arg),*, sqr);
            }

            sum
        }
    }
}
