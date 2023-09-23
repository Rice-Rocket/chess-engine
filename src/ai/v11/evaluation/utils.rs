use crate::board::{board::Board, coord::Coord};
use super::perspective::Perspective;


pub fn sum_sqrs(func: fn(&Board, Perspective, Coord) -> i32, board: &Board, perspective: Perspective) -> i32 {
    let mut sum = 0;
    for sqr in Coord::iterate_squares() {
        sum += func(board, perspective, sqr);
    }
    return sum;
}
pub fn total(func: fn(&Board, Perspective) -> i32, board: &Board) -> i32 {
    func(board, Perspective::White) - func(board, Perspective::Black)
}