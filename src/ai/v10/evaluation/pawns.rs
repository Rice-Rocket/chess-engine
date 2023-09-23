use crate::board::{board::Board, coord::Coord, piece::Piece};

use super::perspective::Perspective;


pub fn supported_sqr(board: &Board, per: Perspective, sqr: Coord) -> i32 {
    if board.square[sqr.index()] != per.friendly_piece(Piece::PAWN) { return 0; }
    let c1 = Coord::new(sqr.file() - 1, per.rank_closer_by(sqr.rank(), 1));
    let c2 = Coord::new(sqr.file() + 1, per.rank_closer_by(sqr.rank(), 1));
    if !c1.is_valid() || !c2.is_valid() { return 0; }
    return if board.square[c1.index()] == per.friendly_piece(Piece::PAWN) { 1 } else { 0 }
        + if board.square[c2.index()] == per.friendly_piece(Piece::PAWN) { 1 } else { 0 } 
}