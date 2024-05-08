use crate::board::coord::Coord;
use super::state::State;

pub fn winnable(state: &State, sqr: Coord) -> i32 {
    todo!();
}

pub fn winnable_total_mg(state: &State, v: i32) -> i32 {
    todo!();
}

pub fn winnable_total_eg(state: &State, v: i32) -> i32 {
    todo!();
}


#[cfg(test)]
mod tests {
    use crate::{sum_sqrs, assert_eval, eval::state::Color, Board, Zobrist};
    use super::*;

    #[test]
    fn test_winnable() {
        assert_eval!(winnable, 58, 58, "n3r3/2p1p1Q1/p2n4/k1p1bP1r/P1PB3r/R2BN2P/Pq3P1R/1B2RnK1 b kq - 0 9");
    }
}
