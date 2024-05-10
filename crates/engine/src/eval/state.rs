use std::ops::Index;

use crate::board::{coord::Coord, Board};
use crate::color::Color;


pub struct State<'a> {
    pub board: &'a Board,
    pub color: Color,
}

impl<'a> State<'a> {
    pub fn new(board: &'a Board, color: Color) -> Self {
        Self {
            board, 
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
    ($f:ident, [$file:expr, $rank:expr], $w:expr, $b:expr, $fen:literal $(; $($arg:expr),*)?) => {
        assert_eq!($f(
            &State::new(&Board::load_position(Some(String::from($fen)), &mut Zobrist::new()), Color::White), 
            $($($arg,)*)? 
            Coord::new($file, $rank)
        ), $w);

        assert_eq!($f(
            &State::new(&Board::load_position(Some(String::from($fen)), &mut Zobrist::new()), Color::Black), 
            $($($arg,)*)? 
            Coord::new($file, $rank)
        ), $b);
    };

    ($f:ident, $w:expr, $b:expr, $fen:literal $(; $($arg:expr),*)?) => {
        assert_eq!(sum_sqrs!(
            $f:
            &State::new(&Board::load_position(Some(String::from($fen)), &mut Zobrist::new()), Color::White), 
            $($($arg,)*)? 
        ), $w);

        assert_eq!(sum_sqrs!(
            $f:
            &State::new(&Board::load_position(Some(String::from($fen)), &mut Zobrist::new()), Color::Black), 
            $($($arg,)*)? 
        ), $b);
    };

    (- $f:ident, $w:expr, $b:expr, $fen:literal $(; $($arg:expr),*)?) => {
        assert_eq!($f(
            &State::new(&Board::load_position(Some(String::from($fen)), &mut Zobrist::new()), Color::White), 
            $($($arg,)*)? 
        ), $w);

        assert_eq!($f(
            &State::new(&Board::load_position(Some(String::from($fen)), &mut Zobrist::new()), Color::Black), 
            $($($arg,)*)? 
        ), $b);
    };
}
