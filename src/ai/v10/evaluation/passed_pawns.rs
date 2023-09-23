use crate::board::{board::Board, coord::Coord, piece::Piece};

use super::{perspective::Perspective, pawns::supported_sqr, utils::sum_sqrs};


pub fn candidate_passed(board: &Board, per: Perspective) -> i32 {
    sum_sqrs(candidate_passed_sqr, board, per)
}

// Determines if a pawn is passed or is a candidate passer
pub fn candidate_passed_sqr(board: &Board, per: Perspective, sqr: Coord) -> i32 {
    if board.square[sqr.index()] != per.friendly_piece(Piece::PAWN) { return 0 };
    let mut ty1 = per.home_outbounds_rank();
    let mut ty2 = per.home_outbounds_rank();

    for y in per.iter_ranks_forward_from_excl(sqr.rank()) {
        if board.square[Coord::new(sqr.file(), y).index()] == per.friendly_piece(Piece::PAWN) {
            return 0;
        }
        if board.square[Coord::new(sqr.file(), y).index()] == per.enemy_piece(Piece::PAWN) {
            ty1 = y;
        }
        let sqr_1 = Coord::new(sqr.file() - 1, y);
        let sqr_2 = Coord::new(sqr.file() + 1, y);
        if sqr_1.is_valid() && sqr_2.is_valid() {
            if board.square[sqr_1.index()] == per.enemy_piece(Piece::PAWN) || 
                board.square[sqr_2.index()] == per.enemy_piece(Piece::PAWN) {
                ty2 = y;
            }
        }
    };

    if ty1 == per.enemy_outbounds_rank() && per.rank_is_closer_or_eq(ty2, per.rank_farther_by(sqr.rank(), 1)) {
        return 1;
    }
    if per.rank_is_farther(ty2, per.rank_farther_by(sqr.rank(), 2)) || per.rank_is_farther(ty1, per.rank_farther_by(sqr.rank(), 1)) {
        return 0; 
    }
    if per.rank_is_closer_or_eq(ty2, sqr.rank()) && ty1 == sqr.rank() - 1 && per.rank_is_far_half(sqr.rank()) {
        let c1 = Coord::new(sqr.file() - 1, per.rank_closer_by(sqr.rank(), 1));
        let c2 = Coord::new(sqr.file() - 1, sqr.rank());
        let c3 = Coord::new(sqr.file() - 2, per.rank_farther_by(sqr.rank(), 1));
        if c1.is_valid() && c2.is_valid() && c3.is_valid() {
            if board.square[c1.index()] == per.friendly_piece(Piece::PAWN)
            && board.square[c2.index()] != per.enemy_piece(Piece::PAWN)
            && board.square[c3.index()] != per.enemy_piece(Piece::PAWN) { return 1; }
        }

        let c1 = Coord::new(sqr.file() + 1, per.rank_closer_by(sqr.rank(), 1));
        let c2 = Coord::new(sqr.file() + 1, sqr.rank());
        let c3 = Coord::new(sqr.file() + 2, per.rank_farther_by(sqr.rank(), 1));
        if c1.is_valid() && c2.is_valid() && c3.is_valid() {
            if board.square[c1.index()] == per.friendly_piece(Piece::PAWN)
            && board.square[c2.index()] != per.enemy_piece(Piece::PAWN)
            && board.square[c3.index()] != per.enemy_piece(Piece::PAWN) { return 1; }
        }
    }

    if board.square[Coord::new(sqr.file(), per.rank_farther_by(sqr.rank(), 1)).index()] == per.enemy_piece(Piece::PAWN) { return 0; }
    
    let c1 = Coord::new(sqr.file() - 1, per.rank_farther_by(sqr.rank(), 1));
    let c2 = Coord::new(sqr.file() + 1, per.rank_farther_by(sqr.rank(), 1));
    let lever = if c1.is_valid() && board.square[c1.index()] == per.enemy_piece(Piece::PAWN) { 1 } else { 0 }
        + if c2.is_valid() && board.square[c2.index()] == per.enemy_piece(Piece::PAWN) { 1 } else { 0 };
    
    let c1 = Coord::new(sqr.file() - 1, per.rank_farther_by(sqr.rank(), 2));
    let c2 = Coord::new(sqr.file() + 1, per.rank_farther_by(sqr.rank(), 2));
    let lever_push = if c1.is_valid() && board.square[c1.index()] == per.enemy_piece(Piece::PAWN) { 1 } else { 0 }
        + if c2.is_valid() && board.square[c2.index()] == per.enemy_piece(Piece::PAWN) { 1 } else { 0 };
    
    let c1 = Coord::new(sqr.file() - 1, sqr.rank());
    let c2 = Coord::new(sqr.file() + 1, sqr.rank());
    let phalanx = if c1.is_valid() && board.square[c1.index()] == per.friendly_piece(Piece::PAWN) { 1 } else { 0 }
        + if c2.is_valid() && board.square[c2.index()] == per.friendly_piece(Piece::PAWN) { 1 } else { 0 };
    
    if lever - supported_sqr(board, per, sqr) > 1 { return 0; }
    if lever_push - phalanx > 0 { return 0; }
    if lever > 0 && lever_push > 0 { return 0; }
    return 1;
}