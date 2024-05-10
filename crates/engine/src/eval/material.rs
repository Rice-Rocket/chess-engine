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


#[cfg(test)]
mod tests {
    use crate::{sum_sqrs, assert_eval, color::Color, board::{Board, zobrist::Zobrist}};
    use super::*;

    #[test]
    #[ignore = "unimplemented evaluation function"]
    fn test_non_pawn_material() {
        assert_eval!(non_pawn_material, 11335, 11577, "1r3q1R/p1p1n2n/n2k1pR1/pQ3P1B/1bP2qpr/QP3n1P/P1P1P3/2B1N1RK w kq - 9 6");
    }

    #[test]
    #[ignore = "unimplemented evaluation function"]
    fn test_piece_value_bonus() {
        assert_eval!(piece_value_bonus, 12203, 12197, "1r3q1R/p1p1n2n/n2k1pR1/pQ3P1B/1bP2qpr/QP3n1P/P1P1P3/2B1N1RK w kq - 9 6"; true);
        assert_eval!(piece_value_bonus, 13630, 13485, "1r3q1R/p1p1n2n/n2k1pR1/pQ3P1B/1bP2qpr/QP3n1P/P1P1P3/2B1N1RK w kq - 9 6"; false);
    }

    #[test]
    #[ignore = "unimplemented evaluation function"]
    fn test_psqt_bonus() {
        assert_eval!(psqt_bonus, 146, 32, "1r3q1R/p1p1n2n/n2k1pR1/pQ3P1B/1bP2qpr/QP3n1P/P1P1P3/2B1N1RK w kq - 9 6"; true);
        assert_eval!(psqt_bonus, -126, 26, "1r3q1R/p1p1n2n/n2k1pR1/pQ3P1B/1bP2qpr/QP3n1P/P1P1P3/2B1N1RK w kq - 9 6"; false);
    }
}
