use crate::{board::{coord::Coord, piece::Piece}, color::Color, prelude::BitBoard};
use super::state::State;


// TODO: Save this data from move generation
pub fn pin_rays(state: &State) -> BitBoard {
    let mut start_dir_idx = 0;
    let mut end_dir_idx = 8;
    let mut pin_rays = BitBoard(0);
    let mut in_double_check = false;
    let mut in_check = false;
    let friendly_king_sqr = state.board.king_square[state.color];

    // Don't calculate unecessary directions
    if state.board.piece_bitboards[state.color.flip().piece(Piece::QUEEN)].count() == 0 {
        start_dir_idx = if state.board.piece_bitboards[state.color.flip().piece(Piece::ROOK)].count() > 0 { 0 } else { 4 };
        end_dir_idx = if state.board.piece_bitboards[state.color.flip().piece(Piece::BISHOP)].count() > 0 { 8 } else { 4 };
    }

    for dir in start_dir_idx..end_dir_idx {
        let is_diagonal = dir > 3;
        let slider = if is_diagonal { state.board.enemy_diagonal_sliders } else { state.board.enemy_orthogonal_sliders };
        if (state.precomp.dir_ray_mask[friendly_king_sqr][dir] & slider).0 == 0 { continue; }

        let n = state.precomp.num_sqrs_to_edge[friendly_king_sqr][dir];
        let dir_offset = state.precomp.direction_offsets[dir];
        let mut is_friendly_piece_along_ray = false;
        let mut ray_mask = BitBoard(0);

        for i in 0..n {
            let sqr = friendly_king_sqr + dir_offset * (i + 1);
            ray_mask |= sqr.to_bitboard();
            let piece = state.board.square[sqr];

            if piece != Piece::NULL {
                if piece.is_color(state.color.piece_color()) {
                    if !is_friendly_piece_along_ray {
                        is_friendly_piece_along_ray = true
                    } else { break };
                } else if (is_diagonal && piece.is_bishop_or_queen()) || (!is_diagonal && piece.is_rook_or_queen()) {
                    if is_friendly_piece_along_ray {
                        pin_rays |= ray_mask;
                    } else {
                        in_double_check = in_check;
                        in_check = true;
                    }
                    break;
                } else { break; }
            }
        }
        if in_double_check { break; }
    };

    pin_rays
}

pub fn pinned(state: &State, sqr: Coord) -> bool {
    ((pin_rays(state) >> sqr.index()) & 1).0 != 0
}

/// Calculates the number of friendly knights attacking `sqr`. If s2 specified, only counts attacks coming
/// from that square. 
pub fn knight_attack(state: &State, s2: Option<Coord>, sqr: Coord) -> i32 {
    let mut attacks = state.precomp.knight_moves[sqr] & state.board.piece_bitboards[state.color.piece(Piece::KNIGHT)];
    attacks &= !pin_rays(state);
    if let Some(s) = s2 {
        attacks &= s.to_bitboard();
    }
    attacks.count() as i32
}

/// Calculates the number of friendly bishops attacking `sqr`, including xray attacks through queens. 
/// If s2 specified, only counts attacks coming from that square.  
pub fn bishop_xray_attack(state: &State, s2: Option<Coord>, sqr: Coord) -> i32 {
    let blockers = state.board.all_pieces_bitboard & !(
        state.board.piece_bitboards[Piece::new(Piece::WHITE_QUEEN)] 
        | state.board.piece_bitboards[Piece::new(Piece::BLACK_QUEEN)]);
    let mut attacks = state.magics.get_bishop_attacks(sqr, blockers) 
        & state.board.piece_bitboards[state.color.piece(Piece::BISHOP)];
    if let Some(s) = s2 {
        attacks &= s.to_bitboard();
    }

    let mut count = 0;
    while attacks.0 != 0 {
        let start = Coord::from_idx(attacks.pop_lsb() as i8);
        if pinned(state, start) {
            if state.precomp.align_mask[start][state.movegen.friendly_king_sqr].contains_square(sqr.square()) {
                count += 1;
            }
        } else {
            count += 1;
        };
    }

    count
}

/// Calculates the number of friendly rooks attacking `sqr`, including xray attacks through queens. 
/// If s2 specified, only counts attacks coming from that square.  
pub fn rook_xray_attack(state: &State, s2: Option<Coord>, sqr: Coord) -> i32 {
    let blockers = state.board.all_pieces_bitboard & !(
        state.board.piece_bitboards[Piece::new(Piece::WHITE_QUEEN)] 
        | state.board.piece_bitboards[Piece::new(Piece::BLACK_QUEEN)]);
    let mut attacks = state.magics.get_rook_attacks(sqr, blockers) 
        & state.board.piece_bitboards[state.color.piece(Piece::ROOK)];
    if let Some(s) = s2 {
        attacks &= s.to_bitboard();
    }

    let mut count = 0;
    while attacks.0 != 0 {
        let start = Coord::from_idx(attacks.pop_lsb() as i8);
        if pinned(state, start) {
            if state.precomp.align_mask[start][state.movegen.friendly_king_sqr].contains_square(sqr.square()) {
                count += 1;
            }
        } else {
            count += 1;
        };
    }
    count
}

/// Calculates the number of friendly queens attacking `sqr`. If s2 specified, only counts attacks coming
/// from that square. 
pub fn queen_attack(state: &State, s2: Option<Coord>, sqr: Coord) -> i32 {
    let blockers = state.board.all_pieces_bitboard;
    let mut attacks = (state.magics.get_bishop_attacks(sqr, blockers) | state.magics.get_rook_attacks(sqr, blockers))
        & state.board.piece_bitboards[state.color.piece(Piece::QUEEN)];
    if let Some(s) = s2 {
        attacks &= s.to_bitboard();
    }

    let mut count = 0;
    while attacks.0 != 0 {
        let start = Coord::from_idx(attacks.pop_lsb() as i8);
        if pinned(state, start) {
            if state.precomp.align_mask[start][state.movegen.friendly_king_sqr].contains_square(sqr.square()) {
                count += 1;
            }
        } else {
            count += 1;
        };
    }
    count
}

/// Calculates the number of friendly pawns attacking `sqr`, excluding pins and en-passant. 
/// If s2 specified, only counts attacks coming from that square. 
pub fn pawn_attack(state: &State, s2: Option<Coord>, sqr: Coord) -> i32 {
    let map = if state.color == Color::White { state.precomp.black_pawn_attacks[sqr] } else { state.precomp.white_pawn_attacks[sqr] };
    let mut attacks = map & state.board.piece_bitboards[state.color.piece(Piece::PAWN)];
    if let Some(s) = s2 {
        attacks &= s.to_bitboard();
    }

    let mut count = 0;
    while attacks.0 != 0 {
        let start = Coord::from_idx(attacks.pop_lsb() as i8);
        if pinned(state, start) {
            if state.precomp.align_mask[start][state.movegen.friendly_king_sqr].contains_square(sqr.square()) {
                count += 1;
            }
        } else {
            count += 1;
        };
    }
    count
}

/// Calculates the number of friendly kings attacking `sqr`. If s2 specified, only counts attacks coming
/// from that square. 
pub fn king_attack(state: &State, s2: Option<Coord>, sqr: Coord) -> i32 {
    let mut attacks = state.precomp.king_moves[sqr] & state.board.piece_bitboards[state.color.piece(Piece::KING)];
    if let Some(s) = s2 {
        attacks &= s.to_bitboard();
    }
    attacks.count() as i32
}

/// Calculates the number of friendly attacks on `sqr` by all pieces.
pub fn attack(state: &State, sqr: Coord) -> i32 {
    pawn_attack(state, None, sqr)
    + king_attack(state, None, sqr)
    + knight_attack(state, None, sqr)
    + bishop_xray_attack(state, None, sqr)
    + rook_xray_attack(state, None, sqr)
    + queen_attack(state, None, sqr)
}

/// Calculates the number of friendly queens attacking `sqr` diagonally. If s2 specified, only counts
/// attacks coming from that square. 
pub fn queen_attack_diagonal(state: &State, s2: Option<Coord>, sqr: Coord) -> i32 {
    let blockers = state.board.all_pieces_bitboard;
    let mut attacks = state.magics.get_bishop_attacks(sqr, blockers)
        & state.board.piece_bitboards[state.color.piece(Piece::QUEEN)];
    if let Some(s) = s2 {
        attacks &= s.to_bitboard();
    }

    let mut count = 0;
    while attacks.0 != 0 {
        let start = Coord::from_idx(attacks.pop_lsb() as i8);
        if pinned(state, start) {
            if state.precomp.align_mask[start][state.movegen.friendly_king_sqr].contains_square(sqr.square()) {
                count += 1;
            }
        } else {
            count += 1;
        };
    }
    count
}


#[cfg(test)]
mod tests {
    use crate::{dbg_sqr_vals, eval::state::test_prelude::*};
    use super::*;

    #[test]
    fn test_knight_attack() {
        let precomp = PrecomputedData::new();
        let magics = MagicBitBoards::default();
        let board = Board::load_position(Some(String::from("nb2kb1R/pppppppp/6n1/4R2B/Qb2P3/4r3/PPPP1PPP/2BNK1Rq b KQkq e3 0 1")), &mut Zobrist::new());
        let mut movegen = MoveGenerator::default();
        movegen.generate_moves(&board, &precomp, &magics, false);
        let mut state = State::new(&board, &precomp, &movegen, &magics, Color::White);

        assert_eval!(knight_attack, 4, 8, state; None);
    }

    #[test]
    fn test_bishop_xray_attack() {
        let precomp = PrecomputedData::new();
        let magics = MagicBitBoards::default();
        let board = Board::load_position(Some(String::from("rnb1k1nr/pppp1ppp/6q1/4p2B/1b2PPQ1/8/PPPB1PPP/RN2K1NR b KQkq - 1 2")), &mut Zobrist::new());
        let mut movegen = MoveGenerator::default();
        movegen.generate_moves(&board, &precomp, &magics, false);
        let mut state = State::new(&board, &precomp, &movegen, &magics, Color::White);

        dbg_sqr_vals!(bishop_xray_attack, state; None);
        assert_eval!(bishop_xray_attack, 9, 10, state; None);
    }

    #[test]
    fn test_rook_xray_attack() {
        let precomp = PrecomputedData::new();
        let magics = MagicBitBoards::default();
        let board = Board::load_position(Some(String::from("2p1kbn1/pp1bpppr/r7/3p1q1B/Q7/P2R4/PPP1PPPP/1N2KBNR w KQkq d6 0 2")), &mut Zobrist::new());
        let mut movegen = MoveGenerator::default();
        movegen.generate_moves(&board, &precomp, &magics, false);
        let mut state = State::new(&board, &precomp, &movegen, &magics, Color::White);

        assert_eval!(rook_xray_attack, 13, 15, state; None);
    }

    #[test]
    fn test_queen_attack() {
        let precomp = PrecomputedData::new();
        let magics = MagicBitBoards::default();
        let board = Board::load_position(Some(String::from("nb2kb1R/pppppppp/6n1/4R2B/1bPP1q2/Q3r3/PPP2PPP/2BNK1R1 b KQkq e3 0 1")), &mut Zobrist::new());
        let mut movegen = MoveGenerator::default();
        movegen.generate_moves(&board, &precomp, &magics, false);
        let mut state = State::new(&board, &precomp, &movegen, &magics, Color::White);

        assert_eval!(queen_attack, 11, 15, state; None);
    }

    #[test]
    fn test_pawn_attack() {
        let precomp = PrecomputedData::new();
        let magics = MagicBitBoards::default();
        let board = Board::load_position(Some(String::from("nb2kb1R/p1p1n2p/1p3pn1/n3R2B/1bPP1qpP/QP2r1P1/P1P2P2/2BNK1R1 w KQkq - 0 2")), &mut Zobrist::new());
        let mut movegen = MoveGenerator::default();
        movegen.generate_moves(&board, &precomp, &magics, false);
        let mut state = State::new(&board, &precomp, &movegen, &magics, Color::White);

        assert_eval!(pawn_attack, 14, 10, state; None);
    }

    #[test]
    fn test_king_attack() {
        let precomp = PrecomputedData::new();
        let magics = MagicBitBoards::default();
        let board = Board::load_position(Some(String::from("nb3b1R/p1pkn2p/1p2Rpn1/n6B/1bPP1qpP/QP2r1P1/P1P2P2/2BN2RK b Qkq - 1 2")), &mut Zobrist::new());
        let mut movegen = MoveGenerator::default();
        movegen.generate_moves(&board, &precomp, &magics, false);
        let mut state = State::new(&board, &precomp, &movegen, &magics, Color::White);

        assert_eval!(king_attack, 3, 8, state; None);
    }

    #[test]
    fn test_attack() {
        let precomp = PrecomputedData::new();
        let magics = MagicBitBoards::default();
        let board = Board::load_position(Some(String::from("nb3b1R/p1pkn2p/1p2Rpn1/n6B/1bPP1qpP/QP2r1P1/P1P2P2/2BN2RK b Qkq - 1 2")), &mut Zobrist::new());
        let mut movegen = MoveGenerator::default();
        movegen.generate_moves(&board, &precomp, &magics, false);
        let mut state = State::new(&board, &precomp, &movegen, &magics, Color::White);

        assert_eval!(attack, 51, 73, state);
    }

    #[test]
    fn test_queen_attack_diagonal() {
        let precomp = PrecomputedData::new();
        let magics = MagicBitBoards::default();
        let board = Board::load_position(Some(String::from("nb3b1R/p1pkn2p/1p2Rpn1/n6B/1bPP1qpP/QP2r1P1/P1P2P2/2BN2RK b Qkq - 1 2")), &mut Zobrist::new());
        let mut movegen = MoveGenerator::default();
        movegen.generate_moves(&board, &precomp, &magics, false);
        let mut state = State::new(&board, &precomp, &movegen, &magics, Color::White);

        assert_eval!(queen_attack_diagonal, 3, 7, state; None);
    }
}
