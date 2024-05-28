use proc_macro_utils::evaluation_fn;

use crate::{bitboard::square_values::SquareEvaluations, board::{coord::Coord, piece::Piece}, color::{Black, Color, White}};
use super::Evaluation;


impl<'a> Evaluation<'a> {
    const Q_OURS: [[i32; 6]; 5] = [
        [  40,  38,   0,   0,    0,  0],
        [  32, 255, -62,   0,    0,  0],
        [  0,  104,   4,   0,    0,  0],
        [ -26,  -2,  47, 105, -208,  0],
        [-189,  24, 117, 133, -134, -6]
    ];
    const Q_THEIRS: [[i32; 6]; 5] = [
        [36,   0,   0,   0,   0, 0],
        [ 9,  63,   0,   0,   0, 0],
        [59,  65,  42,   0,   0, 0],
        [46,  39,  24, -24,   0, 0],
        [97, 100, -42, 137, 268, 0]
    ];
    pub fn imbalance<W: Color, B: Color>(&self) -> SquareEvaluations {
        let mut eval = SquareEvaluations::new();
        let mut sqrs = self.board.color_bitboards[W::index()] & !self.board.piece_bitboards[W::piece(Piece::KING)];

        let bishops = (self.bishop_count::<B, W>(), self.bishop_count::<W, B>());

        while sqrs.0 != 0 {
            let sqr = Coord::from_idx(sqrs.pop_lsb() as i8);
            let pval = self.board.square[sqr].piece_type();
            let mut friendly = self.board.color_bitboards[W::index()] & !self.board.piece_bitboards[W::piece(Piece::KING)];
            let mut enemy = self.board.color_bitboards[B::index()] & !self.board.piece_bitboards[B::piece(Piece::KING)];
            let mut v = 0;

            while friendly.0 != 0 {
                let s = Coord::from_idx(friendly.pop_lsb() as i8);
                let pv = self.board.square[s].piece_type();
                if pv > pval { continue };
                // if pval == Piece::NONE || pval == Piece::KING { continue };
                v += Self::Q_OURS[pval as usize - 1][pv as usize];
            }

            while enemy.0 != 0 {
                let s = Coord::from_idx(enemy.pop_lsb() as i8);
                let pv = self.board.square[s].piece_type();
                if pv > pval { continue };
                // if pval == Piece::NONE || pval == Piece::KING { continue };
                v += Self::Q_THEIRS[pval as usize - 1][pv as usize];
            }

            if bishops.0 > 1 { v += Self::Q_THEIRS[pval as usize - 1][0] };
            if bishops.1 > 1 { v += Self::Q_OURS[pval as usize - 1][0] };

            eval[sqr] = v;
        }

        eval
    }

    pub fn bishop_pair<W: Color, B: Color>(&self) -> i32 {
        if self.bishop_count::<W, B>() > 1 {
            1438
        } else {
            0
        }
    }

    pub fn imbalance_total<W: Color, B: Color>(&self) -> i32 {
        let mut v = 0;

        v += self.imbalance::<W, B>().count() - self.imbalance::<B, W>().count();
        v += self.bishop_pair::<W, B>() - self.bishop_pair::<B, W>();

        v / 16
    }
}


#[cfg(test)]
mod tests {
    use crate::eval::test_prelude::*;
    use super::*;

    #[test]
    #[evaluation_test("nb3b1R/p1pkn3/n3Rpn1/pQ5B/1bPP1qpP/QP2r3/P1P2P1P/2BN2RK b Qkq - 3 3")]
    fn test_imbalance() {
        assert_eval!(+ - imbalance, 9878, 14273, eval);
    }

    #[test]
    #[evaluation_test("nr3q1R/p1pkn3/n3Rpn1/pQ5B/1bPP1qpP/QP2r3/P1P2P1P/2BN2RK b Qkq - 3 3")]
    fn test_bishop_pair() {
        assert_eval!(- bishop_pair, 1438, 0, eval);
    }

    #[test]
    #[evaluation_test("nr3q1R/p1pkn3/n3Rpn1/pQ5B/1bPP1qpP/QP2r3/P1P2P1P/2BN2RK b Qkq - 3 3")]
    fn test_imbalance_total() {
        assert_eval!(- imbalance_total, -181, 181, eval);
    }
}
