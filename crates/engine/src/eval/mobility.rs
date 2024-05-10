use crate::board::coord::Coord;
use super::state::State;

pub fn mobility(state: &State, sqr: Coord) -> i32 {
    todo!();
}

pub fn mobility_area(state: &State, sqr: Coord) -> i32 {
    todo!();
}

pub fn mobility_bonus(state: &State, mg: bool, sqr: Coord) -> i32 {
    todo!();
}

#[inline]
pub fn mobility_mg(state: &State, sqr: Coord) -> i32 {
    todo!();
}

#[inline]
pub fn mobility_eg(state: &State, sqr: Coord) -> i32 {
    todo!();
}


#[cfg(test)]
mod tests {
    use crate::{sum_sqrs, assert_eval, color::Color, board::{Board, zobrist::Zobrist}};
    use super::*;

    #[test]
    #[ignore = "unimplemented evaluation function"]
    fn test_mobility() {
        assert_eval!(mobility, 41, 48, "1r3q1R/p1p1n2n/n2k1pR1/pQ3P1B/1bP2qpr/QP3n1P/P1P1P3/2B1N1RK w kq - 9 6");
    }

    #[test]
    #[ignore = "unimplemented evaluation function"]
    fn test_mobility_area() {
        assert_eval!(mobility_area, 49, 47, "1r3q1R/p1p1n2n/n2k1pR1/pQ3P1B/1bP2qpr/QP3n1P/P1P1P3/2B1N1RK w kq - 9 6");
    }

    #[test]
    #[ignore = "unimplemented evaluation function"]
    fn test_mobility_bonus() {
        assert_eval!(mobility_bonus, 193, 158, "1r3q1R/p1p1n2n/n2k1pR1/pQ3P1B/1bP2qpr/QP3n1P/P1P1P3/2B1N1RK w kq - 9 6"; true);
        assert_eval!(mobility_bonus, 467, 293, "1r3q1R/p1p1n2n/n2k1pR1/pQ3P1B/1bP2qpr/QP3n1P/P1P1P3/2B1N1RK w kq - 9 6"; false);
    }
}
