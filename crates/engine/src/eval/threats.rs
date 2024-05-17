use proc_macro_utils::evaluation_fn;

use crate::{board::coord::Coord, color::{Color, White, Black}, prelude::BitBoard};
use super::Evaluation;


impl<'a> Evaluation<'a> {
    pub fn safe_pawn<W: Color, B: Color>(&self) -> BitBoard {
        todo!();
    }

    pub fn threat_safe_pawn<W: Color, B: Color>(&self) -> BitBoard {
        todo!();
    }

    pub fn weak_enemies<W: Color, B: Color>(&self) -> BitBoard {
        todo!();
    }

    pub fn minor_threat<W: Color, B: Color>(&self, sqr: Coord) -> i32 {
        todo!();
    }

    pub fn rook_threat<W: Color, B: Color>(&self, sqr: Coord) -> i32 {
        todo!();
    }

    pub fn hanging<W: Color, B: Color>(&self) -> BitBoard {
        todo!();
    }

    pub fn king_threat<W: Color, B: Color>(&self) -> BitBoard {
        todo!();
    }

    pub fn pawn_push_threat<W: Color, B: Color>(&self) -> BitBoard {
        todo!();
    }

    /// Returns `(when at least one friendly queen, when no friendly queens)`
    pub fn slider_on_queen<W: Color, B: Color>(&self) -> (BitBoard, BitBoard) {
        todo!();
    }

    /// Returns `(when at least one friendly queen, when no friendly queens)`
    pub fn knight_on_queen<W: Color, B: Color>(&self) -> (BitBoard, BitBoard) {
        todo!();
    }

    pub fn restricted<W: Color, B: Color>(&self) -> BitBoard {
        todo!();
    }

    pub fn weak_queen_protection<W: Color, B: Color>(&self) -> BitBoard {
        todo!();
    }

    pub fn threats_mg<W: Color, B: Color>(&self) -> i32 {
        todo!();
    }

    pub fn threats_eg<W: Color, B: Color>(&self) -> i32 {
        todo!();
    }
}


#[cfg(test)]
mod tests {
    use crate::eval::test_prelude::*;
    use super::*;

    #[test]
    #[ignore = "unimplemented evaluation function"]
    #[evaluation_test("nr1B3Q/1k2p2p/p2n2R1/p1p1bP1q/R1P1qB1r/1NP3nP/P4PBR/6K1 w kq - 3 9")]
    fn test_safe_pawn() {
        assert_eval!(+ - safe_pawn, 4, 2, eval);
    }

    #[test]
    #[ignore = "unimplemented evaluation function"]
    #[evaluation_test("nr1B3Q/1k2p2p/p2n2R1/1pp1bP1q/R1P1qB1r/1NP3nP/P4PBR/6K1 b kq - 0 9")]
    fn test_threat_safe_pawn() {
        assert_eval!(+ - threat_safe_pawn, 1, 2, eval);
    }

    #[test]
    #[ignore = "unimplemented evaluation function"]
    #[evaluation_test("nr1B3Q/1k2p2p/p2n2R1/1pp1bP1q/R1P1qB1r/1NP3nP/P4PBR/6K1 b kq - 0 9")]
    fn test_weak_enemies() {
        assert_eval!(+ - weak_enemies, 5, 7, eval);
    }

    #[test]
    #[ignore = "unimplemented evaluation function"]
    #[evaluation_test("nr1B3Q/1k2p2p/p2n2R1/1pp1bP1q/R1P1qB1r/1NP3nP/P4PBR/6K1 b kq - 0 9")]
    fn test_minor_threat() {
        assert_eval!(minor_threat, 18, 11, eval);
    }

    #[test]
    #[ignore = "unimplemented evaluation function"]
    #[evaluation_test("nr1B3Q/1k2p2p/p2n2R1/1pp1bP1q/R1P1qB1r/1NP3nP/P4PBR/6K1 b kq - 0 9")]
    fn test_rook_threat() {
        assert_eval!(rook_threat, 3, 6, eval);
    }

    #[test]
    #[ignore = "unimplemented evaluation function"]
    #[evaluation_test("nr1B3Q/1k2p2p/p2n2R1/1pp1bP1q/R1P1qB1r/1NP3nP/P4PBR/6K1 b kq - 0 9")]
    fn test_hanging() {
        assert_eval!(+ - hanging, 4, 5, eval);
    }

    #[test]
    #[ignore = "unimplemented evaluation function"]
    #[evaluation_test("nr1B3Q/4p2p/p2n2R1/kPp1bP1q/R3qB1r/1NP4P/P4PBR/5nK1 b kq - 0 9")]
    fn test_king_threat() {
        assert_eval!(+ - king_threat, 1, 2, eval);
    }

    #[test]
    #[ignore = "unimplemented evaluation function"]
    #[evaluation_test("nr1B3Q/2p1p3/p2n2R1/kRp1bP1q/P3qB1r/1NP4P/P4PBR/5nK1 b kq - 0 9")]
    fn test_pawn_push_threat() {
        assert_eval!(+ - pawn_push_threat, 1, 2, eval);
    }

    #[test]
    #[ignore = "unimplemented evaluation function"]
    #[evaluation_test("n3r3/2p1p1Q1/p2n4/k1p1bP1r/P1PB3r/R2BN2P/Pq3P1R/1B2RnK1 b kq - 0 9")]
    fn test_slider_on_queen() {
        assert_eval!(* - [0, 1] slider_on_queen, (4, 0), (3, 0), eval);
    }

    #[test]
    #[ignore = "unimplemented evaluation function"]
    #[evaluation_test("n2Br3/2p1p1Q1/p2n4/kRp1bP1r/P1P4r/3BN2P/Pq3P1R/1B2RnK1 b kq - 0 9")]
    fn test_knight_on_queen() {
        assert_eval!(* - [0, 1] knight_on_queen, (1, 0), (2, 0), eval);
    }

    #[test]
    #[ignore = "unimplemented evaluation function"]
    #[evaluation_test("n3r3/2p1p1Q1/p2n4/k1p1bP1r/P1PB3r/R2BN2P/Pq3P1R/1B2RnK1 b kq - 0 9")]
    fn test_restricted() {
        assert_eval!(+ - restricted, 20, 16, eval);
    }

    #[test]
    #[ignore = "unimplemented evaluation function"]
    #[evaluation_test("n1n1r3/4p1Q1/1q2pP2/kpp1bB1r/P1PB3r/R3N2P/P4P1R/1B2RnK1 b kq - 2 11")]
    fn test_weak_queen_protection() {
        assert_eval!(+ - weak_queen_protection, 3, 1, eval);
    }

    #[test]
    #[ignore = "unimplemented evaluation function"]
    #[evaluation_test("n3r3/2p1p1Q1/p2n4/k1p1bP1r/P1PB3r/R2BN2P/Pq3P1R/1B2RnK1 b kq - 0 9")]
    fn test_threats_mg() {
        assert_eval!(- threats_mg, 951, 978, eval);
    }

    #[test]
    #[ignore = "unimplemented evaluation function"]
    #[evaluation_test("n3r3/2p1p1Q1/p2n4/k1p1bP1r/P1PB3r/R2BN2P/Pq3P1R/1B2RnK1 b kq - 0 9")]
    fn test_threats_eg() {
        assert_eval!(- threats_eg, 814, 982, eval);
    }
}
