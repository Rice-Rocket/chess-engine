use proc_macro_utils::evaluation_fn;

use crate::{board::coord::Coord, color::{Color, White, Black}};
use super::Evaluation;


impl<'a> Evaluation<'a> {
    pub fn imbalance<W: Color, B: Color>(&self, sqr: Coord) -> i32 {
        todo!();
    }

    pub fn bishop_pair<W: Color, B: Color>(&self, sqr: Coord) -> i32 {
        todo!();
    }

    pub fn imbalance_total<W: Color, B: Color>(&self) -> i32 {
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
        assert_eval!(imbalance, 9878, 14273, eval);
    }

    #[test]
    #[ignore = "unimplemented evaluation function"]
    #[evaluation_test("nr3q1R/p1pkn3/n3Rpn1/pQ5B/1bPP1qpP/QP2r3/P1P2P1P/2BN2RK b Qkq - 3 3")]
    fn test_bishop_pair() {
        assert_eval!(bishop_pair, 1438, 0, eval);
    }

    #[test]
    #[ignore = "unimplemented evaluation function"]
    #[evaluation_test("nr3q1R/p1pkn3/n3Rpn1/pQ5B/1bPP1qpP/QP2r3/P1P2P1P/2BN2RK b Qkq - 3 3")]
    fn test_imbalance_total() {
        assert_eval!(- imbalance_total, -181, 181, eval);
    }
}
