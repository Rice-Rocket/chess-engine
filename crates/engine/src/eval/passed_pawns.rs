use proc_macro_utils::evaluation_fn;

use crate::{board::coord::Coord, color::{Color, White, Black}, prelude::BitBoard};
use super::Evaluation;


impl<'a> Evaluation<'a> {
    pub fn candidate_passed<W: Color, B: Color>(&self) -> BitBoard {
        todo!();
    }

    pub fn king_proximity<W: Color, B: Color>(&self, sqr: Coord) -> i32 {
        todo!();
    }

    pub fn passed_block<W: Color, B: Color>(&self, sqr: Coord) -> i32 {
        todo!();
    }

    pub fn passed_file<W: Color, B: Color>(&self, sqr: Coord) -> i32 {
        todo!();
    }

    pub fn passed_rank<W: Color, B: Color>(&self, sqr: Coord) -> i32 {
        todo!();
    }

    pub fn passed_leverable<W: Color, B: Color>(&self) -> BitBoard {
        todo!();
    }

    pub fn passed_mg<W: Color, B: Color>(&self, sqr: Coord) -> i32 {
        todo!();
    }

    pub fn passed_eg<W: Color, B: Color>(&self, sqr: Coord) -> i32 {
        todo!();
    }
}


#[cfg(test)]
mod tests {
    use crate::eval::test_prelude::*;
    use super::*;

    #[test]
    #[ignore = "unimplemented evaluation function"]
    #[evaluation_test("1r3q1R/p3n2n/np1k1pR1/pQ3P1B/1b1P1qpr/QP3n1P/P1P1P3/2B1N1RK w kq - 9 6")]
    fn test_candidate_passed() {
        assert_eval!(+ - candidate_passed, 2, 1, eval);
    }

    #[test]
    #[ignore = "unimplemented evaluation function"]
    #[evaluation_test("1r3q1R/p3n2n/np1k1pR1/pQ3P1B/1b1P1qpr/QP3n1P/P1P1P3/2B1N1RK w kq - 9 6")]
    fn test_king_proximity() {
        assert_eval!(king_proximity, -18, -7, eval);
    }

    #[test]
    #[ignore = "unimplemented evaluation function"]
    #[evaluation_test("1r3q1R/p3n2n/np1k1pR1/pQ3P1B/1b1P1qpr/QP3n1P/P1P1P3/2B1N1RK w kq - 9 6")]
    fn test_passed_block() {
        assert_eval!(passed_block, 10, 35, eval);
    }

    #[test]
    #[ignore = "unimplemented evaluation function"]
    #[evaluation_test("1r3q1R/p3n2n/np1k1pR1/pQ3P1B/1b1P1qpr/QP3n1P/P1P1P3/2B1N1RK w kq - 9 6")]
    fn test_passed_file() {
        assert_eval!(passed_file, 3, 1, eval);
    }

    #[test]
    #[ignore = "unimplemented evaluation function"]
    #[evaluation_test("1r3q1R/p3n2n/np1k1pR1/pQ3P1B/1b1P1qpr/QP3n1P/P1P1P3/2B1N1RK w kq - 9 6")]
    fn test_passed_rank() {
        assert_eval!(passed_rank, 5, 4, eval);
    }

    #[test]
    #[ignore = "unimplemented evaluation function"]
    #[evaluation_test("1r3q1R/p3n2n/np1k1pR1/pQ3P1B/1b1P1qpr/QP3n1P/P1P1P3/2B1N1RK w kq - 9 6")]
    fn test_passed_leverable() {
        assert_eval!(+ - passed_leverable, 2, 1, eval);
    }

    #[test]
    #[ignore = "unimplemented evaluation function"]
    #[evaluation_test("1r3q1R/p3n2n/np1k1pR1/pQ3P1B/1b1P1qpr/QP3n1P/P1P1P3/2B1N1RK w kq - 9 6")]
    fn test_passed_mg() {
        assert_eval!(passed_mg, 9, 86, eval);
    }

    #[test]
    #[ignore = "unimplemented evaluation function"]
    #[evaluation_test("1r3q1R/p3n2n/np1k1pR1/pQ3P1B/1b1P1qpr/QP3n1P/P1P1P3/2B1N1RK w kq - 9 6")]
    fn test_passed_eg() {
        assert_eval!(passed_eg, 42, 92, eval);
    }
}
