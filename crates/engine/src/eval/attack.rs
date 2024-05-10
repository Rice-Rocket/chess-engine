use crate::board::coord::Coord;
use super::state::State;

pub fn pinned_direction(state: &State, sqr: Coord) -> i32 {
    todo!();
}

/// Calculates the number of knights attacking `sqr`. If s2 specified, only counts attacks coming
/// from that square. 
pub fn knight_attack(state: &State, s2: Option<Coord>, sqr: Coord) -> i32 {
    todo!();
}

/// Calculates the number of bishops attacking `sqr`, including xray attacks through queens. 
/// If s2 specified, only counts attacks coming from that square.  
pub fn bishop_xray_attack(state: &State, s2: Option<Coord>, sqr: Coord) -> i32 {
    todo!();
}

/// Calculates the number of rooks attacking `sqr`, including xray attacks through queens. 
/// If s2 specified, only counts attacks coming from that square.  
pub fn rook_xray_attack(state: &State, s2: Option<Coord>, sqr: Coord) -> i32 {
    todo!();
}

/// Calculates the number of queens attacking `sqr`. If s2 specified, only counts attacks coming
/// from that square. 
pub fn queen_attack(state: &State, s2: Option<Coord>, sqr: Coord) -> i32 {
    todo!();
}

/// Calculates the number of pawns attacking `sqr`, excluding pins and en-passant. 
/// If s2 specified, only counts attacks coming from that square. 
pub fn pawn_attack(state: &State, s2: Option<Coord>, sqr: Coord) -> i32 {
    todo!();
}

/// Calculates the number of kings attacking `sqr`. If s2 specified, only counts attacks coming
/// from that square. 
pub fn king_attack(state: &State, s2: Option<Coord>, sqr: Coord) -> i32 {
    todo!();
}

/// Calculates the number of attacks on `sqr` by all pieces.
pub fn attack(state: &State, sqr: Coord) -> i32 {
    todo!();
}

/// Calculates whether or not `sqr` is pinned.
pub fn pinned(state: &State, sqr: Coord) -> i32 {
    todo!();
}

/// Calculates the number of queens attacking `sqr` diagonally. If s2 specified, only counts
/// attacks coming from that square. 
pub fn queen_attack_diagonal(state: &State, s2: Option<Coord>, sqr: Coord) -> i32 {
    todo!();
}


#[cfg(test)]
mod tests {
    use crate::eval::state::test_prelude::*;
    use super::*;

    #[test]
    #[ignore = "unimplemented evaluation function"]
    fn test_pinned_direction() {
        let precomp = PrecomputedData::new();
        let magics = MagicBitBoards::default();
        let board = Board::load_position(Some(String::from("rn2kbnR/pppppppp/8/4R2B/Qb2P3/4r3/PPPP1PPP/1NB1K1Nq b KQkq e3 0 1")), &mut Zobrist::new());
        let mut movegen = MoveGenerator::default();
        movegen.generate_moves(&board, &precomp, &magics, false);
        let mut state = State::new(&board, &precomp, &movegen, Color::White);

        assert_eval!(pinned_direction, 5, 9, state);
    }

    #[test]
    #[ignore = "unimplemented evaluation function"]
    fn test_knight_attack() {
        let precomp = PrecomputedData::new();
        let magics = MagicBitBoards::default();
        let board = Board::load_position(Some(String::from("nb2kb1R/pppppppp/6n1/4R2B/Qb2P3/4r3/PPPP1PPP/2BNK1Rq b KQkq e3 0 1")), &mut Zobrist::new());
        let mut movegen = MoveGenerator::default();
        movegen.generate_moves(&board, &precomp, &magics, false);
        let mut state = State::new(&board, &precomp, &movegen, Color::White);

        assert_eval!(knight_attack, 4, 8, state; None);
    }

    #[test]
    #[ignore = "unimplemented evaluation function"]
    fn test_bishop_xray_attack() {
        let precomp = PrecomputedData::new();
        let magics = MagicBitBoards::default();
        let board = Board::load_position(Some(String::from("nb2kb1R/pppppppp/6n1/4R2B/Qb2P3/4r3/PPPP1PPP/2BNK1Rq b KQkq e3 0 1")), &mut Zobrist::new());
        let mut movegen = MoveGenerator::default();
        movegen.generate_moves(&board, &precomp, &magics, false);
        let mut state = State::new(&board, &precomp, &movegen, Color::White);

        assert_eval!(bishop_xray_attack, 7, 9, state; None);
    }

    #[test]
    #[ignore = "unimplemented evaluation function"]
    fn test_rook_xray_attack() {
        let precomp = PrecomputedData::new();
        let magics = MagicBitBoards::default();
        let board = Board::load_position(Some(String::from("nb2kb1R/pppppppp/6n1/4R2B/Qb1P4/4r3/PPPP1PPP/2BNK1Rq b KQkq e3 0 1")), &mut Zobrist::new());
        let mut movegen = MoveGenerator::default();
        movegen.generate_moves(&board, &precomp, &magics, false);
        let mut state = State::new(&board, &precomp, &movegen, Color::White);

        assert_eval!(rook_xray_attack, 17, 11, state; None);
    }

    #[test]
    #[ignore = "unimplemented evaluation function"]
    fn test_queen_attack() {
        let precomp = PrecomputedData::new();
        let magics = MagicBitBoards::default();
        let board = Board::load_position(Some(String::from("nb2kb1R/pppppppp/6n1/4R2B/1bPP1q2/Q3r3/PPP2PPP/2BNK1R1 b KQkq e3 0 1")), &mut Zobrist::new());
        let mut movegen = MoveGenerator::default();
        movegen.generate_moves(&board, &precomp, &magics, false);
        let mut state = State::new(&board, &precomp, &movegen, Color::White);

        assert_eval!(queen_attack, 11, 15, state; None);
    }

    #[test]
    #[ignore = "unimplemented evaluation function"]
    fn test_pawn_attack() {
        let precomp = PrecomputedData::new();
        let magics = MagicBitBoards::default();
        let board = Board::load_position(Some(String::from("nb2kb1R/p1p1n2p/1p3pn1/n3R2B/1bPP1qpP/QP2r1P1/P1P2P2/2BNK1R1 w KQkq - 0 2")), &mut Zobrist::new());
        let mut movegen = MoveGenerator::default();
        movegen.generate_moves(&board, &precomp, &magics, false);
        let mut state = State::new(&board, &precomp, &movegen, Color::White);

        assert_eval!(pawn_attack, 14, 10, state; None);
    }

    #[test]
    #[ignore = "unimplemented evaluation function"]
    fn test_king_attack() {
        let precomp = PrecomputedData::new();
        let magics = MagicBitBoards::default();
        let board = Board::load_position(Some(String::from("nb3b1R/p1pkn2p/1p2Rpn1/n6B/1bPP1qpP/QP2r1P1/P1P2P2/2BN2RK b Qkq - 1 2")), &mut Zobrist::new());
        let mut movegen = MoveGenerator::default();
        movegen.generate_moves(&board, &precomp, &magics, false);
        let mut state = State::new(&board, &precomp, &movegen, Color::White);

        assert_eval!(king_attack, 3, 8, state; None);
    }

    #[test]
    #[ignore = "unimplemented evaluation function"]
    fn test_attack() {
        let precomp = PrecomputedData::new();
        let magics = MagicBitBoards::default();
        let board = Board::load_position(Some(String::from("nb3b1R/p1pkn2p/1p2Rpn1/n6B/1bPP1qpP/QP2r1P1/P1P2P2/2BN2RK b Qkq - 1 2")), &mut Zobrist::new());
        let mut movegen = MoveGenerator::default();
        movegen.generate_moves(&board, &precomp, &magics, false);
        let mut state = State::new(&board, &precomp, &movegen, Color::White);

        assert_eval!(attack, 51, 73, state);
    }

    #[test]
    #[ignore = "unimplemented evaluation function"]
    fn test_queen_attack_diagonal() {
        let precomp = PrecomputedData::new();
        let magics = MagicBitBoards::default();
        let board = Board::load_position(Some(String::from("nb3b1R/p1pkn2p/1p2Rpn1/n6B/1bPP1qpP/QP2r1P1/P1P2P2/2BN2RK b Qkq - 1 2")), &mut Zobrist::new());
        let mut movegen = MoveGenerator::default();
        movegen.generate_moves(&board, &precomp, &magics, false);
        let mut state = State::new(&board, &precomp, &movegen, Color::White);

        assert_eval!(queen_attack_diagonal, 3, 7, state; None);
    }

    #[test]
    #[ignore = "unimplemented evaluation function"]
    fn test_pinned() {
        let precomp = PrecomputedData::new();
        let magics = MagicBitBoards::default();
        let board = Board::load_position(Some(String::from("rnb1kbn1/pppppppp/8/qB6/8/2N1R3/PPPPPPPP/R1BQK1Nr w KQkq - 0 1")), &mut Zobrist::new());
        let mut movegen = MoveGenerator::default();
        movegen.generate_moves(&board, &precomp, &magics, false);
        let mut state = State::new(&board, &precomp, &movegen, Color::White);

        assert_eval!(pinned, 1, 2, state);
    }
}
