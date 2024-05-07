use crate::board::coord::Coord;
use super::state::State;

pub fn non_pawn_material(state: &State, sqr: Coord) -> i32 {
    todo!();
}

pub fn piece_value_bonus(state: &State, mg: bool, sqr: Coord) -> i32 {
    todo!();
}

pub fn psqt_bonus(state: &State, mg: bool, sqr: Coord) -> i32 {
    todo!();
}

#[inline]
pub fn piece_value_mg(state: &State, sqr: Coord) -> i32 {
    todo!();
}

#[inline]
pub fn piece_value_eg(state: &State, sqr: Coord) -> i32 {
    todo!();
}

#[inline]
pub fn psqt_mg(state: &State, sqr: Coord) -> i32 {
    todo!();
}

#[inline]
pub fn psqt_eg(state: &State, sqr: Coord) -> i32 {
    todo!();
}
