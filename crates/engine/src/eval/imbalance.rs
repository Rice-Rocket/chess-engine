use proc_macro_utils::evaluation_fn;

use crate::board::coord::Coord;
use super::Evaluation;


impl<'a> Evaluation<'a> {
    #[evaluation_fn]
    pub fn imbalance(&self, sqr: Coord) -> i32 {
        todo!();
    }

    #[evaluation_fn]
    pub fn bishop_pair(&self, sqr: Coord) -> i32 {
        todo!();
    }

    #[evaluation_fn]
    pub fn imbalance_total(&self) -> i32 {
        todo!();
    }
}


#[cfg(test)]
mod tests {
    use crate::eval::test_prelude::*;
    use super::*;

    #[test]
    #[ignore = "unimplemented evaluation function"]
    #[evaluation_test("nb3b1R/p1pkn3/n3Rpn1/pQ5B/1bPP1qpP/QP2r3/P1P2P1P/2BN2RK b Qkq - 3 3")]
    fn test_imbalance() {
        assert_eval!(friendly_imbalance, 9878, 14273, eval);
    }

    #[test]
    #[ignore = "unimplemented evaluation function"]
    #[evaluation_test("nr3q1R/p1pkn3/n3Rpn1/pQ5B/1bPP1qpP/QP2r3/P1P2P1P/2BN2RK b Qkq - 3 3")]
    fn test_bishop_pair() {
        assert_eval!(friendly_bishop_pair, 1438, 0, eval);
    }

    #[test]
    #[ignore = "unimplemented evaluation function"]
    #[evaluation_test("nr3q1R/p1pkn3/n3Rpn1/pQ5B/1bPP1qpP/QP2r3/P1P2P1P/2BN2RK b Qkq - 3 3")]
    fn test_imbalance_total() {
        assert_eval!(- friendly_imbalance_total, -181, 181, eval);
    }
}
