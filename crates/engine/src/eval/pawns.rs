use proc_macro_utils::evaluation_fn;

use crate::{board::{coord::Coord, piece::Piece}, color::{Color, White, Black}, prelude::BitBoard};
use super::Evaluation;


impl<'a> Evaluation<'a> {
    pub fn isolated<W: Color, B: Color>(&self) -> BitBoard {
        todo!();
    }

    pub fn opposed<W: Color, B: Color>(&self) -> BitBoard {
        todo!();
    }

    pub fn phalanx<W: Color, B: Color>(&self) -> BitBoard {
        todo!();
    }

    pub fn supported<W: Color, B: Color>(&self) -> BitBoard {
        todo!();
    }

    pub fn backward<W: Color, B: Color>(&self) -> BitBoard {
        let mut pawns = self.board.piece_bitboards[W::piece(Piece::PAWN)];
        let mut backward = BitBoard(0);

        let opp_pawns = self.board.piece_bitboards[B::piece(Piece::PAWN)];
        pawns &= opp_pawns.shifted_2d(Coord::new(-1, 2 * W::down_dir()))
            | opp_pawns.shifted_2d(Coord::new(1, 2 * W::down_dir()))
            | opp_pawns.shifted_2d(Coord::new(0, W::down_dir()));

        while pawns.0 != 0 {
            let sqr = Coord::from_idx(pawns.pop_lsb() as i8);
            let span = self.precomp.pawn_attack_span[B::index()][sqr];
            if (span & self.board.piece_bitboards[W::piece(Piece::PAWN)]).0 == 0 {
                backward |= sqr.to_bitboard();
            }
        }

        backward
    }

    pub fn doubled<W: Color, B: Color>(&self) -> BitBoard {
        todo!();
    }

    pub fn connected<W: Color, B: Color>(&self) -> BitBoard {
        todo!();
    }

    pub fn connected_bonus<W: Color, B: Color>(&self, sqr: Coord) -> i32 {
        todo!();
    }

    pub fn weak_unopposed_pawn<W: Color, B: Color>(&self) -> BitBoard {
        todo!();
    }

    pub fn weak_lever<W: Color, B: Color>(&self) -> BitBoard {
        todo!();
    }

    pub fn blocked<W: Color, B: Color>(&self) -> BitBoard {
        todo!();
    }

    pub fn doubled_isolated<W: Color, B: Color>(&self) -> BitBoard {
        todo!();
    }

    pub fn pawns_mg<W: Color, B: Color>(&self, sqr: Coord) -> i32 {
        todo!();
    }

    pub fn pawns_eg<W: Color, B: Color>(&self, sqr: Coord) -> i32 {
        todo!();
    }
}


#[cfg(test)]
mod tests {
    use crate::eval::test_prelude::*;
    use super::*;

    #[test]
    #[ignore = "unimplemented evaluation function"]
    #[evaluation_test("1r3q1R/p3n2n/np1k1pR1/pQ3P1B/1b1P1qpr/QP3n1P/P4P2/1PB1N1RK w kq - 9 6")]
    fn test_isolated() {
        assert_eval!(+ - isolated, 4, 0, eval);
    }

    #[test]
    #[ignore = "unimplemented evaluation function"]
    #[evaluation_test("1r3q1R/p3n2n/np1k1pR1/pQ3P1B/1b1P1qpr/QP3n1P/P4P2/1PB1N1RK w kq - 9 6")]
    fn test_opposed() {
        assert_eval!(+ - opposed, 5, 4, eval);
    }

    #[test]
    #[ignore = "unimplemented evaluation function"]
    #[evaluation_test("1r3q1R/p3n2n/np1k1pR1/pQ3P1B/1b1P1qpr/QPP2n1P/P4P2/2B1N1RK w kq - 9 6")]
    fn test_phalanx() {
        assert_eval!(+ - phalanx, 2, 0, eval);
    }

    #[test]
    #[ignore = "unimplemented evaluation function"]
    #[evaluation_test("1r3q1R/p3n2n/np1k1pR1/pQ3P1B/1b1P1qpr/QP3n1P/P2P1P2/2B1N1RK w kq - 9 6")]
    fn test_supported() {
        assert_eval!(+ - supported, 1, 2, eval);
    }

    #[test]
    #[evaluation_test("nb3b1R/p1pkn3/n3Rpn1/pQ5B/1bPP1qpP/QP2r3/P1P2P1P/2BN2RK b Qkq - 3 3")]
    fn test_backward() {
        assert_eval!(+ - backward, 2, 1, eval);
    }

    #[test]
    #[ignore = "unimplemented evaluation function"]
    #[evaluation_test("1r3q1R/2n1n2n/pp1k1pR1/pQ3P1B/1b1P1qpr/QP1P1n1P/P4P2/2B1N1RK w kq - 1 7")]
    fn test_doubled() {
        assert_eval!(+ - doubled, 1, 0, eval);
    }

    #[test]
    #[ignore = "unimplemented evaluation function"]
    #[evaluation_test("1r3q1R/2n1n2n/pp1k1pR1/pQ3P1B/1b1P1qpr/QP1P1n1P/P4P2/2B1N1RK w kq - 1 7")]
    fn test_connected() {
        assert_eval!(+ - connected, 1, 3, eval);
    }

    #[test]
    #[ignore = "unimplemented evaluation function"]
    #[evaluation_test("1r3q1R/2n1n2n/pp1k1pR1/pQ3P1B/1b1P1qpr/QP1P1n1P/P4P2/2B1N1RK w kq - 1 7")]
    fn test_connected_bonus() {
        assert_eval!(connected_bonus, 29, 65, eval);
    }

    #[test]
    #[ignore = "unimplemented evaluation function"]
    #[evaluation_test("1r3q1R/2n1n2n/pp1k1pR1/pQ3P1B/1b1P1qpr/QP1P1n1P/P4P2/2B1N1RK w kq - 1 7")]
    fn test_weak_unopposed_pawn() {
        assert_eval!(+ - weak_unopposed_pawn, 3, 0, eval);
    }

    #[test]
    #[ignore = "unimplemented evaluation function"]
    #[evaluation_test("1r3q1R/2n1n2n/pp1k1pRp/pQP3PB/1b2Pq1r/Q4n1P/P1P2P2/2B1N1RK w kq - 1 7")]
    fn test_weak_lever() {
        assert_eval!(+ - weak_lever, 1, 0, eval);
    }

    #[test]
    #[ignore = "unimplemented evaluation function"]
    #[evaluation_test("1r3q1R/2n4n/p2knpRp/1Qp2PPB/1bP2q1r/p4n1P/P1P2P2/2B1N1RK b kq - 0 7")]
    fn test_blocked() {
        assert_eval!(+ - blocked, 1, 2, eval);
    }

    #[test]
    #[ignore = "unimplemented evaluation function"]
    #[evaluation_test("1r3q1R/2n4n/p2knpRp/pQp2PPB/1bP2q1r/5n1P/P1P2P2/2B1N1RK b kq - 0 7")]
    fn test_doubled_isolated() {
        assert_eval!(+ - doubled_isolated, 1, 1, eval);
    }

    #[test]
    #[ignore = "unimplemented evaluation function"]
    #[evaluation_test("1r3q1R/2n4n/p2knpRp/pQp2PPB/1bP2q1r/5n1P/P1P2P2/2B1N1RK b kq - 0 7")]
    fn test_pawns_mg() {
        assert_eval!(pawns_mg, 113, -42, eval);
    }

    #[test]
    #[ignore = "unimplemented evaluation function"]
    #[evaluation_test("1r3q1R/2n4n/p2knpRp/pQp2PPB/1bP2q1r/5n1P/P1P2P2/2B1N1RK b kq - 0 7")]
    fn test_pawns_eg() {
        assert_eval!(pawns_eg, -74, -172, eval);
    }
}
