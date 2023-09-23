use crate::board::{board::Board, coord::Coord};
use super::{perspective::Perspective, attack::pinned_direction_sqr};
use super::utils::sum_sqrs;

pub fn blockers_for_king(board: &Board, per: Perspective) -> i32 {
    sum_sqrs(blockers_for_king_sqr, board, per)
}

pub fn blockers_for_king_sqr(board: &Board, per: Perspective, sqr: Coord) -> i32 {
    if pinned_direction_sqr(board, per.other(), sqr) > 0 { return 1; }
    return 0;
}