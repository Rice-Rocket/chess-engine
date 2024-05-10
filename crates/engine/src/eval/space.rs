use crate::board::coord::Coord;
use super::state::State;

pub fn space_area(state: &State, sqr: Coord) -> i32 {
    todo!();
}

pub fn space(state: &State, sqr: Coord) -> i32 {
    todo!();
}


#[cfg(test)]
mod tests {
    use crate::{sum_sqrs, assert_eval, color::Color, Board, Zobrist};
    use super::*;

    #[test]
    #[ignore = "unimplemented evaluation function"]
    fn test_space_area() {
        assert_eval!(space_area, 9, 8, "nr1B3Q/1k2p2p/p2n2R1/p1p1bP1q/R1P1qB1r/1NP3nP/P4PBR/6K1 w kq - 3 9");
    }

    #[test]
    #[ignore = "unimplemented evaluation function"]
    fn test_space() {
        assert_eval!(space, 110, 84, "nr1B3Q/1k2p2p/p2n2R1/p1p1bP1q/R1P1qB1r/1NP3nP/P4PBR/6K1 w kq - 3 9");
    }
}
