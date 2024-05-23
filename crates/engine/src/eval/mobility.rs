use proc_macro_utils::evaluation_fn;

use crate::{bitboard::square_values::SquareEvaluations, board::coord::Coord, color::{Black, Color, White}, prelude::BitBoard};
use super::Evaluation;


impl<'a> Evaluation<'a> {
    pub fn mobility<W: Color, B: Color>(&self) -> SquareEvaluations {
        todo!();
    }

    pub fn mobility_area<W: Color, B: Color>(&self) -> BitBoard {
        todo!();
    }

    pub fn mobility_bonus<W: Color, B: Color>(&self, mg: bool) -> SquareEvaluations {
        todo!();
    }

    #[inline]
    pub fn mobility_mg<W: Color, B: Color>(&self) -> SquareEvaluations {
        todo!();
    }

    #[inline]
    pub fn mobility_eg<W: Color, B: Color>(&self) -> SquareEvaluations {
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
    fn test_mobility() {
        assert_eval!(+ - mobility, 41, 48, eval);
    }

    #[test]
    #[ignore = "unimplemented evaluation function"]
    #[evaluation_test("1r3q1R/p1p1n2n/n2k1pR1/pQ3P1B/1bP2qpr/QP3n1P/P1P1P3/2B1N1RK w kq - 9 6")]
    fn test_mobility_area() {
        assert_eval!(+ - mobility_area, 49, 47, eval);
    }

    #[test]
    #[ignore = "unimplemented evaluation function"]
    #[evaluation_test("1r3q1R/p1p1n2n/n2k1pR1/pQ3P1B/1bP2qpr/QP3n1P/P1P1P3/2B1N1RK w kq - 9 6")]
    fn test_mobility_bonus() {
        assert_eval!(+ - mobility_bonus, 193, 158, eval; true);
    }
}
