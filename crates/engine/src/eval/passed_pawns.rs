use proc_macro_utils::evaluation_fn;

use crate::{board::coord::Coord, prelude::BitBoard};
use super::Evaluation;


impl<'a> Evaluation<'a> {
    #[evaluation_fn]
    pub fn candidate_passed(&self) -> BitBoard {
        todo!();
    }

    #[evaluation_fn]
    pub fn king_proximity(&self, sqr: Coord) -> i32 {
        todo!();
    }

    #[evaluation_fn]
    pub fn passed_block(&self, sqr: Coord) -> i32 {
        todo!();
    }

    #[evaluation_fn]
    pub fn passed_file(&self, sqr: Coord) -> i32 {
        todo!();
    }

    #[evaluation_fn]
    pub fn passed_rank(&self, sqr: Coord) -> i32 {
        todo!();
    }

    #[evaluation_fn]
    pub fn passed_leverable(&self) -> BitBoard {
        todo!();
    }

    #[evaluation_fn]
    pub fn passed_mg(&self, sqr: Coord) -> i32 {
        todo!();
    }

    #[evaluation_fn]
    pub fn passed_eg(&self, sqr: Coord) -> i32 {
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
        assert_eval!(+ - friendly_candidate_passed, 2, 1, eval);
    }

    #[test]
    #[ignore = "unimplemented evaluation function"]
    #[evaluation_test("1r3q1R/p3n2n/np1k1pR1/pQ3P1B/1b1P1qpr/QP3n1P/P1P1P3/2B1N1RK w kq - 9 6")]
    fn test_king_proximity() {
        assert_eval!(friendly_king_proximity, -18, -7, eval);
    }

    #[test]
    #[ignore = "unimplemented evaluation function"]
    #[evaluation_test("1r3q1R/p3n2n/np1k1pR1/pQ3P1B/1b1P1qpr/QP3n1P/P1P1P3/2B1N1RK w kq - 9 6")]
    fn test_passed_block() {
        assert_eval!(friendly_passed_block, 10, 35, eval);
    }

    #[test]
    #[ignore = "unimplemented evaluation function"]
    #[evaluation_test("1r3q1R/p3n2n/np1k1pR1/pQ3P1B/1b1P1qpr/QP3n1P/P1P1P3/2B1N1RK w kq - 9 6")]
    fn test_passed_file() {
        assert_eval!(friendly_passed_file, 3, 1, eval);
    }

    #[test]
    #[ignore = "unimplemented evaluation function"]
    #[evaluation_test("1r3q1R/p3n2n/np1k1pR1/pQ3P1B/1b1P1qpr/QP3n1P/P1P1P3/2B1N1RK w kq - 9 6")]
    fn test_passed_rank() {
        assert_eval!(friendly_passed_rank, 5, 4, eval);
    }

    #[test]
    #[ignore = "unimplemented evaluation function"]
    #[evaluation_test("1r3q1R/p3n2n/np1k1pR1/pQ3P1B/1b1P1qpr/QP3n1P/P1P1P3/2B1N1RK w kq - 9 6")]
    fn test_passed_leverable() {
        assert_eval!(+ - friendly_passed_leverable, 2, 1, eval);
    }

    #[test]
    #[ignore = "unimplemented evaluation function"]
    #[evaluation_test("1r3q1R/p3n2n/np1k1pR1/pQ3P1B/1b1P1qpr/QP3n1P/P1P1P3/2B1N1RK w kq - 9 6")]
    fn test_passed_mg() {
        assert_eval!(friendly_passed_mg, 9, 86, eval);
    }

    #[test]
    #[ignore = "unimplemented evaluation function"]
    #[evaluation_test("1r3q1R/p3n2n/np1k1pR1/pQ3P1B/1b1P1qpr/QP3n1P/P1P1P3/2B1N1RK w kq - 9 6")]
    fn test_passed_eg() {
        assert_eval!(friendly_passed_eg, 42, 92, eval);
    }
}
