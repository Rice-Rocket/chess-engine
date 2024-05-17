use proc_macro_utils::evaluation_fn;

use crate::{board::coord::Coord, color::{Color, White, Black}};
use super::Evaluation;


impl<'a> Evaluation<'a> {
    pub fn space_area<W: Color, B: Color>(&self, sqr: Coord) -> i32 {
        todo!();
    }

    pub fn space<W: Color, B: Color>(&self, sqr: Coord) -> i32 {
        todo!();
    }
}


#[cfg(test)]
mod tests {
    use crate::eval::test_prelude::*;
    use super::*;

    #[test]
    #[ignore = "unimplemented evaluation function"]
    #[evaluation_test("nr1B3Q/1k2p2p/p2n2R1/p1p1bP1q/R1P1qB1r/1NP3nP/P4PBR/6K1 w kq - 3 9")]
    fn test_space_area() {
        assert_eval!(space_area, 9, 8, eval);
    }

    #[test]
    #[ignore = "unimplemented evaluation function"]
    #[evaluation_test("nr1B3Q/1k2p2p/p2n2R1/p1p1bP1q/R1P1qB1r/1NP3nP/P4PBR/6K1 w kq - 3 9")]
    fn test_space() {
        assert_eval!(space, 110, 84, eval);
    }
}
