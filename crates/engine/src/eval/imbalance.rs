use proc_macro_utils::flipped_eval;

use crate::board::coord::Coord;
use super::Evaluation;


impl<'a> Evaluation<'a> {
    #[flipped_eval]
    pub fn imbalance(&self, sqr: Coord) -> i32 {
        todo!();
    }

    #[flipped_eval]
    pub fn bishop_pair(&self, sqr: Coord) -> i32 {
        todo!();
    }

    #[flipped_eval]
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
    fn test_imbalance() {
        let precomp = PrecomputedData::new();
        let magics = MagicBitBoards::default();
        let board = Board::load_position(Some(String::from("nb3b1R/p1pkn3/n3Rpn1/pQ5B/1bPP1qpP/QP2r3/P1P2P1P/2BN2RK b Qkq - 3 3")), &mut Zobrist::new());
        let mut eval = Evaluation::new(&board, &precomp, &magics, Color::White);

        assert_eval!(imbalance, 9878, 14273, eval);
    }

    #[test]
    #[ignore = "unimplemented evaluation function"]
    fn test_bishop_pair() {
        let precomp = PrecomputedData::new();
        let magics = MagicBitBoards::default();
        let board = Board::load_position(Some(String::from("nr3q1R/p1pkn3/n3Rpn1/pQ5B/1bPP1qpP/QP2r3/P1P2P1P/2BN2RK b Qkq - 3 3")), &mut Zobrist::new());
        let mut eval = Evaluation::new(&board, &precomp, &magics, Color::White);

        assert_eval!(bishop_pair, 1438, 0, eval);
    }

    #[test]
    #[ignore = "unimplemented evaluation function"]
    fn test_imbalance_total() {
        let precomp = PrecomputedData::new();
        let magics = MagicBitBoards::default();
        let board = Board::load_position(Some(String::from("nr3q1R/p1pkn3/n3Rpn1/pQ5B/1bPP1qpP/QP2r3/P1P2P1P/2BN2RK b Qkq - 3 3")), &mut Zobrist::new());
        let mut eval = Evaluation::new(&board, &precomp, &magics, Color::White);

        assert_eval!(- imbalance_total, -181, 181, eval);
    }
}
