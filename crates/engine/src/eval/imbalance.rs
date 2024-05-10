use crate::board::coord::Coord;
use super::state::State;

pub fn imbalance(state: &State, sqr: Coord) -> i32 {
    todo!();
}

pub fn bishop_pair(state: &State, sqr: Coord) -> i32 {
    todo!();
}

pub fn imbalance_total(state: &State) -> i32 {
    todo!();
}


#[cfg(test)]
mod tests {
    use crate::{sum_sqrs, assert_eval, color::Color, board::{Board, zobrist::Zobrist}};
    use super::*;

    #[test]
    #[ignore = "unimplemented evaluation function"]
    fn test_imbalance() {
        assert_eval!(imbalance, 9878, 14273, "nb3b1R/p1pkn3/n3Rpn1/pQ5B/1bPP1qpP/QP2r3/P1P2P1P/2BN2RK b Qkq - 3 3");
    }

    #[test]
    #[ignore = "unimplemented evaluation function"]
    fn test_bishop_pair() {
        assert_eval!(bishop_pair, 1438, 0, "nr3q1R/p1pkn3/n3Rpn1/pQ5B/1bPP1qpP/QP2r3/P1P2P1P/2BN2RK b Qkq - 3 3");
    }

    #[test]
    #[ignore = "unimplemented evaluation function"]
    fn test_imbalance_total() {
        assert_eval!(- imbalance_total, -181, 181, "nr3q1R/p1pkn3/n3Rpn1/pQ5B/1bPP1qpP/QP2r3/P1P2P1P/2BN2RK b Qkq - 3 3");
    }
}
