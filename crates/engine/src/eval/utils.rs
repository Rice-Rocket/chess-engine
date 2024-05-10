use crate::board::coord::Coord;
use super::state::State;

pub fn bishop_count(state: &State, sqr: Coord) -> i32 {
    todo!();
}

pub fn queen_count(state: &State, sqr: Coord) -> i32 {
    todo!();
}

pub fn pawn_count(state: &State, sqr: Coord) -> i32 {
    todo!();
}

pub fn knight_count(state: &State, sqr: Coord) -> i32 {
    todo!();
}

pub fn rook_count(state: &State, sqr: Coord) -> i32 {
    todo!();
}

pub fn opposite_bishops(state: &State) -> i32 {
    todo!();
}

pub fn king_distance(state: &State, sqr: Coord) -> i32 {
    todo!();
}

pub fn king_ring(state: &State, full: bool, sqr: Coord) -> i32 {
    todo!();
}

pub fn piece_count(state: &State, sqr: Coord) -> i32 {
    todo!();
}

pub fn pawn_attacks_span(state: &State, sqr: Coord) -> i32 {
    todo!();
}


#[cfg(test)]
mod tests {
    use crate::{assert_eval, board::{zobrist::Zobrist, Board}, color::Color, sum_sqrs};
    use super::*;

    #[test]
    #[ignore = "unimplemented evaluation function"]
    fn test_bishop_count() {
        assert_eval!(bishop_count, 2, 3, "nb3b1R/p1pkn2p/1p2Rpn1/n6B/1bPP1qpP/QP2r1P1/P1P2P2/2BN2RK b Qkq - 1 2");
    }

    #[test]
    #[ignore = "unimplemented evaluation function"]
    fn test_queen_count() {
        assert_eval!(queen_count, 2, 1, "nb3b1R/p1pkn2p/1p2Rpn1/nQ5B/1bPP1qpP/QP2r1P1/P1P2P2/2BN2RK b Qkq - 1 2");
    }

    #[test]
    #[ignore = "unimplemented evaluation function"]
    fn test_pawn_count() {
        assert_eval!(pawn_count, 8, 6, "nb3b1R/p1pkn2p/1p2Rpn1/nQ5B/1bPP1qpP/QP2r1P1/P1P2P2/2BN2RK b Qkq - 1 2");
    }

    #[test]
    #[ignore = "unimplemented evaluation function"]
    fn test_knight_count() {
        assert_eval!(knight_count, 1, 4, "nb3b1R/p1pkn2p/1p2Rpn1/nQ5B/1bPP1qpP/QP2r1P1/P1P2P2/2BN2RK b Qkq - 1 2");
    }

    #[test]
    #[ignore = "unimplemented evaluation function"]
    fn test_rook_count() {
        assert_eval!(rook_count, 3, 1, "nb3b1R/p1pkn2p/1p2Rpn1/nQ5B/1bPP1qpP/QP2r1P1/P1P2P2/2BN2RK b Qkq - 1 2");
    }

    #[test]
    #[ignore = "unimplemented evaluation function"]
    fn test_opposite_bishops() {
        assert_eval!(- opposite_bishops, 1, 1, "n1b1r3/4p1Q1/1q2pP2/kpp4r/P1P4r/R1B1N2P/P4P1R/4RnK1 b kq - 2 11");
        assert_eval!(- opposite_bishops, 0, 0, "nb2r3/4p1Q1/1q2pP2/kpp4r/P1P4r/R1B1N2P/P4P1R/4RnK1 b kq - 2 11");
    }

    #[test]
    #[ignore = "unimplemented evaluation function"]
    fn test_king_distance() {
        assert_eval!(king_distance, 308, 215, "nb3b1R/p1pkn2p/1p2Rpn1/nQ5B/1bPP1qpP/QP2r1P1/P1P2P2/2BN2RK b Qkq - 1 2");
        assert_eval!(king_distance, [2, 3], 5, 3, "nb3b1R/p1pkn2p/1p2Rpn1/nQ5B/1bPP1qpP/QP2r1P1/P1P2P2/2BN2RK b Qkq - 1 2");
    }

    #[test]
    #[ignore = "unimplemented evaluation function"]
    fn test_king_ring() {
        assert_eval!(king_ring, [6, 2], 0, 0, "nb3b1R/p1pkn3/1p2Rpn1/nQ5B/1bPP1qpP/QP2r3/P1P2P1P/2BN2RK w Qkq - 2 3"; false);
        assert_eval!(king_ring, [5, 2], 0, 1, "nb3b1R/p1pkn3/1p2Rpn1/nQ5B/1bPP1qpP/QP2r3/P1P2P1P/2BN2RK w Qkq - 2 3"; false);
        assert_eval!(king_ring, [3, 5], 1, 0, "nb3b1R/p1pkn3/1p2Rpn1/nQ5B/1bPP1qpP/QP2r3/P1P2P1P/2BN2RK w Qkq - 2 3"; false);
    }

    #[test]
    #[ignore = "unimplemented evaluation function"]
    fn test_piece_count() {
        assert_eval!(piece_count, 17, 15, "nb3b1R/p1pkn3/1p2Rpn1/nQ5B/1bPP1qpP/QP2r3/P1P2P1P/2BN2RK w Qkq - 2 3");
    }

    #[test]
    #[ignore = "unimplemented evaluation function"]
    fn test_pawn_attacks_span() {
        assert_eval!(pawn_attacks_span, 28, 32, "nb3b1R/p1pkn3/n3Rpn1/pQ5B/1bPP1qpP/QP2r3/P1P2P1P/2BN2RK b Qkq - 3 3");
    }
}
