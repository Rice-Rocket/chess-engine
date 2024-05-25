use proc_macro_utils::evaluation_fn;

use crate::{bitboard::square_values::SquareEvaluations, board::{coord::Coord, piece::Piece}, color::{Black, Color, White}, precomp::Precomputed, prelude::BitBoard};
use super::Evaluation;


impl<'a> Evaluation<'a> {
    /// The number of friendly bishops.
    pub fn bishop_count<W: Color, B: Color>(&self) -> i32 {
        self.board.piece_bitboards[W::piece(Piece::BISHOP)].count() as i32
    }

    /// The number of friendly queens.
    pub fn queen_count<W: Color, B: Color>(&self) -> i32 {
        self.board.piece_bitboards[W::piece(Piece::QUEEN)].count() as i32
    }

    /// The number of friendly pawns.
    pub fn pawn_count<W: Color, B: Color>(&self) -> i32 {
        self.board.piece_bitboards[W::piece(Piece::PAWN)].count() as i32
    }

    /// The number of friendly knights.
    pub fn knight_count<W: Color, B: Color>(&self) -> i32 {
        self.board.piece_bitboards[W::piece(Piece::KNIGHT)].count() as i32
    }

    /// The number of friendly rooks.
    pub fn rook_count<W: Color, B: Color>(&self) -> i32 {
        self.board.piece_bitboards[W::piece(Piece::ROOK)].count() as i32
    }

    /// Whether or not we have bishops of opposite colors (one on light squares one on dark squares).
    pub fn opposite_bishops(&self) -> bool {
        let mut bb1 = self.board.piece_bitboards[Piece::new(Piece::BISHOP | Piece::WHITE)];
        let mut bb2 = self.board.piece_bitboards[Piece::new(Piece::BISHOP | Piece::BLACK)];
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
    pub fn king_distance<W: Color, B: Color>(&self) -> SquareEvaluations {
        let mut eval = SquareEvaluations::new();

        for sqr in Coord::iter_squares() {
            eval[sqr] = self.precomp.king_distance[self.king_square::<W, B>()][sqr] as i32;
        }

        eval
    }

    /// The enemy's king ring. Squares defended by two pawns are excluded. 
    pub fn king_ring<W: Color, B: Color>(&self, full: bool) -> BitBoard {;
        let mut king_ring = self.precomp.king_ring[self.king_square::<B, W>()];
        if full { return king_ring };

        let pawns = self.board.piece_bitboards[B::piece(Piece::PAWN)];
        let offset = self.precomp.pawn_attack_dirs[B::index()];
        let attacked = pawns.shifted(offset[0].offset()) & pawns.shifted(offset[1].offset());

        king_ring & !attacked
    }

    /// The number of friendly pieces.
    pub fn piece_count<W: Color, B: Color>(&self) -> i32 {
        self.board.color_bitboards[W::index()].count() as i32
    }

    pub fn pawn_attacks_span<W: Color, B: Color>(&self) -> BitBoard {
        let mut pawns = self.board.piece_bitboards[B::piece(Piece::PAWN)];
        let other_pawns = self.board.piece_bitboards[W::piece(Piece::PAWN)];
        let mut span = Precomputed::pawn_attacks(pawns, B::is_white());
        pawns &= !self.backward::<B, W>();

        while pawns.0 != 0 {
            let sqr = Coord::from_idx(pawns.pop_lsb() as i8);
            let pawn_span = self.precomp.pawn_attack_span[B::index()][sqr];
            let blockers = Precomputed::pawn_attacks((sqr.add_clamp(W::down())).to_bitboard(), B::is_white())
                & other_pawns & !self.backward::<W, B>();
            if (blockers & pawn_span).0 == 0 {
                span |= pawn_span;
            }
        }
        
        span
    }
}


#[cfg(test)]
mod tests {
    use crate::eval::test_prelude::*;
    use super::*;

    #[test]
    #[evaluation_test("nb3b1R/p1pkn2p/1p2Rpn1/n6B/1bPP1qpP/QP2r1P1/P1P2P2/2BN2RK b Qkq - 1 2")]
    fn test_bishop_count() {
        assert_eval!(- bishop_count, 2, 3, eval);
    }

    #[test]
    #[evaluation_test("nb3b1R/p1pkn2p/1p2Rpn1/nQ5B/1bPP1qpP/QP2r1P1/P1P2P2/2BN2RK b Qkq - 1 2")]
    fn test_queen_count() {
        assert_eval!(- queen_count, 2, 1, eval);
    }

    #[test]
    #[evaluation_test("nb3b1R/p1pkn2p/1p2Rpn1/nQ5B/1bPP1qpP/QP2r1P1/P1P2P2/2BN2RK b Qkq - 1 2")]
    fn test_pawn_count() {
        assert_eval!(- pawn_count, 8, 6, eval);
    }

    #[test]
    #[evaluation_test("nb3b1R/p1pkn2p/1p2Rpn1/nQ5B/1bPP1qpP/QP2r1P1/P1P2P2/2BN2RK b Qkq - 1 2")]
    fn test_knight_count() {
        assert_eval!(- knight_count, 1, 4, eval);
    }

    #[test]
    #[evaluation_test("nb3b1R/p1pkn2p/1p2Rpn1/nQ5B/1bPP1qpP/QP2r1P1/P1P2P2/2BN2RK b Qkq - 1 2")]
    fn test_rook_count() {
        assert_eval!(- rook_count, 3, 1, eval);
    }

    #[test]
    #[evaluation_test("n1b1r3/4p1Q1/1q2pP2/kpp4r/P1P4r/R1B1N2P/P4P1R/4RnK1 b kq - 2 11")]
    fn test_opposite_bishops() {
        assert_eval!(! - opposite_bishops, true, true, eval);
    }

    #[test]
    #[evaluation_test("nb3b1R/p1pkn2p/1p2Rpn1/nQ5B/1bPP1qpP/QP2r1P1/P1P2P2/2BN2RK b Qkq - 1 2")]
    fn test_king_distance() {
        assert_eval!(+ - king_distance, 308, 215, eval);
    }

    #[test]
    #[evaluation_test("nb3b1R/p1pkn3/1p2Rpn1/nQ5B/1bPP1qpP/QP2r3/P1P2P1P/2BN2RK w Qkq - 2 3")]
    fn test_king_ring() {
        assert_eval!(+ king_ring, [6, 2], 0, 0, eval; false);
    }

    #[test]
    #[evaluation_test("nb3b1R/p1pkn3/1p2Rpn1/nQ5B/1bPP1qpP/QP2r3/P1P2P1P/2BN2RK w Qkq - 2 3")]
    fn test_piece_count() {
        assert_eval!(- piece_count, 17, 15, eval);
    }

    #[test]
    #[evaluation_test("nb3b1R/p1pkn3/n3Rpn1/pQ5B/1bPP1qpP/QP2r3/P1P2P1P/2BN2RK b Qkq - 3 3")]
    fn test_pawn_attacks_span() {
        assert_eval!(+ - pawn_attacks_span, 28, 32, eval);
    }
}
