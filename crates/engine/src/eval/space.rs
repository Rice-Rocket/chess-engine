use proc_macro_utils::evaluation_fn;

use crate::{board::{coord::Coord, piece::Piece}, color::{Black, Color, White}, prelude::BitBoard};
use super::Evaluation;


impl<'a> Evaluation<'a> {
    /// Returns `(space, space behind friendly pawn)`
    pub fn space_area<W: Color, B: Color>(&self) -> (BitBoard, BitBoard) {
        let pawns = self.board.piece_bitboards[W::piece(Piece::PAWN)];
        let center = BitBoard::from_ranks(W::ranks(1..=3)) & BitBoard::from_files(2..=5);

        let space = center & !pawns & !self.all_pawn_attacks::<B, W>().0;
        let behind_pawn = (pawns.shifted_2d(W::offset(0, -1))
            | pawns.shifted_2d(W::offset(0, -2))
            | pawns.shifted_2d(W::offset(0, -3)))
            & !self.all_attacks::<B, W>().0;

        (space, space & behind_pawn)
    }

    pub fn space<W: Color, B: Color>(&self) -> i32 {
        if self.non_pawn_material::<W, B>() + self.non_pawn_material::<B, W>() < 12222 { return 0 };
        let piece_count = self.piece_count::<W, B>();

        let friendly_pawns = self.board.piece_bitboards[W::piece(Piece::PAWN)];
        let enemy_pawns = self.board.piece_bitboards[B::piece(Piece::PAWN)];
        let mut blocked = friendly_pawns & (enemy_pawns.shifted_2d(W::offset(0, -1))
            | (enemy_pawns.shifted_2d(W::offset(-1, -2)) & enemy_pawns.shifted_2d(W::offset(1, -2))));
        blocked |= enemy_pawns & (friendly_pawns.shifted_2d(W::offset(0, 1))
            | (friendly_pawns.shifted_2d(W::offset(-1, 2)) & friendly_pawns.shifted_2d(W::offset(1, 2))));

        let weight = piece_count - 3 + (blocked.count() as i32).min(9);
        let area = self.space_area::<W, B>();
        (area.0.count() + area.1.count()) as i32 * weight * weight / 16
    }
}


#[cfg(test)]
mod tests {
    use crate::eval::test_prelude::*;
    use super::*;

    #[test]
    #[evaluation_test("nr1B3Q/1k2p2p/p2n2R1/p1p1bP1q/R1P1qB1r/1NP1P1nP/P5BR/6K1 w kq - 3 9")]
    fn test_space_area() {
        assert_eval!(* - [0, 1] space_area, (8, 2), (8, 0), eval);
    }

    #[test]
    #[evaluation_test("nr1B3Q/1k2p2p/p2n2R1/p1p1bP1q/R1P1qB1r/1NP3nP/P4PBR/6K1 w kq - 3 9")]
    fn test_space() {
        assert_eval!(- space, 110, 84, eval);
    }
}
