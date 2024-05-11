use proc_macro_utils::flipped_eval;

use crate::board::coord::Coord;
use super::Evaluation;


impl<'a> Evaluation<'a> {
    #[flipped_eval]
    pub fn safe_pawn(&self, sqr: Coord) -> i32 {
        todo!();
    }

    #[flipped_eval]
    pub fn threat_safe_pawn(&self, sqr: Coord) -> i32 {
        todo!();
    }

    #[flipped_eval]
    pub fn weak_enemies(&self, sqr: Coord) -> i32 {
        todo!();
    }

    #[flipped_eval]
    pub fn minor_threat(&self, sqr: Coord) -> i32 {
        todo!();
    }

    #[flipped_eval]
    pub fn rook_threat(&self, sqr: Coord) -> i32 {
        todo!();
    }

    #[flipped_eval]
    pub fn hanging(&self, sqr: Coord) -> i32 {
        todo!();
    }

    #[flipped_eval]
    pub fn king_threat(&self, sqr: Coord) -> i32 {
        todo!();
    }

    #[flipped_eval]
    pub fn pawn_push_threat(&self, sqr: Coord) -> i32 {
        todo!();
    }

    #[flipped_eval]
    pub fn slider_on_queen(&self, sqr: Coord) -> i32 {
        todo!();
    }

    #[flipped_eval]
    pub fn knight_on_queen(&self, sqr: Coord) -> i32 {
        todo!();
    }

    #[flipped_eval]
    pub fn restricted(&self, sqr: Coord) -> i32 {
        todo!();
    }

    #[flipped_eval]
    pub fn weak_queen_protection(&self, sqr: Coord) -> i32 {
        todo!();
    }

    #[flipped_eval]
    pub fn threats_mg(&self) -> i32 {
        todo!();
    }

    #[flipped_eval]
    pub fn threats_eg(&self) -> i32 {
        todo!();
    }
}


#[cfg(test)]
mod tests {
    use crate::eval::test_prelude::*;
    use super::*;

    #[test]
    #[ignore = "unimplemented evaluation function"]
    fn test_safe_pawn() {
        let precomp = PrecomputedData::new();
        let magics = MagicBitBoards::default();
        let board = Board::load_position(Some(String::from("nr1B3Q/1k2p2p/p2n2R1/p1p1bP1q/R1P1qB1r/1NP3nP/P4PBR/6K1 w kq - 3 9")), &mut Zobrist::new());
        let mut eval = Evaluation::new(&board, &precomp, &magics, Color::White);

        assert_eval!(safe_pawn, 4, 2, eval);
    }

    #[test]
    #[ignore = "unimplemented evaluation function"]
    fn test_threat_safe_pawn() {
        let precomp = PrecomputedData::new();
        let magics = MagicBitBoards::default();
        let board = Board::load_position(Some(String::from("nr1B3Q/1k2p2p/p2n2R1/1pp1bP1q/R1P1qB1r/1NP3nP/P4PBR/6K1 b kq - 0 9")), &mut Zobrist::new());
        let mut eval = Evaluation::new(&board, &precomp, &magics, Color::White);

        assert_eval!(threat_safe_pawn, 1, 2, eval);
    }

    #[test]
    #[ignore = "unimplemented evaluation function"]
    fn test_weak_enemies() {
        let precomp = PrecomputedData::new();
        let magics = MagicBitBoards::default();
        let board = Board::load_position(Some(String::from("nr1B3Q/1k2p2p/p2n2R1/1pp1bP1q/R1P1qB1r/1NP3nP/P4PBR/6K1 b kq - 0 9")), &mut Zobrist::new());
        let mut eval = Evaluation::new(&board, &precomp, &magics, Color::White);

        assert_eval!(weak_enemies, 5, 7, eval);
    }

    #[test]
    #[ignore = "unimplemented evaluation function"]
    fn test_minor_threat() {
        let precomp = PrecomputedData::new();
        let magics = MagicBitBoards::default();
        let board = Board::load_position(Some(String::from("nr1B3Q/1k2p2p/p2n2R1/1pp1bP1q/R1P1qB1r/1NP3nP/P4PBR/6K1 b kq - 0 9")), &mut Zobrist::new());
        let mut eval = Evaluation::new(&board, &precomp, &magics, Color::White);

        assert_eval!(minor_threat, 18, 11, eval);
    }

    #[test]
    #[ignore = "unimplemented evaluation function"]
    fn test_rook_threat() {
        let precomp = PrecomputedData::new();
        let magics = MagicBitBoards::default();
        let board = Board::load_position(Some(String::from("nr1B3Q/1k2p2p/p2n2R1/1pp1bP1q/R1P1qB1r/1NP3nP/P4PBR/6K1 b kq - 0 9")), &mut Zobrist::new());
        let mut eval = Evaluation::new(&board, &precomp, &magics, Color::White);

        assert_eval!(rook_threat, 3, 6, eval);
    }

    #[test]
    #[ignore = "unimplemented evaluation function"]
    fn test_hanging() {
        let precomp = PrecomputedData::new();
        let magics = MagicBitBoards::default();
        let board = Board::load_position(Some(String::from("nr1B3Q/1k2p2p/p2n2R1/1pp1bP1q/R1P1qB1r/1NP3nP/P4PBR/6K1 b kq - 0 9")), &mut Zobrist::new());
        let mut eval = Evaluation::new(&board, &precomp, &magics, Color::White);

        assert_eval!(hanging, 4, 5, eval);
    }

    #[test]
    #[ignore = "unimplemented evaluation function"]
    fn test_king_threat() {
        let precomp = PrecomputedData::new();
        let magics = MagicBitBoards::default();
        let board = Board::load_position(Some(String::from("nr1B3Q/4p2p/p2n2R1/kPp1bP1q/R3qB1r/1NP4P/P4PBR/5nK1 b kq - 0 9")), &mut Zobrist::new());
        let mut eval = Evaluation::new(&board, &precomp, &magics, Color::White);

        assert_eval!(king_threat, 1, 2, eval);
    }

    #[test]
    #[ignore = "unimplemented evaluation function"]
    fn test_pawn_push_threat() {
        let precomp = PrecomputedData::new();
        let magics = MagicBitBoards::default();
        let board = Board::load_position(Some(String::from("nr1B3Q/2p1p3/p2n2R1/kRp1bP1q/P3qB1r/1NP4P/P4PBR/5nK1 b kq - 0 9")), &mut Zobrist::new());
        let mut eval = Evaluation::new(&board, &precomp, &magics, Color::White);

        assert_eval!(pawn_push_threat, 1, 2, eval);
    }

    #[test]
    #[ignore = "unimplemented evaluation function"]
    fn test_slider_on_queen() {
        let precomp = PrecomputedData::new();
        let magics = MagicBitBoards::default();
        let board = Board::load_position(Some(String::from("n3r3/2p1p1Q1/p2n4/k1p1bP1r/P1PB3r/R2BN2P/Pq3P1R/1B2RnK1 b kq - 0 9")), &mut Zobrist::new());
        let mut eval = Evaluation::new(&board, &precomp, &magics, Color::White);

        assert_eval!(slider_on_queen, 4, 3, eval);
    }

    #[test]
    #[ignore = "unimplemented evaluation function"]
    fn test_knight_on_queen() {
        let precomp = PrecomputedData::new();
        let magics = MagicBitBoards::default();
        let board = Board::load_position(Some(String::from("n2Br3/2p1p1Q1/p2n4/kRp1bP1r/P1P4r/3BN2P/Pq3P1R/1B2RnK1 b kq - 0 9")), &mut Zobrist::new());
        let mut eval = Evaluation::new(&board, &precomp, &magics, Color::White);

        assert_eval!(knight_on_queen, 1, 2, eval);
    }

    #[test]
    #[ignore = "unimplemented evaluation function"]
    fn test_restricted() {
        let precomp = PrecomputedData::new();
        let magics = MagicBitBoards::default();
        let board = Board::load_position(Some(String::from("n3r3/2p1p1Q1/p2n4/k1p1bP1r/P1PB3r/R2BN2P/Pq3P1R/1B2RnK1 b kq - 0 9")), &mut Zobrist::new());
        let mut eval = Evaluation::new(&board, &precomp, &magics, Color::White);

        assert_eval!(restricted, 20, 16, eval);
    }

    #[test]
    #[ignore = "unimplemented evaluation function"]
    fn test_weak_queen_protection() {
        let precomp = PrecomputedData::new();
        let magics = MagicBitBoards::default();
        let board = Board::load_position(Some(String::from("n1n1r3/4p1Q1/1q2pP2/kpp1bB1r/P1PB3r/R3N2P/P4P1R/1B2RnK1 b kq - 2 11")), &mut Zobrist::new());
        let mut eval = Evaluation::new(&board, &precomp, &magics, Color::White);

        assert_eval!(weak_queen_protection, 3, 1, eval);
    }

    #[test]
    #[ignore = "unimplemented evaluation function"]
    fn test_threats_mg() {
        let precomp = PrecomputedData::new();
        let magics = MagicBitBoards::default();
        let board = Board::load_position(Some(String::from("n3r3/2p1p1Q1/p2n4/k1p1bP1r/P1PB3r/R2BN2P/Pq3P1R/1B2RnK1 b kq - 0 9")), &mut Zobrist::new());
        let mut eval = Evaluation::new(&board, &precomp, &magics, Color::White);

        assert_eval!(- threats_mg, 951, 978, eval);
    }

    #[test]
    #[ignore = "unimplemented evaluation function"]
    fn test_threats_eg() {
        let precomp = PrecomputedData::new();
        let magics = MagicBitBoards::default();
        let board = Board::load_position(Some(String::from("n3r3/2p1p1Q1/p2n4/k1p1bP1r/P1PB3r/R2BN2P/Pq3P1R/1B2RnK1 b kq - 0 9")), &mut Zobrist::new());
        let mut eval = Evaluation::new(&board, &precomp, &magics, Color::White);

        assert_eval!(- threats_eg, 814, 982, eval);
    }
}
