use proc_macro_utils::evaluation_fn;

use crate::board::coord::Coord;
use super::Evaluation;


impl<'a> Evaluation<'a> {
    #[evaluation_fn]
    pub fn outpost(&self, sqr: Coord) -> i32 {
        todo!();
    }

    #[evaluation_fn]
    pub fn outpost_square(&self, sqr: Coord) -> i32 {
        todo!();
    }

    #[evaluation_fn]
    pub fn reachable_outpost(&self, sqr: Coord) -> i32 {
        todo!();
    }

    #[evaluation_fn]
    pub fn minor_behind_pawn(&self, sqr: Coord) -> i32 {
        todo!();
    }

    #[evaluation_fn]
    pub fn bishop_pawns(&self, sqr: Coord) -> i32 {
        todo!();
    }

    #[evaluation_fn]
    pub fn rook_on_file(&self, sqr: Coord) -> i32 {
        todo!();
    }

    #[evaluation_fn]
    pub fn trapped_rook(&self, sqr: Coord) -> i32 {
        todo!();
    }

    #[evaluation_fn]
    pub fn weak_queen(&self, sqr: Coord) -> i32 {
        todo!();
    }

    #[evaluation_fn]
    pub fn king_protector(&self, sqr: Coord) -> i32 {
        todo!();
    }

    #[evaluation_fn]
    pub fn long_diagonal_bishop(&self, sqr: Coord) -> i32 {
        todo!();
    }

    #[evaluation_fn]
    pub fn outpost_total(&self, sqr: Coord) -> i32 {
        todo!();
    }

    #[evaluation_fn]
    pub fn rook_on_queen_file(&self, sqr: Coord) -> i32 {
        todo!();
    }

    #[evaluation_fn]
    pub fn bishop_xray_pawns(&self, sqr: Coord) -> i32 {
        todo!();
    }

    #[evaluation_fn]
    pub fn rook_on_king_ring(&self, sqr: Coord) -> i32 {
        todo!();
    }

    #[evaluation_fn]
    pub fn bishop_on_king_ring(&self, sqr: Coord) -> i32 {
        todo!();
    }

    #[evaluation_fn]
    pub fn queen_infiltration(&self, sqr: Coord) -> i32 {
        todo!();
    }

    #[evaluation_fn]
    pub fn pieces_mg(&self, sqr: Coord) -> i32 {
        todo!();
    }

    #[evaluation_fn]
    pub fn pieces_eg(&self, sqr: Coord) -> i32 {
        todo!();
    }
}


#[cfg(test)]
mod tests {
    use crate::eval::test_prelude::*;
    use super::*;

    #[test]
    #[ignore = "unimplemented evaluation function"]
    fn test_outpost() {
        let precomp = PrecomputedData::new();
        let magics = MagicBitBoards::default();
        let board = Board::load_position(Some(String::from("r2qk2r/6p1/1ppNp3/p1Pn1pNp/Pb1PnPbP/6P1/1P2P3/R1BQKB1R b KQkq - 1 2")), &mut Zobrist::new());
        let mut eval = Evaluation::new(&board, &precomp, &magics, Color::White);

        assert_eval!(friendly_outpost, 2, 3, eval);
    }

    #[test]
    #[ignore = "unimplemented evaluation function"]
    fn test_outpost_square() {
        let precomp = PrecomputedData::new();
        let magics = MagicBitBoards::default();
        let board = Board::load_position(Some(String::from("1r3q1R/2n4n/p2knpRp/pQp2PPB/1bP2q1r/5n1P/P1P2P2/2B1N1RK b kq - 0 7")), &mut Zobrist::new());
        let mut eval = Evaluation::new(&board, &precomp, &magics, Color::White);

        assert_eval!(friendly_outpost_square, 5, 0, eval);
    }

    #[test]
    #[ignore = "unimplemented evaluation function"]
    fn test_reachable_outpost() {
        let precomp = PrecomputedData::new();
        let magics = MagicBitBoards::default();
        let board = Board::load_position(Some(String::from("r2qk2r/6p1/1pp1p3/p1Pn1b1p/PbNPnP1P/5NP1/1P2P3/R1BQKB1R w KQkq - 2 3")), &mut Zobrist::new());
        let mut eval = Evaluation::new(&board, &precomp, &magics, Color::White);

        assert_eval!(friendly_reachable_outpost, 4, 2, eval);
    }

    #[test]
    #[ignore = "unimplemented evaluation function"]
    fn test_minor_behind_pawn() {
        let precomp = PrecomputedData::new();
        let magics = MagicBitBoards::default();
        let board = Board::load_position(Some(String::from("1r3q1R/2n5/p2knpRp/pQpn1PPB/1bP2q1r/5n1P/P1P2P2/2B1N1RK b kq - 0 7")), &mut Zobrist::new());
        let mut eval = Evaluation::new(&board, &precomp, &magics, Color::White);

        assert_eval!(friendly_reachable_outpost, 2, 1, eval);
    }

    #[test]
    #[ignore = "unimplemented evaluation function"]
    fn test_bishop_pawns() {
        let precomp = PrecomputedData::new();
        let magics = MagicBitBoards::default();
        let board = Board::load_position(Some(String::from("1r3q1R/3b4/p2knpRp/pQpn1PPB/1bP2q1r/5n1P/P1P2P2/2B1N1RK w kq - 1 8")), &mut Zobrist::new());
        let mut eval = Evaluation::new(&board, &precomp, &magics, Color::White);

        assert_eval!(friendly_bishop_pawns, 28, 11, eval);
    }

    #[test]
    #[ignore = "unimplemented evaluation function"]
    fn test_rook_on_file() {
        let precomp = PrecomputedData::new();
        let magics = MagicBitBoards::default();
        let board = Board::load_position(Some(String::from("1r3q1R/3b4/p2knpRp/pQpn1P1B/1bP2q1r/5n1P/P1P2P1P/2B1N1RK w kq - 1 8")), &mut Zobrist::new());
        let mut eval = Evaluation::new(&board, &precomp, &magics, Color::White);

        assert_eval!(friendly_rook_on_file, 4, 2, eval);
    }

    #[test]
    #[ignore = "unimplemented evaluation function"]
    fn test_trapped_rook() {
        let precomp = PrecomputedData::new();
        let magics = MagicBitBoards::default();
        let board = Board::load_position(Some(String::from("1r3q1R/1k1b4/p3npRp/pQpn1P1B/1bP2q1r/5n1P/P1P2P2/2B1N1KR w kq - 1 8")), &mut Zobrist::new());
        let mut eval = Evaluation::new(&board, &precomp, &magics, Color::White);

        assert_eval!(friendly_trapped_rook, 1, 0, eval);
    }

    #[test]
    #[ignore = "unimplemented evaluation function"]
    fn test_weak_queen() {
        let precomp = PrecomputedData::new();
        let magics = MagicBitBoards::default();
        let board = Board::load_position(Some(String::from("1r3q1R/1k1b4/p3npRp/pQpn1P1B/1bP4r/5n1P/P1P1qP2/2B1N1KR w kq - 1 8")), &mut Zobrist::new());
        let mut eval = Evaluation::new(&board, &precomp, &magics, Color::White);

        assert_eval!(friendly_weak_queen, 1, 1, eval);
    }

    #[test]
    #[ignore = "unimplemented evaluation function"]
    fn test_long_diagonal_bishop() {
        let precomp = PrecomputedData::new();
        let magics = MagicBitBoards::default();
        let board = Board::load_position(Some(String::from("1r3q1R/1k6/p1b1npRp/pQpn1P2/1bP4r/2B2n1P/P1P1qPB1/4N1KR w kq - 1 8")), &mut Zobrist::new());
        let mut eval = Evaluation::new(&board, &precomp, &magics, Color::White);

        assert_eval!(friendly_long_diagonal_bishop, 2, 1, eval);
    }

    #[test]
    #[ignore = "unimplemented evaluation function"]
    fn test_outpost_total() {
        let precomp = PrecomputedData::new();
        let magics = MagicBitBoards::default();
        let board = Board::load_position(Some(String::from("r2qk2r/6p1/1pp1p3/p1Pn1bNp/PbNPnP1P/6P1/1P2P3/R1BQKB1R w KQkq - 2 3")), &mut Zobrist::new());
        let mut eval = Evaluation::new(&board, &precomp, &magics, Color::White);

        assert_eval!(friendly_outpost_total, 5, 3, eval);
    }

    #[test]
    #[ignore = "unimplemented evaluation function"]
    fn test_rook_on_queen_file() {
        let precomp = PrecomputedData::new();
        let magics = MagicBitBoards::default();
        let board = Board::load_position(Some(String::from("1r3q1R/1k2R3/p1b1np1p/pQpnRP2/1bP4r/2B2n1P/P1P1qPB1/4N1K1 w kq - 1 8")), &mut Zobrist::new());
        let mut eval = Evaluation::new(&board, &precomp, &magics, Color::White);

        assert_eval!(friendly_rook_on_queen_file, 2, 1, eval);
    }

    #[test]
    #[ignore = "unimplemented evaluation function"]
    fn test_bishop_xray_pawns() {
        let precomp = PrecomputedData::new();
        let magics = MagicBitBoards::default();
        let board = Board::load_position(Some(String::from("1r1B1q1R/1k2R3/p3np1p/pQpnRP2/2P4r/1bB2n1P/P1P1qPB1/4N1K1 w kq - 1 8")), &mut Zobrist::new());
        let mut eval = Evaluation::new(&board, &precomp, &magics, Color::White);

        assert_eval!(friendly_bishop_xray_pawns, 4, 3, eval);
    }

    #[test]
    #[ignore = "unimplemented evaluation function"]
    fn test_rook_on_king_ring() {
        let precomp = PrecomputedData::new();
        let magics = MagicBitBoards::default();
        let board = Board::load_position(Some(String::from("k2q4/3r2p1/1pprp3/p1Pn1bNp/PbNPnP1P/6P1/1P2PR2/1RBQKB2 w KQkq - 2 3")), &mut Zobrist::new());
        let mut eval = Evaluation::new(&board, &precomp, &magics, Color::White);

        assert_eval!(friendly_rook_on_king_ring, 1, 2, eval);
    }

    #[test]
    #[ignore = "unimplemented evaluation function"]
    fn test_bishop_on_king_ring() {
        let precomp = PrecomputedData::new();
        let magics = MagicBitBoards::default();
        let board = Board::load_position(Some(String::from("1r1B1q1R/1k6/p2Rnp1p/pQpnbP2/2P2B1r/5n1P/P1P1qPBR/4N1K1 w kq - 1 8")), &mut Zobrist::new());
        let mut eval = Evaluation::new(&board, &precomp, &magics, Color::White);

        assert_eval!(friendly_bishop_on_king_ring, 2, 1, eval);
    }

    #[test]
    #[ignore = "unimplemented evaluation function"]
    fn test_queen_infiltration() {
        let precomp = PrecomputedData::new();
        let magics = MagicBitBoards::default();
        let board = Board::load_position(Some(String::from("1r1B1q1R/1k1np1Q1/p5Rp/p1pnbP2/2P2B1r/5n1P/P1P1qPBR/4N1K1 w kq - 1 8")), &mut Zobrist::new());
        let mut eval = Evaluation::new(&board, &precomp, &magics, Color::White);

        assert_eval!(friendly_queen_infiltration, 1, 1, eval);
    }

    #[test]
    #[ignore = "unimplemented evaluation function"]
    fn test_pieces_mg() {
        let precomp = PrecomputedData::new();
        let magics = MagicBitBoards::default();
        let board = Board::load_position(Some(String::from("nr1B1q2/1k2p1Q1/p5Rp/p1pnbP2/R1P2B1r/2P2n1P/P3qPBR/4N1K1 w kq - 1 8")), &mut Zobrist::new());
        let mut eval = Evaluation::new(&board, &precomp, &magics, Color::White);

        assert_eval!(friendly_pieces_mg, -121, -14, eval);
    }

    #[test]
    #[ignore = "unimplemented evaluation function"]
    fn test_pieces_eg() {
        let precomp = PrecomputedData::new();
        let magics = MagicBitBoards::default();
        let board = Board::load_position(Some(String::from("nr1B1q2/1k2p1Q1/p5Rp/p1pnbP2/R1P2B1r/2P2n1P/P3qPBR/4N1K1 w kq - 1 8")), &mut Zobrist::new());
        let mut eval = Evaluation::new(&board, &precomp, &magics, Color::White);

        assert_eval!(friendly_pieces_eg, -325, -105, eval);
    }
}
