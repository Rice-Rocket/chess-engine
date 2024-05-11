use crate::board::coord::Coord;
use super::state::State;

pub fn safe_pawn(state: &State, sqr: Coord) -> i32 {
    todo!();
}

pub fn threat_safe_pawn(state: &State, sqr: Coord) -> i32 {
    todo!();
}

pub fn weak_enemies(state: &State, sqr: Coord) -> i32 {
    todo!();
}

pub fn minor_threat(state: &State, sqr: Coord) -> i32 {
    todo!();
}

pub fn rook_threat(state: &State, sqr: Coord) -> i32 {
    todo!();
}

pub fn hanging(state: &State, sqr: Coord) -> i32 {
    todo!();
}

pub fn king_threat(state: &State, sqr: Coord) -> i32 {
    todo!();
}

pub fn pawn_push_threat(state: &State, sqr: Coord) -> i32 {
    todo!();
}

pub fn slider_on_queen(state: &State, sqr: Coord) -> i32 {
    todo!();
}

pub fn knight_on_queen(state: &State, sqr: Coord) -> i32 {
    todo!();
}

pub fn restricted(state: &State, sqr: Coord) -> i32 {
    todo!();
}

pub fn weak_queen_protection(state: &State, sqr: Coord) -> i32 {
    todo!();
}

pub fn threats_mg(state: &State) -> i32 {
    todo!();
}

pub fn threats_eg(state: &State) -> i32 {
    todo!();
}


#[cfg(test)]
mod tests {
    use crate::eval::state::test_prelude::*;
    use super::*;

    #[test]
    #[ignore = "unimplemented evaluation function"]
    fn test_safe_pawn() {
        let precomp = PrecomputedData::new();
        let magics = MagicBitBoards::default();
        let board = Board::load_position(Some(String::from("nr1B3Q/1k2p2p/p2n2R1/p1p1bP1q/R1P1qB1r/1NP3nP/P4PBR/6K1 w kq - 3 9")), &mut Zobrist::new());
        let mut movegen = MoveGenerator::default();
        movegen.generate_moves(&board, &precomp, &magics, false);
        let mut state = State::new(&board, &precomp, &movegen, &magics, Color::White);

        assert_eval!(safe_pawn, 4, 2, state);
    }

    #[test]
    #[ignore = "unimplemented evaluation function"]
    fn test_threat_safe_pawn() {
        let precomp = PrecomputedData::new();
        let magics = MagicBitBoards::default();
        let board = Board::load_position(Some(String::from("nr1B3Q/1k2p2p/p2n2R1/1pp1bP1q/R1P1qB1r/1NP3nP/P4PBR/6K1 b kq - 0 9")), &mut Zobrist::new());
        let mut movegen = MoveGenerator::default();
        movegen.generate_moves(&board, &precomp, &magics, false);
        let mut state = State::new(&board, &precomp, &movegen, &magics, Color::White);

        assert_eval!(threat_safe_pawn, 1, 2, state);
    }

    #[test]
    #[ignore = "unimplemented evaluation function"]
    fn test_weak_enemies() {
        let precomp = PrecomputedData::new();
        let magics = MagicBitBoards::default();
        let board = Board::load_position(Some(String::from("nr1B3Q/1k2p2p/p2n2R1/1pp1bP1q/R1P1qB1r/1NP3nP/P4PBR/6K1 b kq - 0 9")), &mut Zobrist::new());
        let mut movegen = MoveGenerator::default();
        movegen.generate_moves(&board, &precomp, &magics, false);
        let mut state = State::new(&board, &precomp, &movegen, &magics, Color::White);

        assert_eval!(weak_enemies, 5, 7, state);
    }

    #[test]
    #[ignore = "unimplemented evaluation function"]
    fn test_minor_threat() {
        let precomp = PrecomputedData::new();
        let magics = MagicBitBoards::default();
        let board = Board::load_position(Some(String::from("nr1B3Q/1k2p2p/p2n2R1/1pp1bP1q/R1P1qB1r/1NP3nP/P4PBR/6K1 b kq - 0 9")), &mut Zobrist::new());
        let mut movegen = MoveGenerator::default();
        movegen.generate_moves(&board, &precomp, &magics, false);
        let mut state = State::new(&board, &precomp, &movegen, &magics, Color::White);

        assert_eval!(minor_threat, 18, 11, state);
    }

    #[test]
    #[ignore = "unimplemented evaluation function"]
    fn test_rook_threat() {
        let precomp = PrecomputedData::new();
        let magics = MagicBitBoards::default();
        let board = Board::load_position(Some(String::from("nr1B3Q/1k2p2p/p2n2R1/1pp1bP1q/R1P1qB1r/1NP3nP/P4PBR/6K1 b kq - 0 9")), &mut Zobrist::new());
        let mut movegen = MoveGenerator::default();
        movegen.generate_moves(&board, &precomp, &magics, false);
        let mut state = State::new(&board, &precomp, &movegen, &magics, Color::White);

        assert_eval!(rook_threat, 3, 6, state);
    }

    #[test]
    #[ignore = "unimplemented evaluation function"]
    fn test_hanging() {
        let precomp = PrecomputedData::new();
        let magics = MagicBitBoards::default();
        let board = Board::load_position(Some(String::from("nr1B3Q/1k2p2p/p2n2R1/1pp1bP1q/R1P1qB1r/1NP3nP/P4PBR/6K1 b kq - 0 9")), &mut Zobrist::new());
        let mut movegen = MoveGenerator::default();
        movegen.generate_moves(&board, &precomp, &magics, false);
        let mut state = State::new(&board, &precomp, &movegen, &magics, Color::White);

        assert_eval!(hanging, 4, 5, state);
    }

    #[test]
    #[ignore = "unimplemented evaluation function"]
    fn test_king_threat() {
        let precomp = PrecomputedData::new();
        let magics = MagicBitBoards::default();
        let board = Board::load_position(Some(String::from("nr1B3Q/4p2p/p2n2R1/kPp1bP1q/R3qB1r/1NP4P/P4PBR/5nK1 b kq - 0 9")), &mut Zobrist::new());
        let mut movegen = MoveGenerator::default();
        movegen.generate_moves(&board, &precomp, &magics, false);
        let mut state = State::new(&board, &precomp, &movegen, &magics, Color::White);

        assert_eval!(king_threat, 1, 2, state);
    }

    #[test]
    #[ignore = "unimplemented evaluation function"]
    fn test_pawn_push_threat() {
        let precomp = PrecomputedData::new();
        let magics = MagicBitBoards::default();
        let board = Board::load_position(Some(String::from("nr1B3Q/2p1p3/p2n2R1/kRp1bP1q/P3qB1r/1NP4P/P4PBR/5nK1 b kq - 0 9")), &mut Zobrist::new());
        let mut movegen = MoveGenerator::default();
        movegen.generate_moves(&board, &precomp, &magics, false);
        let mut state = State::new(&board, &precomp, &movegen, &magics, Color::White);

        assert_eval!(pawn_push_threat, 1, 2, state);
    }

    #[test]
    #[ignore = "unimplemented evaluation function"]
    fn test_slider_on_queen() {
        let precomp = PrecomputedData::new();
        let magics = MagicBitBoards::default();
        let board = Board::load_position(Some(String::from("n3r3/2p1p1Q1/p2n4/k1p1bP1r/P1PB3r/R2BN2P/Pq3P1R/1B2RnK1 b kq - 0 9")), &mut Zobrist::new());
        let mut movegen = MoveGenerator::default();
        movegen.generate_moves(&board, &precomp, &magics, false);
        let mut state = State::new(&board, &precomp, &movegen, &magics, Color::White);

        assert_eval!(slider_on_queen, 4, 3, state);
    }

    #[test]
    #[ignore = "unimplemented evaluation function"]
    fn test_knight_on_queen() {
        let precomp = PrecomputedData::new();
        let magics = MagicBitBoards::default();
        let board = Board::load_position(Some(String::from("n2Br3/2p1p1Q1/p2n4/kRp1bP1r/P1P4r/3BN2P/Pq3P1R/1B2RnK1 b kq - 0 9")), &mut Zobrist::new());
        let mut movegen = MoveGenerator::default();
        movegen.generate_moves(&board, &precomp, &magics, false);
        let mut state = State::new(&board, &precomp, &movegen, &magics, Color::White);

        assert_eval!(knight_on_queen, 1, 2, state);
    }

    #[test]
    #[ignore = "unimplemented evaluation function"]
    fn test_restricted() {
        let precomp = PrecomputedData::new();
        let magics = MagicBitBoards::default();
        let board = Board::load_position(Some(String::from("n3r3/2p1p1Q1/p2n4/k1p1bP1r/P1PB3r/R2BN2P/Pq3P1R/1B2RnK1 b kq - 0 9")), &mut Zobrist::new());
        let mut movegen = MoveGenerator::default();
        movegen.generate_moves(&board, &precomp, &magics, false);
        let mut state = State::new(&board, &precomp, &movegen, &magics, Color::White);

        assert_eval!(restricted, 20, 16, state);
    }

    #[test]
    #[ignore = "unimplemented evaluation function"]
    fn test_weak_queen_protection() {
        let precomp = PrecomputedData::new();
        let magics = MagicBitBoards::default();
        let board = Board::load_position(Some(String::from("n1n1r3/4p1Q1/1q2pP2/kpp1bB1r/P1PB3r/R3N2P/P4P1R/1B2RnK1 b kq - 2 11")), &mut Zobrist::new());
        let mut movegen = MoveGenerator::default();
        movegen.generate_moves(&board, &precomp, &magics, false);
        let mut state = State::new(&board, &precomp, &movegen, &magics, Color::White);

        assert_eval!(weak_queen_protection, 3, 1, state);
    }

    #[test]
    #[ignore = "unimplemented evaluation function"]
    fn test_threats_mg() {
        let precomp = PrecomputedData::new();
        let magics = MagicBitBoards::default();
        let board = Board::load_position(Some(String::from("n3r3/2p1p1Q1/p2n4/k1p1bP1r/P1PB3r/R2BN2P/Pq3P1R/1B2RnK1 b kq - 0 9")), &mut Zobrist::new());
        let mut movegen = MoveGenerator::default();
        movegen.generate_moves(&board, &precomp, &magics, false);
        let mut state = State::new(&board, &precomp, &movegen, &magics, Color::White);

        assert_eval!(- threats_mg, 951, 978, state);
    }

    #[test]
    #[ignore = "unimplemented evaluation function"]
    fn test_threats_eg() {
        let precomp = PrecomputedData::new();
        let magics = MagicBitBoards::default();
        let board = Board::load_position(Some(String::from("n3r3/2p1p1Q1/p2n4/k1p1bP1r/P1PB3r/R2BN2P/Pq3P1R/1B2RnK1 b kq - 0 9")), &mut Zobrist::new());
        let mut movegen = MoveGenerator::default();
        movegen.generate_moves(&board, &precomp, &magics, false);
        let mut state = State::new(&board, &precomp, &movegen, &magics, Color::White);

        assert_eval!(- threats_eg, 814, 982, state);
    }
}
