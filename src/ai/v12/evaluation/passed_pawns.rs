use crate::board::{coord::Coord, piece::Piece};

use super::{perspective::Perspective, pawns::supported_sqr, pos::{sum_sqrs, PositionEvaluation}};


pub fn candidate_passed(pos: &PositionEvaluation, per: Perspective) -> i32 {
    sum_sqrs(candidate_passed_sqr, pos, per)
}

// Determines if a pawn is passed or is a candidate passer
pub fn candidate_passed_sqr(pos: &PositionEvaluation, per: Perspective, sqr: Coord) -> i32 {
    if pos.square(sqr) != per.friendly_piece(Piece::PAWN) { return 0 };
    let mut ty1 = per.home_outbounds_rank();
    let mut ty2 = per.home_outbounds_rank();

    for y in per.iter_ranks_forward_excl(sqr.rank()) {
        if pos.square(Coord::new(sqr.file(), y)) == per.friendly_piece(Piece::PAWN) {
            return 0;
        }
        if pos.square(Coord::new(sqr.file(), y)) == per.enemy_piece(Piece::PAWN) {
            ty1 = y;
        }
        let sqr_1 = Coord::new(sqr.file() - 1, y);
        let sqr_2 = Coord::new(sqr.file() + 1, y);
        if sqr_1.is_valid() && sqr_2.is_valid() {
            if pos.square(sqr_1) == per.enemy_piece(Piece::PAWN) || 
                pos.square(sqr_2) == per.enemy_piece(Piece::PAWN) {
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
        if pos.square(Coord::new(sqr.file() - 1, per.rank_closer_by(sqr.rank(), 1))) == per.friendly_piece(Piece::PAWN)
        && pos.square(Coord::new(sqr.file() - 1, sqr.rank())) != per.enemy_piece(Piece::PAWN)
        && pos.square(Coord::new(sqr.file() - 2, per.rank_farther_by(sqr.rank(), 1))) != per.enemy_piece(Piece::PAWN) { return 1; }

        if pos.square(Coord::new(sqr.file() + 1, per.rank_closer_by(sqr.rank(), 1))) == per.friendly_piece(Piece::PAWN)
        && pos.square(Coord::new(sqr.file() + 1, sqr.rank())) != per.enemy_piece(Piece::PAWN)
        && pos.square(Coord::new(sqr.file() + 2, per.rank_farther_by(sqr.rank(), 1))) != per.enemy_piece(Piece::PAWN) { return 1; }
    }

    if pos.square(Coord::new(sqr.file(), per.rank_farther_by(sqr.rank(), 1))) == per.enemy_piece(Piece::PAWN) { return 0; }
    
    let lever = if pos.square(Coord::new(sqr.file() - 1, per.rank_farther_by(sqr.rank(), 1))) == per.enemy_piece(Piece::PAWN) { 1 } else { 0 }
        + if pos.square(Coord::new(sqr.file() + 1, per.rank_farther_by(sqr.rank(), 1))) == per.enemy_piece(Piece::PAWN) { 1 } else { 0 };
    
    let lever_push = if pos.square(Coord::new(sqr.file() - 1, per.rank_farther_by(sqr.rank(), 2))) == per.enemy_piece(Piece::PAWN) { 1 } else { 0 }
        + if pos.square(Coord::new(sqr.file() + 1, per.rank_farther_by(sqr.rank(), 2))) == per.enemy_piece(Piece::PAWN) { 1 } else { 0 };
    
    let phalanx = if pos.square(Coord::new(sqr.file() - 1, sqr.rank())) == per.friendly_piece(Piece::PAWN) { 1 } else { 0 }
        + if pos.square(Coord::new(sqr.file() + 1, sqr.rank())) == per.friendly_piece(Piece::PAWN) { 1 } else { 0 };
    
    if lever - supported_sqr(pos, per, sqr) > 1 { return 0; }
    if lever_push - phalanx > 0 { return 0; }
    if lever > 0 && lever_push > 0 { return 0; }
    return 1;
}