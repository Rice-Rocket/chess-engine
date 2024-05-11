use crate::board::coord::Coord;
use super::Evaluation;


impl<'a> Evaluation<'a> {
    pub fn pawnless_flank(&self) -> i32 {
        todo!();
    }

    pub fn strength_square(&self, sqr: Coord) -> i32 {
        todo!();
    }

    pub fn storm_square(&self, eg: bool, sqr: Coord) -> i32 {
        todo!();
    }

    pub fn shelter_strength(&self, sqr: Coord) -> i32 {
        todo!();
    }

    pub fn shelter_storm(&self, sqr: Coord) -> i32 {
        todo!();
    }

    pub fn king_pawn_distance(&self, sqr: Coord) -> i32 {
        todo!();
    }

    pub fn check(&self, ty: u8, sqr: Coord) -> i32 {
        todo!();
    }

    pub fn safe_check(&self, ty: u8, sqr: Coord) -> i32 {
        todo!();
    }

    pub fn king_attackers_count(&self, sqr: Coord) -> i32 {
        todo!();
    }

    pub fn king_attackers_weight(&self, sqr: Coord) -> i32 {
        todo!();
    }

    pub fn king_attacks(&self, sqr: Coord) -> i32 {
        todo!();
    }

    pub fn weak_bonus(&self, sqr: Coord) -> i32 {
        todo!();
    }

    pub fn weak_squares(&self, sqr: Coord) -> i32 {
        todo!();
    }

    pub fn unsafe_checks(&self, sqr: Coord) -> i32 {
        todo!();
    }

    pub fn knight_defender(&self, sqr: Coord) -> i32 {
        todo!();
    }

    pub fn endgame_shelter(&self, sqr: Coord) -> i32 {
        todo!();
    }

    pub fn blockers_for_king(&self, sqr: Coord) -> i32 {
        todo!();
    }

    pub fn flank_attack(&self, sqr: Coord) -> i32 {
        todo!();
    }

    pub fn flank_defense(&self, sqr: Coord) -> i32 {
        todo!();
    }

    pub fn king_danger(&self) -> i32 {
        todo!();
    }

    pub fn king_mg(&self) -> i32 {
        todo!();
    }

    pub fn king_eg(&self) -> i32 {
        todo!();
    }
}


#[cfg(test)]
mod tests {
    use crate::eval::test_prelude::*;
    use super::*;

    #[test]
    #[ignore = "unimplemented evaluation function"]
    fn test_pawnless_flank() {
        let precomp = PrecomputedData::new();
        let magics = MagicBitBoards::default();
        let board = Board::load_position(Some(String::from("3q4/4p1p1/bn1rpPp1/kr3bNp/2NPnP1P/3P2P1/3PPR2/1RBQKB2 w KQkq - 2 3")), &mut Zobrist::new());
        let mut eval = Evaluation::new(&board, &precomp, &magics, Color::White);

        assert_eval!(- pawnless_flank, 1, 0, eval);
    }

    #[test]
    #[ignore = "unimplemented evaluation function"]
    fn test_strength_square() {
        let precomp = PrecomputedData::new();
        let magics = MagicBitBoards::default();
        let board = Board::load_position(Some(String::from("nr3q1R/p1p1nR2/n2k1pn1/pQ3P1B/1bP2qp1/QP2r1PP/P1P1P3/2BN2RK b Qkq - 4 3")), &mut Zobrist::new());
        let mut eval = Evaluation::new(&board, &precomp, &magics, Color::White);

        assert_eval!(strength_square, -2768, -956, eval);
    }

    #[test]
    #[ignore = "unimplemented evaluation function"]
    fn test_storm_square() {
        let precomp = PrecomputedData::new();
        let magics = MagicBitBoards::default();
        let board = Board::load_position(Some(String::from("nr3q1R/p1p1nR2/n2k1pn1/pQ3P1B/1bP2qp1/QP2r1PP/P1P1P3/2BN2RK b Qkq - 4 3")), &mut Zobrist::new());
        let mut eval = Evaluation::new(&board, &precomp, &magics, Color::White);

        assert_eval!(storm_square, 672, 2579, eval; false);
    }

    #[test]
    #[ignore = "unimplemented evaluation function"]
    fn test_shelter_strength() {
        let precomp = PrecomputedData::new();
        let magics = MagicBitBoards::default();
        let board = Board::load_position(Some(String::from("nr3q1R/p1p1nR2/n2k1pn1/pQ3P1B/1bP2qp1/QP2r1PP/P1P1P3/2BN2RK b Qkq - 4 3")), &mut Zobrist::new());
        let mut eval = Evaluation::new(&board, &precomp, &magics, Color::White);

        assert_eval!(shelter_strength, -2, 76, eval);
    }

    #[test]
    #[ignore = "unimplemented evaluation function"]
    fn test_shelter_storm() {
        let precomp = PrecomputedData::new();
        let magics = MagicBitBoards::default();
        let board = Board::load_position(Some(String::from("nr3q1R/p1p1nR2/n2k1pn1/pQ3P1B/1bP2qp1/QP2r1PP/P1P1P3/2BN2RK b Qkq - 4 3")), &mut Zobrist::new());
        let mut eval = Evaluation::new(&board, &precomp, &magics, Color::White);

        assert_eval!(shelter_storm, -27, 17, eval);
    }

    #[test]
    #[ignore = "unimplemented evaluation function"]
    fn test_king_pawn_distance() {
        let precomp = PrecomputedData::new();
        let magics = MagicBitBoards::default();
        let board = Board::load_position(Some(String::from("nr3q1R/p1p1nR2/n2k1pn1/pQ3P1B/1bP2qp1/QP2r1PP/P1P1P3/2BN2RK b Qkq - 4 3")), &mut Zobrist::new());
        let mut eval = Evaluation::new(&board, &precomp, &magics, Color::White);

        assert_eval!(king_pawn_distance, 2, 1, eval);
    }

    #[test]
    #[ignore = "unimplemented evaluation function"]
    fn test_check() {
        let precomp = PrecomputedData::new();
        let magics = MagicBitBoards::default();
        let board = Board::load_position(Some(String::from("nr3q1R/p1p1nR2/n2k1pn1/pQ3P1B/1bP2qp1/QP2r1PP/P1P1P3/2BN2RK b Qkq - 4 3")), &mut Zobrist::new());
        let mut eval = Evaluation::new(&board, &precomp, &magics, Color::White);

        assert_eval!(check, 11, 2, eval; 0);
    }

    #[test]
    #[ignore = "unimplemented evaluation function"]
    fn test_safe_check() {
        let precomp = PrecomputedData::new();
        let magics = MagicBitBoards::default();
        let board = Board::load_position(Some(String::from("nr3q1R/p1p1nR2/n2k1pn1/pQ3P1B/1bP2qp1/QP2r1PP/P1P1P3/2BN2RK b Qkq - 4 3")), &mut Zobrist::new());
        let mut eval = Evaluation::new(&board, &precomp, &magics, Color::White);

        assert_eval!(safe_check, 2, 1, eval; 0);
    }

    #[test]
    #[ignore = "unimplemented evaluation function"]
    fn test_king_attackers_count() {
        let precomp = PrecomputedData::new();
        let magics = MagicBitBoards::default();
        let board = Board::load_position(Some(String::from("nr3q1R/p1p1nR2/n2k1pn1/pQ3P1B/1bP2qp1/QP2r1PP/P1P1P3/2BN2RK b Qkq - 4 3")), &mut Zobrist::new());
        let mut eval = Evaluation::new(&board, &precomp, &magics, Color::White);

        assert_eval!(king_attackers_count, 4, 4, eval);
    }

    #[test]
    #[ignore = "unimplemented evaluation function"]
    fn test_king_attackers_weight() {
        let precomp = PrecomputedData::new();
        let magics = MagicBitBoards::default();
        let board = Board::load_position(Some(String::from("1r3q1R/p1p1nR2/n2k1pn1/pQ3PnB/1bP2qp1/QP2r1PP/P1P1P3/2BN2RK b Qkq - 4 3")), &mut Zobrist::new());
        let mut eval = Evaluation::new(&board, &precomp, &magics, Color::White);

        assert_eval!(king_attackers_weight, 54, 135, eval);
    }

    #[test]
    #[ignore = "unimplemented evaluation function"]
    fn test_king_attacks() {
        let precomp = PrecomputedData::new();
        let magics = MagicBitBoards::default();
        let board = Board::load_position(Some(String::from("1r3q1R/p1p1nR2/n2k1pn1/pQ3P1B/1bP2qpn/QP2r1PP/P1P1P3/2BN2RK b Qkq - 4 3")), &mut Zobrist::new());
        let mut eval = Evaluation::new(&board, &precomp, &magics, Color::White);

        assert_eval!(king_attacks, 6, 1, eval);
    }

    #[test]
    #[ignore = "unimplemented evaluation function"]
    fn test_weak_bonus() {
        let precomp = PrecomputedData::new();
        let magics = MagicBitBoards::default();
        let board = Board::load_position(Some(String::from("1r3q1R/p1p1nR2/n2k1pn1/pQ3P1B/1bP2qpn/QP2r2P/P1P1P3/2BN2RK b Qkq - 4 3")), &mut Zobrist::new());
        let mut eval = Evaluation::new(&board, &precomp, &magics, Color::White);

        assert_eval!(weak_bonus, 1, 2, eval);
    }

    #[test]
    #[ignore = "unimplemented evaluation function"]
    fn test_weak_squares() {
        let precomp = PrecomputedData::new();
        let magics = MagicBitBoards::default();
        let board = Board::load_position(Some(String::from("1r3q1R/p1p1nR2/n2k1pn1/pQ3P1B/1bP2qpn/QP2r2P/P1P1P3/2BN2RK b Qkq - 4 3")), &mut Zobrist::new());
        let mut eval = Evaluation::new(&board, &precomp, &magics, Color::White);

        assert_eval!(weak_squares, 22, 20, eval);
    }

    #[test]
    #[ignore = "unimplemented evaluation function"]
    fn test_unsafe_checks() {
        let precomp = PrecomputedData::new();
        let magics = MagicBitBoards::default();
        let board = Board::load_position(Some(String::from("3q4/4p1p1/bn1rpPp1/kr1n1bNp/P1N2P1P/3P2R1/3PP1P1/1RBQKB2 b KQkq - 3 3")), &mut Zobrist::new());
        let mut eval = Evaluation::new(&board, &precomp, &magics, Color::White);

        assert_eval!(unsafe_checks, 2, 0, eval);
    }

    #[test]
    #[ignore = "unimplemented evaluation function"]
    fn test_knight_defender() {
        let precomp = PrecomputedData::new();
        let magics = MagicBitBoards::default();
        let board = Board::load_position(Some(String::from("1r3q1R/p1p1nR2/n2k1pn1/pQ3P1B/1bP2qpn/QP2r2P/P1P1P3/2B1N1RK w Qkq - 5 4")), &mut Zobrist::new());
        let mut eval = Evaluation::new(&board, &precomp, &magics, Color::White);

        assert_eval!(knight_defender, 1, 6, eval);
    }

    #[test]
    #[ignore = "unimplemented evaluation function"]
    fn test_endgame_shelter() {
        let precomp = PrecomputedData::new();
        let magics = MagicBitBoards::default();
        let board = Board::load_position(Some(String::from("1r3q1R/p1p1nR2/n2k1pn1/pQ3P1B/1bP2qpn/QP2r2P/P1P1P3/2B1N1RK b kq - 8 5")), &mut Zobrist::new());
        let mut eval = Evaluation::new(&board, &precomp, &magics, Color::White);

        assert_eval!(endgame_shelter, 5, 11, eval);
    }

    #[test]
    #[ignore = "unimplemented evaluation function"]
    fn test_blockers_for_king() {
        let precomp = PrecomputedData::new();
        let magics = MagicBitBoards::default();
        let board = Board::load_position(Some(String::from("1r3q1R/p1p1n2n/n2k1pR1/pQ3P1B/1bP2qpr/QP3n1P/P1P1P3/2B1N1RK w kq - 9 6")), &mut Zobrist::new());
        let mut eval = Evaluation::new(&board, &precomp, &magics, Color::White);

        assert_eval!(blockers_for_king, 2, 1, eval);
    }

    #[test]
    #[ignore = "unimplemented evaluation function"]
    fn test_flank_attack() {
        let precomp = PrecomputedData::new();
        let magics = MagicBitBoards::default();
        let board = Board::load_position(Some(String::from("1r3q1R/p1p1n2n/n2k1pR1/pQ3P1B/1bP2qpr/QP3n1P/P1P1P3/2B1N1RK w kq - 9 6")), &mut Zobrist::new());
        let mut eval = Evaluation::new(&board, &precomp, &magics, Color::White);

        assert_eval!(flank_attack, 17, 16, eval);
    }

    #[test]
    #[ignore = "unimplemented evaluation function"]
    fn test_flank_defense() {
        let precomp = PrecomputedData::new();
        let magics = MagicBitBoards::default();
        let board = Board::load_position(Some(String::from("1r3q1R/p1p1n2n/n2k1pR1/pQ3P1B/1bP2qpr/QP3n1P/P1P1P3/2B1N1RK w kq - 9 6")), &mut Zobrist::new());
        let mut eval = Evaluation::new(&board, &precomp, &magics, Color::White);

        assert_eval!(flank_defense, 19, 11, eval);
    }

    #[test]
    #[ignore = "unimplemented evaluation function"]
    fn test_king_danger() {
        let precomp = PrecomputedData::new();
        let magics = MagicBitBoards::default();
        let board = Board::load_position(Some(String::from("1r3q1R/p1p1n2n/n2k1pR1/pQ3P1B/1bP2qpr/QP3n1P/P1P1P3/2B1N1RK w kq - 9 6")), &mut Zobrist::new());
        let mut eval = Evaluation::new(&board, &precomp, &magics, Color::White);

        assert_eval!(- king_danger, 2640, 3448, eval);
    }

    #[test]
    #[ignore = "unimplemented evaluation function"]
    fn test_king_mg() {
        let precomp = PrecomputedData::new();
        let magics = MagicBitBoards::default();
        let board = Board::load_position(Some(String::from("1r3q1R/p1p1n2n/n2k1pR1/pQ3P1B/1bP2qpr/QP3n1P/P1P1P3/2B1N1RK w kq - 9 6")), &mut Zobrist::new());
        let mut eval = Evaluation::new(&board, &precomp, &magics, Color::White);

        assert_eval!(- king_mg, 1812, 3168, eval);
    }

    #[test]
    #[ignore = "unimplemented evaluation function"]
    fn test_king_eg() {
        let precomp = PrecomputedData::new();
        let magics = MagicBitBoards::default();
        let board = Board::load_position(Some(String::from("1r3q1R/p1p1n2n/n2k1pR1/pQ3P1B/1bP2qpr/QP3n1P/P1P1P3/2B1N1RK w kq - 9 6")), &mut Zobrist::new());
        let mut eval = Evaluation::new(&board, &precomp, &magics, Color::White);

        assert_eval!(- king_eg, 138, 210, eval);
    }
}
