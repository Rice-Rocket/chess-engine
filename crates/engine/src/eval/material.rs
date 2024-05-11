use proc_macro_utils::flipped_eval;

use crate::board::coord::Coord;
use super::Evaluation;


impl<'a> Evaluation<'a> {
    #[flipped_eval]
    pub fn non_pawn_material(&self, sqr: Coord) -> i32 {
        todo!();
    }

    #[flipped_eval]
    pub fn piece_value_bonus(&self, mg: bool, sqr: Coord) -> i32 {
        todo!();
    }

    #[flipped_eval]
    pub fn psqt_bonus(&self, mg: bool, sqr: Coord) -> i32 {
        todo!();
    }

    #[inline]
    #[flipped_eval]
    pub fn piece_value_mg(&self, sqr: Coord) -> i32 {
        todo!();
    }

    #[inline]
    #[flipped_eval]
    pub fn piece_value_eg(&self, sqr: Coord) -> i32 {
        todo!();
    }

    #[inline]
    #[flipped_eval]
    pub fn psqt_mg(&self, sqr: Coord) -> i32 {
        todo!();
    }

    #[inline]
    #[flipped_eval]
    pub fn psqt_eg(&self, sqr: Coord) -> i32 {
        todo!();
    }
}


#[cfg(test)]
mod tests {
    use crate::eval::test_prelude::*;
    use super::*;

    #[test]
    #[ignore = "unimplemented evaluation function"]
    fn test_non_pawn_material() {
        let precomp = PrecomputedData::new();
        let magics = MagicBitBoards::default();
        let board = Board::load_position(Some(String::from("1r3q1R/p1p1n2n/n2k1pR1/pQ3P1B/1bP2qpr/QP3n1P/P1P1P3/2B1N1RK w kq - 9 6")), &mut Zobrist::new());
        let mut eval = Evaluation::new(&board, &precomp, &magics, Color::White);

        assert_eval!(non_pawn_material, 11335, 11577, eval);
    }

    #[test]
    #[ignore = "unimplemented evaluation function"]
    fn test_piece_value_bonus() {
        let precomp = PrecomputedData::new();
        let magics = MagicBitBoards::default();
        let board = Board::load_position(Some(String::from("1r3q1R/p1p1n2n/n2k1pR1/pQ3P1B/1bP2qpr/QP3n1P/P1P1P3/2B1N1RK w kq - 9 6")), &mut Zobrist::new());
        let mut eval = Evaluation::new(&board, &precomp, &magics, Color::White);

        assert_eval!(piece_value_bonus, 12203, 12197, eval; true);
    }

    #[test]
    #[ignore = "unimplemented evaluation function"]
    fn test_psqt_bonus() {
        let precomp = PrecomputedData::new();
        let magics = MagicBitBoards::default();
        let board = Board::load_position(Some(String::from("1r3q1R/p1p1n2n/n2k1pR1/pQ3P1B/1bP2qpr/QP3n1P/P1P1P3/2B1N1RK w kq - 9 6")), &mut Zobrist::new());
        let mut eval = Evaluation::new(&board, &precomp, &magics, Color::White);

        assert_eval!(psqt_bonus, 146, 32, eval; true);
    }
}
