use proc_macro_utils::evaluation_fn;

use crate::{board::{coord::Coord, piece::Piece}, color::{Color, White, Black}, prelude::BitBoard};
use super::Evaluation;


impl<'a> Evaluation<'a> {
    pub fn outpost<W: Color, B: Color>(&self) -> BitBoard {
        self.outpost_square::<W, B>() 
            & (self.board.piece_bitboards[W::piece(Piece::KNIGHT)] 
            | self.board.piece_bitboards[W::piece(Piece::BISHOP)])
    }

    pub fn outpost_square<W: Color, B: Color>(&self) -> BitBoard {
        BitBoard::from_ranks(W::ranks(4..=5))
            & self.all_pawn_attacks::<W, B>().0
            & !self.pawn_attacks_span::<W, B>()
    }

    /// Returns `(not supported by pawn, supported by pawn)`
    pub fn reachable_outpost<W: Color, B: Color>(&self) -> (BitBoard, BitBoard) {
        let mut reachable = (self.all_knight_attacks::<W, B>().0 | self.all_bishop_xray_attacks::<W, B>().0)
            & self.outpost_square::<W, B>() & !self.board.color_bitboards[W::index()];
        let supported = reachable & self.all_pawn_attacks::<W, B>().0;
        let mut supported_origins = BitBoard(0);
        let mut origins = BitBoard(0);

        while reachable.0 != 0 {
            let sqr = Coord::from_idx(reachable.pop_lsb() as i8);
            let has_support = supported.contains_square(sqr.square());
            let attacks = self.bishop_xray_attack::<W, B>(None, sqr) | self.knight_attack::<W, B>(None, sqr);
            if has_support {
                supported_origins |= attacks;
            } else {
                origins |= attacks;
            }
        }

        (origins, supported_origins)
    }

    pub fn minor_behind_pawn<W: Color, B: Color>(&self) -> BitBoard {
        todo!();
    }

    pub fn bishop_pawns<W: Color, B: Color>(&self, sqr: Coord) -> i32 {
        todo!();
    }

    /// Returns `(on open file, on semi-open file)`
    ///
    /// A semi-open file is defined as having only enemy pawns.
    pub fn rook_on_file<W: Color, B: Color>(&self) -> (BitBoard, BitBoard) {
        todo!();
    }

    pub fn trapped_rook<W: Color, B: Color>(&self) -> BitBoard {
        todo!();
    }

    pub fn weak_queen<W: Color, B: Color>(&self) -> BitBoard {
        todo!();
    }

    pub fn king_protector<W: Color, B: Color>(&self, sqr: Coord) -> i32 {
        todo!();
    }

    pub fn long_diagonal_bishop<W: Color, B: Color>(&self) -> BitBoard {
        todo!();
    }

    // Maybe return array of 4 bitboards? 
    pub fn outpost_total<W: Color, B: Color>(&self, sqr: Coord) -> i32 {
        todo!();
    }

    pub fn rook_on_queen_file<W: Color, B: Color>(&self) -> BitBoard {
        todo!();
    }

    pub fn bishop_xray_pawns<W: Color, B: Color>(&self, sqr: Coord) -> i32 {
        todo!();
    }

    pub fn rook_on_king_ring<W: Color, B: Color>(&self) -> BitBoard {
        todo!();
    }

    pub fn bishop_on_king_ring<W: Color, B: Color>(&self) -> BitBoard {
        todo!();
    }

    pub fn queen_infiltration<W: Color, B: Color>(&self) -> BitBoard {
        todo!();
    }

    pub fn pieces_mg<W: Color, B: Color>(&self, sqr: Coord) -> i32 {
        todo!();
    }

    pub fn pieces_eg<W: Color, B: Color>(&self, sqr: Coord) -> i32 {
        todo!();
    }
}


#[cfg(test)]
mod tests {
    use crate::eval::test_prelude::*;
    use super::*;

    #[test]
    #[evaluation_test("r2qk2r/6p1/1ppNp3/p1Pn1pNp/Pb1PnPbP/6P1/1P2P3/R1BQKB1R b KQkq - 1 2")]
    fn test_outpost() {
        assert_eval!(+ - outpost, 2, 3, eval);
    }

    #[test]
    #[evaluation_test("1r3q1R/2n4n/p2knpRp/pQp2PPB/1bP2q1r/5n1P/P1P2P2/2B1N1RK b kq - 0 7")]
    fn test_outpost_square() {
        assert_eval!(+ - outpost_square, 5, 0, eval);
    }

    #[test]
    #[evaluation_test("r2qk2r/6p1/1pp1p3/p1Pn1b1p/PbNPnP1P/5NP1/1P2P3/R1BQKB1R w KQkq - 2 3")]
    fn test_reachable_outpost() {
        assert_eval!(* - [0, 1] reachable_outpost, (0, 2), (0, 1), eval);
    }

    #[test]
    #[ignore = "unimplemented evaluation function"]
    #[evaluation_test("1r3q1R/2n5/p2knpRp/pQpn1PPB/1bP2q1r/5n1P/P1P2P2/2B1N1RK b kq - 0 7")]
    fn test_minor_behind_pawn() {
        assert_eval!(+ - minor_behind_pawn, 2, 1, eval);
    }

    #[test]
    #[ignore = "unimplemented evaluation function"]
    #[evaluation_test("1r3q1R/3b4/p2knpRp/pQpn1PPB/1bP2q1r/5n1P/P1P2P2/2B1N1RK w kq - 1 8")]
    fn test_bishop_pawns() {
        assert_eval!(bishop_pawns, 28, 11, eval);
    }

    #[test]
    #[ignore = "unimplemented evaluation function"]
    #[evaluation_test("1r3q1R/3b4/pP1knpRp/pQpn1P1B/1b3q1r/5n1P/P1P2P1P/2B1N1RK w kq - 1 8")]
    fn test_rook_on_file() {
        assert_eval!(* - [0, 1] rook_on_file, (2, 0), (0, 1), eval);
    }

    #[test]
    #[ignore = "unimplemented evaluation function"]
    #[evaluation_test("1r3q1R/1k1b4/p3npRp/pQpn1P1B/1bP2q1r/5n1P/P1P2P2/2B1N1KR w kq - 1 8")]
    fn test_trapped_rook() {
        assert_eval!(+ - trapped_rook, 1, 0, eval);
    }

    #[test]
    #[ignore = "unimplemented evaluation function"]
    #[evaluation_test("1r3q1R/1k1b4/p3npRp/pQpn1P1B/1bP4r/5n1P/P1P1qP2/2B1N1KR w kq - 1 8")]
    fn test_weak_queen() {
        assert_eval!(+ - weak_queen, 1, 1, eval);
    }

    #[test]
    #[ignore = "unimplemented evaluation function"]
    #[evaluation_test("1r3q1R/1k6/p1b1npRp/pQpn1P2/1bP4r/2B2n1P/P1P1qPB1/4N1KR w kq - 1 8")]
    fn test_long_diagonal_bishop() {
        assert_eval!(+ - long_diagonal_bishop, 2, 1, eval);
    }

    #[test]
    #[ignore = "unimplemented evaluation function"]
    #[evaluation_test("r2qk2r/6p1/1pp1p3/p1Pn1bNp/PbNPnP1P/6P1/1P2P3/R1BQKB1R w KQkq - 2 3")]
    fn test_outpost_total() {
        assert_eval!(outpost_total, 5, 3, eval);
    }

    #[test]
    #[ignore = "unimplemented evaluation function"]
    #[evaluation_test("1r3q1R/1k2R3/p1b1np1p/pQpnRP2/1bP4r/2B2n1P/P1P1qPB1/4N1K1 w kq - 1 8")]
    fn test_rook_on_queen_file() {
        assert_eval!(+ - rook_on_queen_file, 2, 1, eval);
    }

    #[test]
    #[ignore = "unimplemented evaluation function"]
    #[evaluation_test("1r1B1q1R/1k2R3/p3np1p/pQpnRP2/2P4r/1bB2n1P/P1P1qPB1/4N1K1 w kq - 1 8")]
    fn test_bishop_xray_pawns() {
        assert_eval!(bishop_xray_pawns, 4, 3, eval);
    }

    #[test]
    #[ignore = "unimplemented evaluation function"]
    #[evaluation_test("k2q4/3r2p1/1pprp3/p1Pn1bNp/PbNPnP1P/6P1/1P2PR2/1RBQKB2 w KQkq - 2 3")]
    fn test_rook_on_king_ring() {
        assert_eval!(+ - rook_on_king_ring, 1, 2, eval);
    }

    #[test]
    #[ignore = "unimplemented evaluation function"]
    #[evaluation_test("1r1B1q1R/1k6/p2Rnp1p/pQpnbP2/2P2B1r/5n1P/P1P1qPBR/4N1K1 w kq - 1 8")]
    fn test_bishop_on_king_ring() {
        assert_eval!(+ - bishop_on_king_ring, 2, 1, eval);
    }

    #[test]
    #[ignore = "unimplemented evaluation function"]
    #[evaluation_test("1r1B1q1R/1k1np1Q1/p5Rp/p1pnbP2/2P2B1r/5n1P/P1P1qPBR/4N1K1 w kq - 1 8")]
    fn test_queen_infiltration() {
        assert_eval!(+ - queen_infiltration, 1, 1, eval);
    }

    #[test]
    #[ignore = "unimplemented evaluation function"]
    #[evaluation_test("nr1B1q2/1k2p1Q1/p5Rp/p1pnbP2/R1P2B1r/2P2n1P/P3qPBR/4N1K1 w kq - 1 8")]
    fn test_pieces_mg() {
        assert_eval!(pieces_mg, -121, -14, eval);
    }

    #[test]
    #[ignore = "unimplemented evaluation function"]
    #[evaluation_test("nr1B1q2/1k2p1Q1/p5Rp/p1pnbP2/R1P2B1r/2P2n1P/P3qPBR/4N1K1 w kq - 1 8")]
    fn test_pieces_eg() {
        assert_eval!(pieces_eg, -325, -105, eval);
    }
}
