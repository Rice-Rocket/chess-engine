use proc_macro_utils::evaluation_fn;

use crate::board::coord::Coord;
use super::Evaluation;


impl<'a> Evaluation<'a> {
    #[evaluation_fn]
    pub fn space_area(&self, sqr: Coord) -> i32 {
        todo!();
    }

    #[evaluation_fn]
    pub fn space(&self, sqr: Coord) -> i32 {
        todo!();
    }
}


#[cfg(test)]
mod tests {
    use crate::eval::test_prelude::*;
    use super::*;

    #[test]
    #[ignore = "unimplemented evaluation function"]
    fn test_space_area() {
        let precomp = PrecomputedData::new();
        let magics = MagicBitBoards::default();
        let board = Board::load_position(Some(String::from("nr1B3Q/1k2p2p/p2n2R1/p1p1bP1q/R1P1qB1r/1NP3nP/P4PBR/6K1 w kq - 3 9")), &mut Zobrist::new());
        let mut eval = Evaluation::new(&board, &precomp, &magics, Color::White);

        assert_eval!(friendly_space_area, 9, 8, eval);
    }

    #[test]
    #[ignore = "unimplemented evaluation function"]
    fn test_space() {
        let precomp = PrecomputedData::new();
        let magics = MagicBitBoards::default();
        let board = Board::load_position(Some(String::from("nr1B3Q/1k2p2p/p2n2R1/p1p1bP1q/R1P1qB1r/1NP3nP/P4PBR/6K1 w kq - 3 9")), &mut Zobrist::new());
        let mut eval = Evaluation::new(&board, &precomp, &magics, Color::White);

        assert_eval!(friendly_space, 110, 84, eval);
    }
}
