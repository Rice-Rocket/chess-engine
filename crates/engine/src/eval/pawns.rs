use crate::board::coord::Coord;
use super::state::State;

pub fn isolated(state: &State, sqr: Coord) -> i32 {
    todo!();
}

pub fn opposed(state: &State, sqr: Coord) -> i32 {
    todo!();
}

pub fn phalanx(state: &State, sqr: Coord) -> i32 {
    todo!();
}

pub fn supported(state: &State, sqr: Coord) -> i32 {
    todo!();
}

pub fn backward(state: &State, sqr: Coord) -> i32 {
    todo!();
}

pub fn doubled(state: &State, sqr: Coord) -> i32 {
    todo!();
}

pub fn connected(state: &State, sqr: Coord) -> i32 {
    todo!();
}

pub fn connected_bonus(state: &State, sqr: Coord) -> i32 {
    todo!();
}

pub fn weak_unopposed_pawn(state: &State, sqr: Coord) -> i32 {
    todo!();
}

pub fn weak_lever(state: &State, sqr: Coord) -> i32 {
    todo!();
}

pub fn blocked(state: &State, sqr: Coord) -> i32 {
    todo!();
}

pub fn doubled_isolated(state: &State, sqr: Coord) -> i32 {
    todo!();
}

pub fn pawns_mg(state: &State, sqr: Coord) -> i32 {
    todo!();
}

pub fn pawns_eg(state: &State, sqr: Coord) -> i32 {
    todo!();
}


#[cfg(test)]
mod tests {
    use crate::{sum_sqrs, assert_eval, color::Color, board::{Board, zobrist::Zobrist}};
    use super::*;

    #[test]
    #[ignore = "unimplemented evaluation function"]
    fn test_isolated() {
        assert_eval!(isolated, 4, 0, "1r3q1R/p3n2n/np1k1pR1/pQ3P1B/1b1P1qpr/QP3n1P/P4P2/1PB1N1RK w kq - 9 6");
    }

    #[test]
    #[ignore = "unimplemented evaluation function"]
    fn test_opposed() {
        assert_eval!(opposed, 5, 4, "1r3q1R/p3n2n/np1k1pR1/pQ3P1B/1b1P1qpr/QP3n1P/P4P2/1PB1N1RK w kq - 9 6");
    }

    #[test]
    #[ignore = "unimplemented evaluation function"]
    fn test_phalanx() {
        assert_eval!(phalanx, 2, 0, "1r3q1R/p3n2n/np1k1pR1/pQ3P1B/1b1P1qpr/QPP2n1P/P4P2/2B1N1RK w kq - 9 6");
    }

    #[test]
    #[ignore = "unimplemented evaluation function"]
    fn test_supported() {
        assert_eval!(supported, 1, 2, "1r3q1R/p3n2n/np1k1pR1/pQ3P1B/1b1P1qpr/QP3n1P/P2P1P2/2B1N1RK w kq - 9 6");
    }

    #[test]
    #[ignore = "unimplemented evaluation function"]
    fn test_backward() {
        assert_eval!(backward, 2, 1, "1r3q1R/p3n2n/np1k1pR1/pQ3P1B/1b1P1qpr/QP3n1P/P2P1P2/2B1N1RK w kq - 9 6");
    }

    #[test]
    #[ignore = "unimplemented evaluation function"]
    fn test_doubled() {
        assert_eval!(doubled, 1, 0, "1r3q1R/2n1n2n/pp1k1pR1/pQ3P1B/1b1P1qpr/QP1P1n1P/P4P2/2B1N1RK w kq - 1 7");
    }

    #[test]
    #[ignore = "unimplemented evaluation function"]
    fn test_connected() {
        assert_eval!(connected, 1, 3, "1r3q1R/2n1n2n/pp1k1pR1/pQ3P1B/1b1P1qpr/QP1P1n1P/P4P2/2B1N1RK w kq - 1 7");
    }

    #[test]
    #[ignore = "unimplemented evaluation function"]
    fn test_connected_bonus() {
        assert_eval!(connected_bonus, 29, 65, "1r3q1R/2n1n2n/pp1k1pR1/pQ3P1B/1b1P1qpr/QP1P1n1P/P4P2/2B1N1RK w kq - 1 7");
    }

    #[test]
    #[ignore = "unimplemented evaluation function"]
    fn test_weak_unopposed_pawn() {
        assert_eval!(weak_unopposed_pawn, 3, 0, "1r3q1R/2n1n2n/pp1k1pR1/pQ3P1B/1b1P1qpr/QP1P1n1P/P4P2/2B1N1RK w kq - 1 7");
    }

    #[test]
    #[ignore = "unimplemented evaluation function"]
    fn test_weak_lever() {
        assert_eval!(weak_lever, 1, 0, "1r3q1R/2n1n2n/pp1k1pRp/pQP3PB/1b2Pq1r/Q4n1P/P1P2P2/2B1N1RK w kq - 1 7");
    }

    #[test]
    #[ignore = "unimplemented evaluation function"]
    fn test_blocked() {
        assert_eval!(blocked, 1, 2, "1r3q1R/2n4n/p2knpRp/1Qp2PPB/1bP2q1r/p4n1P/P1P2P2/2B1N1RK b kq - 0 7");
    }

    #[test]
    #[ignore = "unimplemented evaluation function"]
    fn test_doubled_isolated() {
        assert_eval!(doubled_isolated, 1, 1, "1r3q1R/2n4n/p2knpRp/pQp2PPB/1bP2q1r/5n1P/P1P2P2/2B1N1RK b kq - 0 7");
    }

    #[test]
    #[ignore = "unimplemented evaluation function"]
    fn test_pawns_mg() {
        assert_eval!(pawns_mg, 113, -42, "1r3q1R/2n4n/p2knpRp/pQp2PPB/1bP2q1r/5n1P/P1P2P2/2B1N1RK b kq - 0 7");
    }

    #[test]
    #[ignore = "unimplemented evaluation function"]
    fn test_pawns_eg() {
        assert_eval!(pawns_eg, -74, -172, "1r3q1R/2n4n/p2knpRp/pQp2PPB/1bP2q1r/5n1P/P1P2P2/2B1N1RK b kq - 0 7");
    }
}
