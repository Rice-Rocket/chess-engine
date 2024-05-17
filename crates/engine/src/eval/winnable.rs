use proc_macro_utils::evaluation_fn;

use crate::{board::coord::Coord, color::{Color, White, Black}};
use super::Evaluation;


impl<'a> Evaluation<'a> {
    pub fn winnable<W: Color, B: Color>(&self, sqr: Coord) -> i32 {
        todo!();
    }

    pub fn winnable_total_mg<W: Color, B: Color>(&self, v: i32) -> i32 {
        todo!();
    }

    pub fn winnable_total_eg<W: Color, B: Color>(&self, v: i32) -> i32 {
        todo!();
    }
}


#[cfg(test)]
mod tests {
    use crate::eval::test_prelude::*;
    use super::*;

    #[test]
    #[ignore = "unimplemented evaluation function"]
    #[evaluation_test("n3r3/2p1p1Q1/p2n4/k1p1bP1r/P1PB3r/R2BN2P/Pq3P1R/1B2RnK1 b kq - 0 9")]
    fn test_winnable() {
        assert_eval!(winnable, 58, 58, eval);
    }
}
