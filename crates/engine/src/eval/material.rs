use proc_macro_utils::evaluation_fn;

use crate::{board::coord::Coord, color::{Color, White, Black}};
use super::Evaluation;


impl<'a> Evaluation<'a> {
    pub fn non_pawn_material<W: Color, B: Color>(&self) -> i32 {
        Self::PIECE_VALUE_BONUS_MG[1] * self.knight_count::<W, B>()
            + Self::PIECE_VALUE_BONUS_MG[2] * self.bishop_count::<W, B>()
            + Self::PIECE_VALUE_BONUS_MG[3] * self.rook_count::<W, B>()
            + Self::PIECE_VALUE_BONUS_MG[4] * self.queen_count::<W, B>()
    }

    const PIECE_VALUE_BONUS_MG: [i32; 5] = [124, 781, 825, 1276, 2538];
    const PIECE_VALUE_BONUS_EG: [i32; 5] = [206, 854, 915, 1380, 2682];
    pub fn piece_value_bonus<W: Color, B: Color>(&self, mg: bool, sqr: Coord) -> i32 {
        todo!();
    }

    pub fn psqt_bonus<W: Color, B: Color>(&self, mg: bool, sqr: Coord) -> i32 {
        todo!();
    }

    #[inline]
    pub fn piece_value_mg<W: Color, B: Color>(&self, sqr: Coord) -> i32 {
        todo!();
    }

    #[inline]
    pub fn piece_value_eg<W: Color, B: Color>(&self, sqr: Coord) -> i32 {
        todo!();
    }

    #[inline]
    pub fn psqt_mg<W: Color, B: Color>(&self, sqr: Coord) -> i32 {
        todo!();
    }

    #[inline]
    pub fn psqt_eg<W: Color, B: Color>(&self, sqr: Coord) -> i32 {
        todo!();
    }
}


#[cfg(test)]
mod tests {
    use crate::eval::test_prelude::*;
    use super::*;

    #[test]
    #[ignore = "unimplemented evaluation function"]
    #[evaluation_test("1r3q1R/p1p1n2n/n2k1pR1/pQ3P1B/1bP2qpr/QP3n1P/P1P1P3/2B1N1RK w kq - 9 6")]
    fn test_non_pawn_material() {
        assert_eval!(- non_pawn_material, 11335, 11577, eval);
    }

    #[test]
    #[ignore = "unimplemented evaluation function"]
    #[evaluation_test("1r3q1R/p1p1n2n/n2k1pR1/pQ3P1B/1bP2qpr/QP3n1P/P1P1P3/2B1N1RK w kq - 9 6")]
    fn test_piece_value_bonus() {
        assert_eval!(piece_value_bonus, 12203, 12197, eval; true);
    }

    #[test]
    #[ignore = "unimplemented evaluation function"]
    #[evaluation_test("1r3q1R/p1p1n2n/n2k1pR1/pQ3P1B/1bP2qpr/QP3n1P/P1P1P3/2B1N1RK w kq - 9 6")]
    fn test_psqt_bonus() {
        assert_eval!(psqt_bonus, 146, 32, eval; true);
    }
}
