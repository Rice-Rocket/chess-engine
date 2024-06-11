use crate::{board::{coord::Coord, moves::Move, piece::Piece, Board}, color::{Black, Color, White}, eval::Evaluation, move_gen::magics::Magics, precomp::Precomputed, prelude::BitBoard};

use super::ordering::MoveOrdering;

fn least_valuable_piece(board: &Board, attack_def: BitBoard, is_white: bool) -> (Piece, BitBoard) {
    let pcolor = if is_white { Piece::WHITE } else { Piece::BLACK };
    for ptype in Piece::PAWN..=Piece::KING {
        let piece = Piece::new(ptype | pcolor);
        let subset = attack_def & board.piece_bitboards[piece];
        if subset.0 != 0 {
            return (piece, BitBoard::from_bit(subset.lsb()));
        }
    }
    (Piece::new(Piece::NONE), BitBoard(0))
}

#[must_use]
fn attacking(board: &Board, sqr: Coord) -> BitBoard {
    let pawns = board.piece_bitboards[Piece::new(Piece::WHITE_PAWN)] | board.piece_bitboards[Piece::new(Piece::BLACK_PAWN)];
    let knights = board.piece_bitboards[Piece::new(Piece::WHITE_KNIGHT)] | board.piece_bitboards[Piece::new(Piece::BLACK_KNIGHT)];
    let bishops = board.piece_bitboards[Piece::new(Piece::WHITE_BISHOP)] | board.piece_bitboards[Piece::new(Piece::BLACK_BISHOP)];
    let rooks = board.piece_bitboards[Piece::new(Piece::WHITE_ROOK)] | board.piece_bitboards[Piece::new(Piece::BLACK_ROOK)];
    let queens = board.piece_bitboards[Piece::new(Piece::WHITE_QUEEN)] | board.piece_bitboards[Piece::new(Piece::BLACK_QUEEN)];
    let kings = board.piece_bitboards[Piece::new(Piece::WHITE_KING)] | board.piece_bitboards[Piece::new(Piece::BLACK_KING)];

    let bishop_attacks = Magics::bishop_attacks(sqr, board.all_pieces_bitboard);
    let rook_attacks = Magics::rook_attacks(sqr, board.all_pieces_bitboard);

    let knight_attackers = knights & Precomputed::knight_moves(sqr);
    let king_attackers = kings & Precomputed::king_moves(sqr);

    let bishop_attackers = bishops & bishop_attacks;
    let rook_attackers = rooks & rook_attacks;
    let queen_attackers = queens & (bishop_attacks | rook_attacks);
    let pawn_attackers = pawns & (Precomputed::pawn_attacks(sqr.to_bitboard(), true) | Precomputed::pawn_attacks(sqr.to_bitboard(), false));

    knight_attackers | king_attackers | bishop_attackers | rook_attackers | queen_attackers | pawn_attackers
}

#[must_use]
fn attacking_pins(board: &Board, sqr: Coord, pin_rays_w: BitBoard, pin_rays_b: BitBoard) -> BitBoard {
    let pinned = board.all_pieces_bitboard & (pin_rays_w | pin_rays_b);

    let pawns = board.piece_bitboards[Piece::new(Piece::WHITE_PAWN)] | board.piece_bitboards[Piece::new(Piece::BLACK_PAWN)];
    let knights = board.piece_bitboards[Piece::new(Piece::WHITE_KNIGHT)] | board.piece_bitboards[Piece::new(Piece::BLACK_KNIGHT)];
    let bishops = board.piece_bitboards[Piece::new(Piece::WHITE_BISHOP)] | board.piece_bitboards[Piece::new(Piece::BLACK_BISHOP)];
    let rooks = board.piece_bitboards[Piece::new(Piece::WHITE_ROOK)] | board.piece_bitboards[Piece::new(Piece::BLACK_ROOK)];
    let queens = board.piece_bitboards[Piece::new(Piece::WHITE_QUEEN)] | board.piece_bitboards[Piece::new(Piece::BLACK_QUEEN)];
    let kings = board.piece_bitboards[Piece::new(Piece::WHITE_KING)] | board.piece_bitboards[Piece::new(Piece::BLACK_KING)];

    let bishop_attacks = Magics::bishop_attacks(sqr, board.all_pieces_bitboard);
    let rook_attacks = Magics::rook_attacks(sqr, board.all_pieces_bitboard);

    let knight_attackers = knights & !pinned & Precomputed::knight_moves(sqr);
    let king_attackers = kings & Precomputed::king_moves(sqr);

    let bishop_attackers = bishops & bishop_attacks;
    let rook_attackers = rooks & rook_attacks;
    let queen_attackers = queens & (bishop_attacks | rook_attacks);
    let pawn_attackers = pawns & (Precomputed::pawn_attacks(sqr.to_bitboard(), true) | Precomputed::pawn_attacks(sqr.to_bitboard(), false));

    let mut pinned_attackers = pinned & (bishop_attackers | rook_attackers | queen_attackers | pawn_attackers);
    let mut attackers = knight_attackers | king_attackers
        | (!pinned & (bishop_attackers | rook_attackers | queen_attackers | pawn_attackers));

    while pinned_attackers.0 != 0 {
        let s = Coord::from_idx(pinned_attackers.pop_lsb() as i8);
        let is_white = board.square[s].is_white();
        let king = board.king_square[if is_white { Board::WHITE_INDEX } else { Board::BLACK_INDEX }];
        if Precomputed::align_mask(s, king).contains_square(sqr.square()) {
            attackers |= s.to_bitboard();
        }
    }

    attackers
}

#[must_use]
fn consider_xrays(board: &Board, occ: BitBoard, sqr: Coord) -> BitBoard {
    let bishops = occ & (board.piece_bitboards[Piece::new(Piece::WHITE_BISHOP)] | board.piece_bitboards[Piece::new(Piece::BLACK_BISHOP)]);
    let rooks = occ & (board.piece_bitboards[Piece::new(Piece::WHITE_ROOK)] | board.piece_bitboards[Piece::new(Piece::BLACK_ROOK)]);
    let queens = occ & (board.piece_bitboards[Piece::new(Piece::WHITE_QUEEN)] | board.piece_bitboards[Piece::new(Piece::BLACK_QUEEN)]);

    let bishop_attacks = Magics::bishop_attacks(sqr, occ);
    let rook_attacks = Magics::rook_attacks(sqr, occ);

    let bishop_attackers = bishops & bishop_attacks;
    let rook_attackers = rooks & rook_attacks;
    let queen_attackers = queens & (bishop_attacks | rook_attacks);
    
    bishop_attackers | rook_attackers | queen_attackers
}

#[must_use]
fn consider_xrays_pins(board: &Board, occ: BitBoard, sqr: Coord, pin_rays_w: BitBoard, pin_rays_b: BitBoard) -> BitBoard {
    let pinned = occ & (pin_rays_w | pin_rays_b);

    let bishops = occ & (board.piece_bitboards[Piece::new(Piece::WHITE_BISHOP)] | board.piece_bitboards[Piece::new(Piece::BLACK_BISHOP)]);
    let rooks = occ & (board.piece_bitboards[Piece::new(Piece::WHITE_ROOK)] | board.piece_bitboards[Piece::new(Piece::BLACK_ROOK)]);
    let queens = occ & (board.piece_bitboards[Piece::new(Piece::WHITE_QUEEN)] | board.piece_bitboards[Piece::new(Piece::BLACK_QUEEN)]);

    let bishop_attacks = Magics::bishop_attacks(sqr, occ);
    let rook_attacks = Magics::rook_attacks(sqr, occ);

    let bishop_attackers = bishops & bishop_attacks;
    let rook_attackers = rooks & rook_attacks;
    let queen_attackers = queens & (bishop_attacks | rook_attacks);

    let mut pinned_attackers = pinned & (bishop_attackers | rook_attackers | queen_attackers);
    let mut attackers = !pinned & (bishop_attackers | rook_attackers | queen_attackers);

    while pinned_attackers.0 != 0 {
        let s = Coord::from_idx(pinned_attackers.pop_lsb() as i8);
        let is_white = board.square[s].is_white();
        let king = board.king_square[if is_white { Board::WHITE_INDEX } else { Board::BLACK_INDEX }];
        if Precomputed::align_mask(s, king).contains_square(sqr.square()) {
            attackers |= s.to_bitboard();
        }
    }

    attackers
}

pub fn static_exchange_eval(board: &Board, m: Move, target: Piece, mut attacker: Piece) -> i32 {
    let mut gain = [0; 32];
    let mut depth = 0;
    let mut is_white = board.white_to_move;
    let can_xray = board.all_pieces_bitboard & !(
        board.piece_bitboards[Piece::new(Piece::WHITE_KNIGHT)] | board.piece_bitboards[Piece::new(Piece::BLACK_KNIGHT)]
        | board.piece_bitboards[Piece::new(Piece::WHITE_KING)] | board.piece_bitboards[Piece::new(Piece::BLACK_KING)]);

    let mut from_set = BitBoard(1 << m.start_idx());
    let mut occ = board.all_pieces_bitboard;
    let mut attack_def = attacking(board, m.target());
    gain[depth] = MoveOrdering::piece_value_score(target.piece_type());

    while from_set.0 != 0 {
        depth += 1;
        gain[depth] = MoveOrdering::piece_value_score(attacker.piece_type()) - gain[depth - 1];
        if gain[depth].max(-gain[depth - 1]) < 0 { break };

        // Remove the from piece from the temporary bitboards
        attack_def ^= from_set;
        occ ^= from_set;

        // If the piece we just moved could have opened a new attack (from a previously xraying
        // sliding piece), we check for new attackers
        if (from_set & can_xray).0 != 0 {
            attack_def |= consider_xrays(board, occ, m.target());
        }

        is_white = !is_white;
        (attacker, from_set) = least_valuable_piece(board, attack_def, is_white);
    }

    while {
        depth -= 1;
        depth != 0
    } {
        gain[depth - 1] = -(gain[depth].max(-gain[depth - 1]));
    }

    gain[0]
}
