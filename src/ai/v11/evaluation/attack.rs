use crate::board::{board::Board, coord::Coord, piece::Piece};
use super::perspective::Perspective;
use super::utils::sum_sqrs;

pub fn pinned_direction(board: &Board, per: Perspective) -> i32 {
    sum_sqrs(pinned_direction_sqr, board, per)
}

pub fn pinned_direction_sqr(board: &Board, per: Perspective, sqr: Coord) -> i32 {
    let piece = board.square[sqr.index()];
    if piece == Piece::NULL { return 0; }

    let mut color = 1;
    if !per.is_color(piece.color()) { color = -1; }
    for i in 0..8 {
        let ix = (i + if i > 3 { 1 } else { 0 }) % 3 - 1;
        let iy = (i + if i > 3 { 1 } else { 0 }) / 3 - 1;
        let mut king = false;
        for d in 1..8 {
            let c1 = Coord::new(sqr.file() + d * ix, sqr.rank() + d * iy);
            if c1.is_valid() {
                let piece = board.square[c1.index()];
                if piece == per.friendly_piece(Piece::KING) { king = true; }
                if piece != Piece::NULL { break; }
            };
        };

        if king {
            for d in 1..8 {
                let c1 = Coord::new(sqr.file() - d * ix, sqr.rank() - d * iy);
                if c1.is_valid() {
                    let piece = board.square[c1.index()];
                    if piece == per.enemy_piece(Piece::QUEEN) 
                    || (piece == per.enemy_piece(Piece::BISHOP) && ix * iy != 0)
                    || (piece == per.enemy_piece(Piece::ROOK) && ix * iy == 0) {
                        return (ix - iy * 3).abs() as i32 * color;
                    }
                    if piece != Piece::NULL { break; }
                }
            }
        }
    };
    return 0;
}

pub fn pinned_sqr(board: &Board, per: Perspective, sqr: Coord) -> i32 {
    let piece = board.square[sqr.index()];
    if piece == Piece::NULL || !per.is_color(piece.color()) { return 0; }
    return if pinned_direction_sqr(board, per, sqr) > 0 { 1 } else { 0 };
}


pub fn knight_attack_sqr(board: &Board, per: Perspective, sqr: Coord, sqr2: Option<Coord>) -> i32 {
    let mut v = 0;
    for i in 0..8 {
        let ix = ((if i > 3 { 1 } else { 0 }) + 1) * ((if (i % 4) > 1 { 1 } else { 0 }) * 2 - 1);
        let iy = (2 - (if i > 3 { 1 } else { 0 })) * ((if i % 2 == 0 { 1 } else { 0 }) * 2 - 1);

        let c1 = Coord::new(sqr.file() + ix, sqr.rank() + iy);
        if !c1.is_valid() { continue; }
        
        let piece = board.square[c1.index()];
        if piece == per.friendly_piece(Piece::KNIGHT)
        && (sqr2.is_none() || (sqr2.unwrap().file() == c1.file() && sqr2.unwrap().rank() == c1.rank()))
        && pinned_sqr(board, per, c1) == 0 { v += 1 };
    };
    v
}


pub fn bishop_xray_attack(board: &Board, per: Perspective, sqr: Coord, sqr2: Option<Coord>) -> i32 {
    let mut v = 0;
    for i in 0..4 {
        let ix = (if i > 1 { 1 } else { 0 }) * 2 - 1;
        let iy = (if i % 2 == 0 { 1 } else { 0 }) * 2 - 1;
        for d in 1..8 {
            let c1 = Coord::new(sqr.file() + d * ix, sqr.rank() + d * iy);
            if !c1.is_valid() { continue; }

            let piece = board.square[c1.index()];
            if piece == per.friendly_piece(Piece::BISHOP) 
            && (sqr2.is_none() || (sqr2.unwrap().file() == c1.file() && sqr2.unwrap().rank() == c1.rank())) {
                let dir = pinned_direction_sqr(board, per, c1);
                if dir == 0 || (ix - iy * 3).abs() == dir as i8 { v += 1; }
            }
            
            let ptype = piece.piece_type();
            if ptype != Piece::NONE && ptype != Piece::QUEEN { break; }
        }
    };
    v
}

pub fn rook_xray_attack(board: &Board, per: Perspective, sqr: Coord, sqr2: Option<Coord>) -> i32 {
    let mut v = 0;
    for i in 0..4 {
        let ix = if i == 0 { -1 } else { if i == 1 { 1 } else { 0 } };
        let iy = if i == 2 { -1 } else { if i == 3 { 1 } else  { 0 } };
        for d in 1..8 {
            let c1 = Coord::new(sqr.file() + d * ix, sqr.rank() + d * iy);
            if !c1.is_valid() { continue; }
            
            let piece = board.square[c1.index()];
            if piece == per.friendly_piece(Piece::ROOK) 
            && (sqr2.is_none() || (sqr2.unwrap().file() == c1.file() && sqr2.unwrap().rank() == c1.rank())) {
                let dir = pinned_direction_sqr(board, per, c1);
                if dir == 0 || (ix - iy * 3).abs() == dir as i8 { v += 1; }
            }
            
            let ptype = piece.piece_type();
            if piece != per.friendly_piece(Piece::ROOK) && ptype != Piece::NONE && ptype != Piece::QUEEN { break; }
        }
    };
    v
}

pub fn queen_attack(board: &Board, per: Perspective, sqr: Coord, sqr2: Option<Coord>) -> i32 {
    let mut v = 0;
    for i in 0..8 {
        let ix = (i + (if i > 3 { 1 } else { 0 })) % 3 - 1;
        let iy = (i + (if i > 3 { 1 } else { 0 })) / 3 - 1;
        for d in 1..8 {
            let c1 = Coord::new(sqr.file() + d * ix, sqr.rank() + d * iy);
            if !c1.is_valid() { continue; }
            
            let piece = board.square[c1.index()];
            if piece == per.friendly_piece(Piece::QUEEN) 
            && (sqr2.is_none() || (sqr2.unwrap().file() == c1.file() && sqr2.unwrap().rank() == c1.rank())) {
                let dir = pinned_direction_sqr(board, per, c1);
                if dir == 0 || (ix - iy * 3).abs() == dir as i8 { v += 1; }
            }

            let ptype = piece.piece_type();
            if ptype != Piece::NONE { break; }
        }
    };
    v
}