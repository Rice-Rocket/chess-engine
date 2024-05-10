use std::ops::Index;

use crate::board::zobrist::Zobrist;
use crate::board::{coord::Coord, Board};
use crate::color::Color;
use crate::move_gen::magics::MagicBitBoards;
use crate::move_gen::move_generator::MoveGenerator;
use crate::precomp::PrecomputedData;


pub struct State<'a> {
    pub board: &'a Board,
    pub precomp: &'a PrecomputedData,
    pub movegen: &'a MoveGenerator,
    pub color: Color,
}

impl<'a> State<'a> {
    pub fn new(board: &'a Board, precomp: &'a PrecomputedData, movegen: &'a MoveGenerator, color: Color) -> Self {
        Self {
            board, 
            precomp,
            movegen,
            color,
        }
    }

    pub fn flip(self) -> Self {
        Self {
            color: self.color.flip(),
            ..self
        }
    }
}

pub mod test_prelude {
    pub use crate::precomp::PrecomputedData;
    pub use crate::board::Board;
    pub use crate::board::zobrist::Zobrist;
    pub use crate::color::Color;
    pub use crate::eval::state::State;
    pub use crate::move_gen::magics::MagicBitBoards;
    pub use crate::move_gen::move_generator::MoveGenerator;
    pub use crate::assert_eval;
    pub use crate::sum_sqrs;
}


/// Executes the given function on all squares, returning the sum.
/// 
/// Syntax: {`function`}: {`args`}
///
/// Note that `args` does not include the square.
#[macro_export]
macro_rules! sum_sqrs {
    ($f:ident: $($arg:expr),* $(,)*) => {
        {
            let mut sum = 0i32;
            
            for sqr in Coord::iter_squares() {
                sum += $f($($arg,)* sqr);
            }

            sum
        }
    }
}

/// assert_eval!(`f`, [[`file`, `rank`]], `white_eval`, `black_eval`, `fen`; {`args`})
/// 
///   --> Test evaluation function `f` at the given `rank` and `file`.
///
/// assert_eval!(`f`, `white_eval`, `black_eval`, `fen`; {`args`})
///
///   --> Test evaluation function `f` over all squares, summing their results.
///
/// assert_eval!(- `f`, `white_eval`, `black_eval`, `fen`; {`args`})
///
///   --> Test evaluation function `f` without supplying any square arguments.
#[macro_export]
macro_rules! assert_eval {
    ($f:ident, [$file:expr, $rank:expr], $w:expr, $b:expr, $state:ident $(; $($arg:expr),*)?) => {
        $state.color = Color::White;
        assert_eq!($f(
            &$state,
            $($($arg,)*)? 
            Coord::new($file, $rank)
        ), $w);
        
        $state.color = Color::Black;
        assert_eq!($f(
            &$state,
            $($($arg,)*)? 
            Coord::new($file, $rank)
        ), $b);
    };

    ($f:ident, $w:expr, $b:expr, $state:ident $(; $($arg:expr),*)?) => {
        $state.color = Color::White;
        assert_eq!(sum_sqrs!(
            $f:
            &$state,
            $($($arg,)*)? 
        ), $w);

        $state.color = Color::Black;
        assert_eq!(sum_sqrs!(
            $f:
            &$state,
            $($($arg,)*)? 
        ), $b);
    };

    (- $f:ident, $w:expr, $b:expr, $state:ident $(; $($arg:expr),*)?) => {
        $state.color = Color::White;
        assert_eq!($f(
            &$state,
            $($($arg,)*)? 
        ), $w);

        $state.color = Color::Black;
        assert_eq!($f(
            &$state,
            $($($arg,)*)? 
        ), $b);
    };
}
