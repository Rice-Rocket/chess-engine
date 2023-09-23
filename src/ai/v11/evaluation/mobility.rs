use crate::board::{board::Board, coord::Coord, piece::Piece};
use super::attack::{knight_attack_sqr, bishop_xray_attack, rook_xray_attack, queen_attack};
use super::{perspective::Perspective, king::blockers_for_king_sqr};
use super::utils::sum_sqrs;


const MOBILITY_BONUS_MG: [[i32; 28]; 4] = [
    [-62,-53,-12,-4,3,13,22,28,33,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0],
    [-48,-20,16,26,38,51,55,63,63,68,81,81,91,98,0,0,0,0,0,0,0,0,0,0,0,0,0,0],
    [-60,-20,2,3,3,11,22,31,40,40,41,48,57,57,62,0,0,0,0,0,0,0,0,0,0,0,0,0],
    [-30,-12,-8,-9,20,23,23,35,38,53,64,65,65,66,67,67,72,72,77,79,93,108,108,108,110,114,114,116]
];
const MOBILITY_BONUS_EG: [[i32; 28]; 4] = [
    [-81,-56,-31,-16,5,11,17,20,25,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0],
    [-59,-23,-3,13,24,42,54,57,65,73,78,86,88,97,0,0,0,0,0,0,0,0,0,0,0,0,0,0],
    [-78,-17,23,39,70,99,103,121,134,139,158,164,168,169,172,0,0,0,0,0,0,0,0,0,0,0,0,0],
    [-48,-30,-7,19,40,55,59,75,78,96,96,100,121,127,131,133,136,141,147,150,151,168,168,171,182,182,192,219]
];

pub fn mobility_bonus(board: &Board, per: Perspective, mg: bool) -> i32 {
    let mut sum = 0;
    let bonus = if mg { MOBILITY_BONUS_MG } else { MOBILITY_BONUS_EG };
    for sqr in Coord::iterate_squares() {
        let piece = board.square[sqr.index()];
        if !per.is_color(piece.color()) { continue; }
        let ptype = piece.piece_type();
        if ptype < 2 || ptype > 5 { continue; }
        let mobility = mobility_sqr(board, per, sqr);
        if mobility >= 0 && mobility < 28 {
            sum += bonus[(ptype - 2) as usize][mobility as usize];
        }
    }
    sum
}

pub fn mobility(board: &Board, per: Perspective) -> i32 {
    sum_sqrs(mobility_sqr, board, per)
}

pub fn mobility_sqr(board: &Board, per: Perspective, sqr: Coord) -> i32 {
    let mut v = 0;
    let piece = board.square[sqr.index()];
    let ptype = piece.piece_type();
    if !per.is_color(piece.color()) { return 0; }
    for sqr2 in Coord::iterate_squares() {
        if mobility_area_sqr(board, per, sqr2) == 0 { continue; }
        let has_queen = board.square[sqr2.index()] == per.friendly_piece(Piece::QUEEN);
        if ptype == Piece::KNIGHT && knight_attack_sqr(board, per, sqr2, Some(sqr)) > 0 && !has_queen { v += 1 };
        if ptype == Piece::BISHOP && bishop_xray_attack(board, per, sqr2, Some(sqr)) > 0 && !has_queen { v += 1 };
        if ptype == Piece::ROOK && rook_xray_attack(board, per, sqr2, Some(sqr)) > 0 { v += 1 };
        if ptype == Piece::QUEEN && queen_attack(board, per, sqr2, Some(sqr)) > 0 { v += 1 };
    };
    v
}


pub fn mobility_area(board: &Board, per: Perspective) -> i32 {
    sum_sqrs(mobility_area_sqr, board, per)
}

pub fn mobility_area_sqr(board: &Board, per: Perspective, sqr: Coord) -> i32 {
    if board.square[sqr.index()] == per.friendly_piece(Piece::KING) { return 0; }
    if board.square[sqr.index()] == per.friendly_piece(Piece::QUEEN) { return 0; }
    let c1 = Coord::new(sqr.file() - 1, per.rank_farther_by(sqr.rank(), 1));
    let c2 = Coord::new(sqr.file() + 1, per.rank_farther_by(sqr.rank(), 1));
    if c1.is_valid() {
        if board.square[c1.index()] == per.enemy_piece(Piece::PAWN) { return 0; }
    }
    if c2.is_valid() {
        if board.square[c2.index()] == per.enemy_piece(Piece::PAWN) { return 0; }
    }
    let c1 = Coord::new(sqr.file(), per.rank_farther_by(sqr.rank(), 1));
    if c1.is_valid() {
        if board.square[sqr.index()] == per.friendly_piece(Piece::PAWN) && 
            (per.rank_is_close_half(per.rank_farther_by(sqr.rank(), 1)) || board.square[c1.index()].piece_type() != Piece::NONE) { return 0; }
    }
    if blockers_for_king_sqr(board, per.other(), sqr) > 0 { return 0; }
    return 1;
}