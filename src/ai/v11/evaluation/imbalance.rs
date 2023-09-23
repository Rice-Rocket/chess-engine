use crate::board::piece::Piece;
use crate::board::{board::Board, coord::Coord};
use super::perspective::Perspective;
use super::helpers::*;

pub fn imbalance_total(board: &Board) -> i32 {
    let mut eval = 0;
    eval += imbalance(board, Perspective::White) - imbalance(board, Perspective::Black);
    eval += bishop_pair(board, Perspective::White) - bishop_pair(board, Perspective::Black);
    return eval / 16
}


pub fn imbalance(board: &Board, per: Perspective) -> i32 {
    let quadratic_ours: [Vec<i32>; 5] = [vec![40,38],vec![32,255,-62],vec![0,104,4,0],vec![-26,-2,47,105,-208],vec![-189,24,117,133,-134,-6]];
    let quadratic_theirs: [Vec<i32>; 5] = [vec![36,0],vec![9,63,0],vec![59,65,42,0],vec![46,39,24,-24,0],vec![97,100,-42,137,268,0]];

    let mut sum = 0;
    let mut bishop = [0, 0];

    bishop[0] = piece_count(board, per.other(), Piece::BISHOP);
    bishop[1] = piece_count(board, per, Piece::BISHOP);

    for sqr in Coord::iterate_squares() {
        let piece = board.square[sqr.index()];
        if per.other().is_color(piece.color()) { continue; }

        if let Some(j) = piece.ptype_index() {
            if piece.piece_type() == Piece::KING { continue; }
            let mut v = 0;

            for sqr_1 in Coord::iterate_squares() {
                let piece_1 = board.square[sqr_1.index()];
                if piece_1.piece_type() == Piece::KING { continue; }
                if let Some(i) = piece_1.ptype_index() {
                    if i > j { continue; }
                    if per.is_color(piece_1.color()) {
                        v += quadratic_ours[j][i + 1];
                    } else {
                        v += quadratic_theirs[j][i + 1];
                    }
                }
            }
            if bishop[0] > 1 { v += quadratic_theirs[j][0] };
            if bishop[1] > 1 { v += quadratic_ours[j][0] };
            sum += v;
        }
    }
    return sum;
}


pub fn bishop_pair(board: &Board, per: Perspective) -> i32 {
    if bishop_count(board, per) < 2 {
        return 0;
    } else {
        return 1438;
    }
}