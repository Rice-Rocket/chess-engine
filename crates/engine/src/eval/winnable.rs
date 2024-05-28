use proc_macro_utils::evaluation_fn;

use crate::{board::{coord::Coord, piece::Piece}, color::{Black, Color, White}, prelude::BitBoard};
use super::Evaluation;


impl<'a> Evaluation<'a> {
    pub fn winnable<W: Color, B: Color>(&self) -> i32 {
        let pawns = self.board.piece_bitboards[W::piece(Piece::PAWN)] 
            | self.board.piece_bitboards[B::piece(Piece::PAWN)];
        let friendly_king = self.king_square::<W, B>();
        let enemy_king = self.king_square::<B, W>();
        let left_flank = (BitBoard::from_ranks(0..=3) & pawns).0 > 0;
        let right_flank = (BitBoard::from_ranks(4..=7) & pawns).0 > 0;

        let friendly_king_rank = W::rank(friendly_king.rank());
        let enemy_king_rank = W::rank(enemy_king.rank());

        let passed = (self.candidate_passed[W::index()].count() + self.candidate_passed[B::index()].count()) as i32;
        let both_flanks = if left_flank && right_flank { 1 } else { 0 };
        let outflanking = ((friendly_king.file() - enemy_king.file()).abs() - (friendly_king.rank() - enemy_king.rank()).abs()) as i32;
        let pure_pawn = if (self.non_pawn_material::<W, B>() + self.non_pawn_material::<B, W>()) == 0 { 1 } else { 0 };
        let almost_unwinnable = outflanking < 0 && both_flanks == 0;
        let infiltration = if friendly_king_rank > 3 || enemy_king_rank < 4 { 1 } else { 0 };

        9 * passed
            + 12 * pawns.count() as i32
            + 9 * outflanking
            + 21 * both_flanks
            + 24 * infiltration
            + 51 * pure_pawn
            - 43 * if almost_unwinnable { 1 } else { 0 }
            - 110
    }

    /// Returns `(mg, eg)`
    pub fn winnable_total<W: Color, B: Color>(&self, v: i32) -> (i32, i32) {
        let sign = match v {
            0.. => 1,
            0 => 0,
            ..=-1 => -1,
        };
        (
            sign * (self.winnable::<W, B>() + 50).min(0).max(-v.abs()),
            sign * self.winnable::<W, B>().max(-v.abs()),
        )
    }
}


#[cfg(test)]
mod tests {
    use crate::eval::test_prelude::*;
    use super::*;

    #[test]
    #[evaluation_test("n3r3/2p1p1QK/p2n4/2p1bP1r/PkPB3r/R2BN2P/Pq3P1R/1B2Rn2 b kq - 2 10")]
    fn test_winnable() {
        assert_eval!(- winnable, 91, 91, eval);
    }
}
