use proc_macro_utils::evaluation_fn;

use crate::{board::{coord::Coord, piece::Piece}, precomp::PrecomputedData, prelude::BitBoard};
use super::Evaluation;


impl<'a> Evaluation<'a> {
    /// The number of friendly bishops.
    #[evaluation_fn]
    pub fn bishop_count(&self) -> i32 {
        self.board.piece_bitboards[self.color.piece(Piece::BISHOP)].count() as i32
    }

    /// The number of friendly queens.
    #[evaluation_fn]
    pub fn queen_count(&self) -> i32 {
        self.board.piece_bitboards[self.color.piece(Piece::QUEEN)].count() as i32
    }

    /// The number of friendly pawns.
    #[evaluation_fn]
    pub fn pawn_count(&self) -> i32 {
        self.board.piece_bitboards[self.color.piece(Piece::PAWN)].count() as i32
    }

    /// The number of friendly knights.
    #[evaluation_fn]
    pub fn knight_count(&self) -> i32 {
        self.board.piece_bitboards[self.color.piece(Piece::KNIGHT)].count() as i32
    }

    /// The number of friendly rooks.
    #[evaluation_fn]
    pub fn rook_count(&self) -> i32 {
        self.board.piece_bitboards[self.color.piece(Piece::ROOK)].count() as i32
    }

    /// Whether or not we have bishops of opposite colors (one on light squares one on dark squares).
    pub fn opposite_bishops(&self) -> bool {
        let mut bb1 = self.board.piece_bitboards[self.color.piece(Piece::BISHOP)];
        let mut bb2 = self.board.piece_bitboards[self.color.flip().piece(Piece::BISHOP)];
        if bb1.count() == 1 && bb2.count() == 1 {
            Coord::from_idx(bb1.pop_lsb() as i8).is_light_square()
                != Coord::from_idx(bb2.pop_lsb() as i8).is_light_square()
        } else {
            false
        }
    }

    /// Counts the distance to the friendly king. 
    ///
    /// Requires: `king_square`
    #[evaluation_fn]
    pub fn king_distance(&self, sqr: Coord) -> i32 {
        self.precomp.king_distance[self.friendly_king_square()][sqr] as i32
    }

    /// The enemy's king ring. Squares defended by two pawns are excluded. 
    #[evaluation_fn]
    pub fn king_ring(&self, full: bool) -> BitBoard {;
        let mut king_ring = self.precomp.king_ring[self.enemy_king_square()];
        if full { return king_ring };

        let pawns = self.board.piece_bitboards[self.color.flip().piece(Piece::PAWN)];
        let offset = self.precomp.pawn_attack_dirs[self.color.flip()];
        let attacked = pawns.shifted(offset[0].offset()) & pawns.shifted(offset[1].offset());

        king_ring & !attacked
    }

    /// The number of friendly pieces.
    #[evaluation_fn]
    pub fn piece_count(&self) -> i32 {
        self.board.color_bitboards[self.color].count() as i32
    }

    #[evaluation_fn]
    pub fn pawn_attacks_span(&self) -> BitBoard {
        todo!();
    }
}


#[cfg(test)]
mod tests {
    use crate::eval::test_prelude::*;
    use super::*;

    #[test]
    #[evaluation_test("nb3b1R/p1pkn2p/1p2Rpn1/n6B/1bPP1qpP/QP2r1P1/P1P2P2/2BN2RK b Qkq - 1 2")]
    fn test_bishop_count() {
        assert_eval!(- friendly_bishop_count, 2, 3, eval);
    }

    #[test]
    #[evaluation_test("nb3b1R/p1pkn2p/1p2Rpn1/nQ5B/1bPP1qpP/QP2r1P1/P1P2P2/2BN2RK b Qkq - 1 2")]
    fn test_queen_count() {
        assert_eval!(- friendly_queen_count, 2, 1, eval);
    }

    #[test]
    #[evaluation_test("nb3b1R/p1pkn2p/1p2Rpn1/nQ5B/1bPP1qpP/QP2r1P1/P1P2P2/2BN2RK b Qkq - 1 2")]
    fn test_pawn_count() {
        assert_eval!(- friendly_pawn_count, 8, 6, eval);
    }

    #[test]
    #[evaluation_test("nb3b1R/p1pkn2p/1p2Rpn1/nQ5B/1bPP1qpP/QP2r1P1/P1P2P2/2BN2RK b Qkq - 1 2")]
    fn test_knight_count() {
        assert_eval!(- friendly_knight_count, 1, 4, eval);
    }

    #[test]
    #[evaluation_test("nb3b1R/p1pkn2p/1p2Rpn1/nQ5B/1bPP1qpP/QP2r1P1/P1P2P2/2BN2RK b Qkq - 1 2")]
    fn test_rook_count() {
        assert_eval!(- friendly_rook_count, 3, 1, eval);
    }

    #[test]
    #[evaluation_test("n1b1r3/4p1Q1/1q2pP2/kpp4r/P1P4r/R1B1N2P/P4P1R/4RnK1 b kq - 2 11")]
    fn test_opposite_bishops() {
        assert_eval!(- opposite_bishops, true, true, eval);
    }

    #[test]
    #[evaluation_test("nb3b1R/p1pkn2p/1p2Rpn1/nQ5B/1bPP1qpP/QP2r1P1/P1P2P2/2BN2RK b Qkq - 1 2")]
    fn test_king_distance() {
        assert_eval!(friendly_king_distance, 308, 215, eval);
    }

    #[test]
    #[evaluation_test("nb3b1R/p1pkn3/1p2Rpn1/nQ5B/1bPP1qpP/QP2r3/P1P2P1P/2BN2RK w Qkq - 2 3")]
    fn test_king_ring() {
        assert_eval!(+ friendly_king_ring, [6, 2], 0, 0, eval; false);
    }

    #[test]
    #[evaluation_test("nb3b1R/p1pkn3/1p2Rpn1/nQ5B/1bPP1qpP/QP2r3/P1P2P1P/2BN2RK w Qkq - 2 3")]
    fn test_piece_count() {
        assert_eval!(- friendly_piece_count, 17, 15, eval);
    }

    #[test]
    #[ignore = "unimplemented evaluation function"]
    #[evaluation_test("nb3b1R/p1pkn3/n3Rpn1/pQ5B/1bPP1qpP/QP2r3/P1P2P1P/2BN2RK b Qkq - 3 3")]
    fn test_pawn_attacks_span() {
        // assert_eval!(+ pawn_attacks_span, 28, 32, eval);
    }
}
