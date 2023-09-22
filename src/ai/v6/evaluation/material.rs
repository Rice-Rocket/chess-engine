use crate::board::{board::Board, coord::Coord, piece::Piece};

use super::{perspective::Perspective, utils::sum_sqrs};



pub fn non_pawn_material(board: &Board, perspective: Perspective) -> i32 {
    sum_sqrs(non_pawn_material_sqr, board, perspective)
}
pub fn non_pawn_material_sqr(board: &Board, perspective: Perspective, sqr: Coord) -> i32 {
    let piece = board.square[sqr.index()];
    if piece.is_perspective(perspective) {
        if piece.is_not_pawn() {
            return piece_value_bonus_sqr(board, perspective, true, sqr);
        }
    };
    return 0;
}



pub const PIECE_VALUE_BONUSES_MG: [i32; 5] = [124, 781, 825, 1276, 2538];
pub const PIECE_VALUE_BONUSES_EG: [i32; 5] = [206, 854, 915, 1380, 2682];
pub fn piece_value_bonus(board: &Board, perspective: Perspective, mg: bool) -> i32 {
    let mut sum = 0;
    for sqr in Coord::iterate_squares() {
        sum += piece_value_bonus_sqr(board, perspective, mg, sqr);
    }
    return sum;
}
pub fn piece_value_bonus_sqr(board: &Board, perspective: Perspective, mg: bool, sqr: Coord) -> i32 {
    let piece = board.square[sqr.index()];
    if piece.is_perspective(perspective) {
        let bonuses = if mg { PIECE_VALUE_BONUSES_MG } else { PIECE_VALUE_BONUSES_EG };
        if let Some(i) = piece.ptype_index() {
            if i as u8 + 1 != Piece::KING {
                return bonuses[i];
            }
        }
    }
    return 0;
}