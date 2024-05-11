use proc_macro_utils::flipped_eval;

use crate::board::coord::Coord;
use super::Evaluation;


impl<'a> Evaluation<'a> {
    #[flipped_eval]
    pub fn mobility(&self, sqr: Coord) -> i32 {
        todo!();
    }

    #[flipped_eval]
    pub fn mobility_area(&self, sqr: Coord) -> i32 {
        todo!();
    }

    #[flipped_eval]
    pub fn mobility_bonus(&self, mg: bool, sqr: Coord) -> i32 {
        todo!();
    }

    #[inline]
    #[flipped_eval]
    pub fn mobility_mg(&self, sqr: Coord) -> i32 {
        todo!();
    }

    #[inline]
    #[flipped_eval]
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
    fn test_mobility() {
        let precomp = PrecomputedData::new();
        let magics = MagicBitBoards::default();
        let board = Board::load_position(Some(String::from("1r3q1R/p1p1n2n/n2k1pR1/pQ3P1B/1bP2qpr/QP3n1P/P1P1P3/2B1N1RK w kq - 9 6")), &mut Zobrist::new());
        let mut eval = Evaluation::new(&board, &precomp, &magics, Color::White);

        assert_eval!(mobility, 41, 48, eval);
    }

    #[test]
    #[ignore = "unimplemented evaluation function"]
    fn test_mobility_area() {
        let precomp = PrecomputedData::new();
        let magics = MagicBitBoards::default();
        let board = Board::load_position(Some(String::from("1r3q1R/p1p1n2n/n2k1pR1/pQ3P1B/1bP2qpr/QP3n1P/P1P1P3/2B1N1RK w kq - 9 6")), &mut Zobrist::new());
        let mut eval = Evaluation::new(&board, &precomp, &magics, Color::White);

        assert_eval!(mobility_area, 49, 47, eval);
    }

    #[test]
    #[ignore = "unimplemented evaluation function"]
    fn test_mobility_bonus() {
        let precomp = PrecomputedData::new();
        let magics = MagicBitBoards::default();
        let board = Board::load_position(Some(String::from("1r3q1R/p1p1n2n/n2k1pR1/pQ3P1B/1bP2qpr/QP3n1P/P1P1P3/2B1N1RK w kq - 9 6")), &mut Zobrist::new());
        let mut eval = Evaluation::new(&board, &precomp, &magics, Color::White);

        assert_eval!(mobility_bonus, 193, 158, eval; true);
    }
}
