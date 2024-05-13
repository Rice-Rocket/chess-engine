use proc_macro_utils::evaluation_fn;

use crate::{board::coord::Coord, prelude::BitBoard};
use super::Evaluation;


impl<'a> Evaluation<'a> {
    #[evaluation_fn]
    pub fn mobility(&self, sqr: Coord) -> i32 {
        todo!();
    }

    #[evaluation_fn]
    pub fn mobility_area(&self) -> BitBoard {
        todo!();
    }

    #[evaluation_fn]
    pub fn mobility_bonus(&self, mg: bool, sqr: Coord) -> i32 {
        todo!();
    }

    #[inline]
    #[evaluation_fn]
    pub fn mobility_mg(&self, sqr: Coord) -> i32 {
        todo!();
    }

    #[inline]
    #[evaluation_fn]
    pub fn mobility_eg(&self, sqr: Coord) -> i32 {
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
        assert_eval!(friendly_mobility, 41, 48, eval);
    }

    #[test]
    #[ignore = "unimplemented evaluation function"]
    #[evaluation_test("1r3q1R/p1p1n2n/n2k1pR1/pQ3P1B/1bP2qpr/QP3n1P/P1P1P3/2B1N1RK w kq - 9 6")]
    fn test_mobility_area() {
        assert_eval!(+ - friendly_mobility_area, 49, 47, eval);
    }

    #[test]
    #[ignore = "unimplemented evaluation function"]
    #[evaluation_test("1r3q1R/p1p1n2n/n2k1pR1/pQ3P1B/1bP2qpr/QP3n1P/P1P1P3/2B1N1RK w kq - 9 6")]
    fn test_mobility_bonus() {
        assert_eval!(friendly_mobility_bonus, 193, 158, eval; true);
    }
}
