use crate::board::coord::Coord;
use crate::move_gen::bitboard::bb::BitBoard;
use super::{perspective::Perspective, attack::pinned_direction_sqr};
use super::pos::PositionEvaluation;

pub fn enemy_blockers_for_king(pos: &PositionEvaluation, per: Perspective) -> BitBoard {
    pos.enemy_pin_rays(per) & pos.enemy_color_bb(per)
    // sum_sqrs(enemy_blockers_for_king_sqr, pos, per)
}

pub fn enemy_blockers_for_king_sqr(pos: &PositionEvaluation, per: Perspective, sqr: Coord) -> i32 {
    if pinned_direction_sqr(pos, per.other(), sqr) > 0 { return 1; }
    return 0;
}