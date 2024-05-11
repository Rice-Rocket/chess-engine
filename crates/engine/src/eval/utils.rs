use crate::board::coord::Coord;
use super::Evaluation;


impl<'a> Evaluation<'a> {
    pub fn bishop_count(&self, sqr: Coord) -> i32 {
        todo!();
    }

    pub fn queen_count(&self, sqr: Coord) -> i32 {
        todo!();
    }

    pub fn pawn_count(&self, sqr: Coord) -> i32 {
        todo!();
    }

    pub fn knight_count(&self, sqr: Coord) -> i32 {
        todo!();
    }

    pub fn rook_count(&self, sqr: Coord) -> i32 {
        todo!();
    }

    pub fn opposite_bishops(&self) -> i32 {
        todo!();
    }

    pub fn king_distance(&self, sqr: Coord) -> i32 {
        todo!();
    }

    pub fn king_ring(&self, full: bool, sqr: Coord) -> i32 {
        todo!();
    }

    pub fn piece_count(&self, sqr: Coord) -> i32 {
        todo!();
    }

    pub fn pawn_attacks_span(&self, sqr: Coord) -> i32 {
        todo!();
    }
}


#[cfg(test)]
mod tests {
    use crate::eval::test_prelude::*;
    use super::*;

    #[test]
    #[ignore = "unimplemented evaluation function"]
    fn test_bishop_count() {
        let precomp = PrecomputedData::new();
        let magics = MagicBitBoards::default();
        let board = Board::load_position(Some(String::from("nb3b1R/p1pkn2p/1p2Rpn1/n6B/1bPP1qpP/QP2r1P1/P1P2P2/2BN2RK b Qkq - 1 2")), &mut Zobrist::new());
        let mut eval = Evaluation::new(&board, &precomp, &magics, Color::White);

        assert_eval!(bishop_count, 2, 3, eval);
    }

    #[test]
    #[ignore = "unimplemented evaluation function"]
    fn test_queen_count() {
        let precomp = PrecomputedData::new();
        let magics = MagicBitBoards::default();
        let board = Board::load_position(Some(String::from("nb3b1R/p1pkn2p/1p2Rpn1/nQ5B/1bPP1qpP/QP2r1P1/P1P2P2/2BN2RK b Qkq - 1 2")), &mut Zobrist::new());
        let mut eval = Evaluation::new(&board, &precomp, &magics, Color::White);

        assert_eval!(queen_count, 2, 1, eval);
    }

    #[test]
    #[ignore = "unimplemented evaluation function"]
    fn test_pawn_count() {
        let precomp = PrecomputedData::new();
        let magics = MagicBitBoards::default();
        let board = Board::load_position(Some(String::from("nb3b1R/p1pkn2p/1p2Rpn1/nQ5B/1bPP1qpP/QP2r1P1/P1P2P2/2BN2RK b Qkq - 1 2")), &mut Zobrist::new());
        let mut eval = Evaluation::new(&board, &precomp, &magics, Color::White);

        assert_eval!(pawn_count, 8, 6, eval);
    }

    #[test]
    #[ignore = "unimplemented evaluation function"]
    fn test_knight_count() {
        let precomp = PrecomputedData::new();
        let magics = MagicBitBoards::default();
        let board = Board::load_position(Some(String::from("nb3b1R/p1pkn2p/1p2Rpn1/nQ5B/1bPP1qpP/QP2r1P1/P1P2P2/2BN2RK b Qkq - 1 2")), &mut Zobrist::new());
        let mut eval = Evaluation::new(&board, &precomp, &magics, Color::White);

        assert_eval!(knight_count, 1, 4, eval);
    }

    #[test]
    #[ignore = "unimplemented evaluation function"]
    fn test_rook_count() {
        let precomp = PrecomputedData::new();
        let magics = MagicBitBoards::default();
        let board = Board::load_position(Some(String::from("nb3b1R/p1pkn2p/1p2Rpn1/nQ5B/1bPP1qpP/QP2r1P1/P1P2P2/2BN2RK b Qkq - 1 2")), &mut Zobrist::new());
        let mut eval = Evaluation::new(&board, &precomp, &magics, Color::White);

        assert_eval!(rook_count, 3, 1, eval);
    }

    #[test]
    #[ignore = "unimplemented evaluation function"]
    fn test_opposite_bishops() {
        let precomp = PrecomputedData::new();
        let magics = MagicBitBoards::default();
        let board = Board::load_position(Some(String::from("n1b1r3/4p1Q1/1q2pP2/kpp4r/P1P4r/R1B1N2P/P4P1R/4RnK1 b kq - 2 11")), &mut Zobrist::new());
        let mut eval = Evaluation::new(&board, &precomp, &magics, Color::White);

        assert_eval!(- opposite_bishops, 1, 1, eval);
    }

    #[test]
    #[ignore = "unimplemented evaluation function"]
    fn test_king_distance() {
        let precomp = PrecomputedData::new();
        let magics = MagicBitBoards::default();
        let board = Board::load_position(Some(String::from("nb3b1R/p1pkn2p/1p2Rpn1/nQ5B/1bPP1qpP/QP2r1P1/P1P2P2/2BN2RK b Qkq - 1 2")), &mut Zobrist::new());
        let mut eval = Evaluation::new(&board, &precomp, &magics, Color::White);

        assert_eval!(king_distance, 308, 215, eval);
    }

    #[test]
    #[ignore = "unimplemented evaluation function"]
    fn test_king_ring() {
        let precomp = PrecomputedData::new();
        let magics = MagicBitBoards::default();
        let board = Board::load_position(Some(String::from("nb3b1R/p1pkn3/1p2Rpn1/nQ5B/1bPP1qpP/QP2r3/P1P2P1P/2BN2RK w Qkq - 2 3")), &mut Zobrist::new());
        let mut eval = Evaluation::new(&board, &precomp, &magics, Color::White);

        assert_eval!(king_ring, [6, 2], 0, 0, eval; false);
    }

    #[test]
    #[ignore = "unimplemented evaluation function"]
    fn test_piece_count() {
        let precomp = PrecomputedData::new();
        let magics = MagicBitBoards::default();
        let board = Board::load_position(Some(String::from("nb3b1R/p1pkn3/1p2Rpn1/nQ5B/1bPP1qpP/QP2r3/P1P2P1P/2BN2RK w Qkq - 2 3")), &mut Zobrist::new());
        let mut eval = Evaluation::new(&board, &precomp, &magics, Color::White);

        assert_eval!(piece_count, 17, 15, eval);
    }

    #[test]
    #[ignore = "unimplemented evaluation function"]
    fn test_pawn_attacks_span() {
        let precomp = PrecomputedData::new();
        let magics = MagicBitBoards::default();
        let board = Board::load_position(Some(String::from("nb3b1R/p1pkn3/n3Rpn1/pQ5B/1bPP1qpP/QP2r3/P1P2P1P/2BN2RK b Qkq - 3 3")), &mut Zobrist::new());
        let mut eval = Evaluation::new(&board, &precomp, &magics, Color::White);

        assert_eval!(pawn_attacks_span, 28, 32, eval);
    }
}
