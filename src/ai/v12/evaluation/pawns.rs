use crate::board::{coord::Coord, piece::Piece};

use super::{perspective::Perspective, pos::PositionEvaluation};


pub fn supported_sqr(pos: &PositionEvaluation, per: Perspective, sqr: Coord) -> i32 {
    if pos.square(sqr) != per.friendly_piece(Piece::PAWN) { return 0; }
    return if pos.square(Coord::new(sqr.file() - 1, per.rank_closer_by(sqr.rank(), 1))) == per.friendly_piece(Piece::PAWN) { 1 } else { 0 }
        + if pos.square(Coord::new(sqr.file() + 1, per.rank_closer_by(sqr.rank(), 1))) == per.friendly_piece(Piece::PAWN) { 1 } else { 0 } 
}