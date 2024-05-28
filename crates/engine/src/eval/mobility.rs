use proc_macro_utils::evaluation_fn;

use crate::{bitboard::square_values::SquareEvaluations, board::{coord::Coord, piece::Piece}, color::{Black, Color, White}, prelude::BitBoard};
use super::Evaluation;


impl<'a> Evaluation<'a> {
    pub fn mobility<W: Color, B: Color>(&self) -> SquareEvaluations {
        let mut eval = SquareEvaluations::new();
        let mut pieces = self.board.color_bitboards[W::index()]
            & !self.board.piece_bitboards[W::piece(Piece::PAWN)]
            & !self.board.piece_bitboards[W::piece(Piece::KING)];

        let area = self.mobility_area::<W, B>();
        let area_no_queen = area & !self.board.piece_bitboards[W::piece(Piece::QUEEN)];

        while pieces.0 != 0 {
            let sqr = Coord::from_idx(pieces.pop_lsb() as i8);
            eval[sqr] = if self.board.piece_bitboards[W::piece(Piece::KNIGHT)].contains_square(sqr.square()) {
                (self.knight_attack_from::<W, B>(sqr) & area_no_queen).count() as i32
            } else if self.board.piece_bitboards[W::piece(Piece::BISHOP)].contains_square(sqr.square()) {
                (self.bishop_xray_attack_from::<W, B>(sqr) & area_no_queen).count() as i32
            } else if self.board.piece_bitboards[W::piece(Piece::ROOK)].contains_square(sqr.square()) {
                (self.rook_xray_attack_from::<W, B>(sqr) & area).count() as i32
            } else {
                (self.queen_attack_from::<W, B>(sqr) & area).count() as i32
            };
        }

        eval
    }

    pub fn mobility_area<W: Color, B: Color>(&self) -> BitBoard {
        !self.board.piece_bitboards[W::piece(Piece::KING)]
            & !self.board.piece_bitboards[W::piece(Piece::QUEEN)]
            & !self.all_pawn_attacks[B::index()].0
            & !(self.board.piece_bitboards[W::piece(Piece::PAWN)] 
                & (BitBoard::from_ranks(W::ranks(1..=2)) | self.board.all_pieces_bitboard.shifted_2d(W::offset(0, -1))))
            & !self.blockers_for_king::<B, W>()
    }

    const MOBILITY_BONUS_MG: [[i32; 28]; 4] = [
        [-62, -53, -12, -4,  3, 13, 22, 28,  33,  0,  0,  0,  0,   0,   0,  0,  0,  0,  0,  0,  0,   0,   0,   0,   0,   0,   0,   0],
        [-48, -20,  16, 26, 38, 51, 55, 63,  63, 68, 81, 81, 91,  98,   0,  0,  0,  0,  0,  0,  0,   0,   0,   0,   0,   0,   0,   0],
        [-60, -20,   2,  3,  3, 11, 22, 31,  40, 40, 41, 48, 57,  57,  62,  0,  0,  0,  0,  0,  0,   0,   0,   0,   0,   0,   0,   0],
        [-30, -12,  -8, -9, 20, 23, 23, 35,  38, 53, 64, 65, 65,  66,  67, 67, 72, 72, 77, 79, 93, 108, 108, 108, 110, 114, 114, 116],
    ];
    const MOBILITY_BONUS_EG: [[i32; 28]; 4] = [
        [-81, -56, -31, -16,  5, 11,  17,  20,  25,   0,   0,   0,   0,   0,    0,   0,   0,   0,   0,   0,   0,   0,   0,   0,   0,   0,   0,   0],
        [-59, -23,  -3,  13, 24, 42,  54,  57,  65,  73,  78,  86,  88,  97,    0,   0,   0,   0,   0,   0,   0,   0,   0,   0,   0,   0,   0,   0],
        [-78, -17,  23,  39, 70, 99, 103, 121, 134, 139, 158, 164, 168, 169,  172,   0,   0,   0,   0,   0,   0,   0,   0,   0,   0,   0,   0,   0],
        [-48, -30,  -7,  19, 40, 55,  59,  75,  78,  96,  96, 100, 121, 127,  131, 133, 136, 141, 147, 150, 151, 168, 168, 171, 182, 182, 192, 219],
    ];

    /// Returns `(mg, eg)`
    pub fn mobility_bonus<W: Color, B: Color>(&self) -> (i32, i32) {
        let mut knights = self.board.piece_bitboards[W::piece(Piece::KNIGHT)];
        let mut bishops = self.board.piece_bitboards[W::piece(Piece::BISHOP)];
        let mut rooks = self.board.piece_bitboards[W::piece(Piece::ROOK)];
        let mut queens = self.board.piece_bitboards[W::piece(Piece::QUEEN)];

        let mobility = self.mobility::<W, B>();
        let mut mg = 0;
        let mut eg = 0;

        while knights.0 != 0 {
            let sqr = Coord::from_idx(knights.pop_lsb() as i8);
            mg += Self::MOBILITY_BONUS_MG[0][mobility[sqr] as usize];
            eg += Self::MOBILITY_BONUS_EG[0][mobility[sqr] as usize];
        }

        while bishops.0 != 0 {
            let sqr = Coord::from_idx(bishops.pop_lsb() as i8);
            mg += Self::MOBILITY_BONUS_MG[1][mobility[sqr] as usize];
            eg += Self::MOBILITY_BONUS_EG[1][mobility[sqr] as usize];
        }

        while rooks.0 != 0 {
            let sqr = Coord::from_idx(rooks.pop_lsb() as i8);
            mg += Self::MOBILITY_BONUS_MG[2][mobility[sqr] as usize];
            eg += Self::MOBILITY_BONUS_EG[2][mobility[sqr] as usize];
        }

        while queens.0 != 0 {
            let sqr = Coord::from_idx(queens.pop_lsb() as i8);
            mg += Self::MOBILITY_BONUS_MG[3][mobility[sqr] as usize];
            eg += Self::MOBILITY_BONUS_EG[3][mobility[sqr] as usize];
        }

        (mg, eg)
    }
}


#[cfg(test)]
mod tests {
    use crate::eval::test_prelude::*;
    use super::*;

    #[test]
    #[evaluation_test("1r3q1R/p1p1n2n/n2k1pR1/pQ3P1B/1bP2qpr/QP3n1P/P1P1P3/2B1N1RK w kq - 9 6")]
    fn test_mobility() {
        assert_eval!(+ - mobility, 41, 48, eval);
    }

    #[test]
    #[evaluation_test("1r3q1R/p1p1n2n/n2k1pR1/pQ3P1B/1bP2qpr/QP3n1P/P1P1P3/2B1N1RK w kq - 9 6")]
    fn test_mobility_area() {
        assert_eval!(+ - mobility_area, 49, 47, eval);
    }

    #[test]
    #[evaluation_test("1r3q1R/p1p1n2n/n2k1pR1/pQ3P1B/1bP2qpr/QP3n1P/P1P1P3/2B1N1RK w kq - 9 6")]
    fn test_mobility_bonus() {
        assert_eval!(- mobility_bonus, (193, 467), (158, 293), eval);
    }
}
