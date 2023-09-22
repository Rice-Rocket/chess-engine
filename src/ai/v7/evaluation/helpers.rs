use crate::board::{board::Board, coord::Coord, piece::Piece};

use super::{utils::sum_sqrs, perspective::Perspective};



// Determines the number of pawns of the specified perspective
pub fn pawn_count(board: &Board, perspective: Perspective) -> i32 {
    sum_sqrs(pawn_count_sqr, board, perspective)
}
// Determines if a pawn of the specified perspective is on the given square
pub fn pawn_count_sqr(board: &Board, perspective: Perspective, sqr: Coord) -> i32 {
    if board.square[sqr.index()] == perspective.friendly_piece(Piece::PAWN) {
        return 1;
    }
    return 0;
}

// Determines the number of knights of the specified perspective
pub fn knight_count(board: &Board, perspective: Perspective) -> i32 {
    sum_sqrs(knight_count_sqr, board, perspective)
}
// Determines if a knight of the specified perspective is on the given square
pub fn knight_count_sqr(board: &Board, perspective: Perspective, sqr: Coord) -> i32 {
    if board.square[sqr.index()] == perspective.friendly_piece(Piece::KNIGHT) {
        return 1;
    }
    return 0;
}

// Determines the number of bishops of the specified perspective
pub fn bishop_count(board: &Board, perspective: Perspective) -> i32 {
    sum_sqrs(bishop_count_sqr, board, perspective)
}
// Determines if a bishop of the specified perspective is on the given square
pub fn bishop_count_sqr(board: &Board, perspective: Perspective, sqr: Coord) -> i32 {
    if board.square[sqr.index()] == perspective.friendly_piece(Piece::BISHOP) {
        return 1;
    }
    return 0;
}

// Determines the number of rooks of the specified perspective
pub fn rook_count(board: &Board, perspective: Perspective) -> i32 {
    sum_sqrs(rook_count_sqr, board, perspective)
}
// Determines if a rook of the specified perspective is on the given square
pub fn rook_count_sqr(board: &Board, perspective: Perspective, sqr: Coord) -> i32 {
    if board.square[sqr.index()] == perspective.friendly_piece(Piece::ROOK) {
        return 1;
    }
    return 0;
}

// Determines the number of queens of the specified perspective
pub fn queen_count(board: &Board, perspective: Perspective) -> i32 {
    sum_sqrs(queen_count_sqr, board, perspective)
}
// Determines if a queen of the specified perspective is on the given square
pub fn queen_count_sqr(board: &Board, perspective: Perspective, sqr: Coord) -> i32 {
    if board.square[sqr.index()] == perspective.friendly_piece(Piece::QUEEN) {
        return 1;
    }
    return 0;
}

// Determines the number of pieces of the specified perspective
pub fn piece_count(board: &Board, perspective: Perspective) -> i32 {
    sum_sqrs(piece_count_sqr, board, perspective)
}
// Determines the number of pieces of the specified perspective on the given square
pub fn piece_count_sqr(board: &Board, perspective: Perspective, sqr: Coord) -> i32 {
    let piece = board.square[sqr.index()];
    let some = piece.piece_type() != Piece::NONE;
    return if some && perspective.is_color(piece.color()) { 1 } else { 0 };
}



// Determines if there are two bishops of opposite colors
pub fn opposite_bishops(board: &Board) -> bool {
    if bishop_count(board, Perspective::White) != 1 { return false; }
    if bishop_count(board, Perspective::Black) != 1 { return false; }
    let mut color = [0, 0];
    for sqr in Coord::iterate_squares() {
        if board.square[sqr.index()] == Piece::new(Piece::BLACK_BISHOP) {
            color[0] = (sqr.file() + sqr.rank()) % 2;
        }
        if board.square[sqr.index()] == Piece::new(Piece::WHITE_BISHOP) {
            color[1] = (sqr.file() + sqr.rank()) % 2;
        }
    }
    return if color[0] == color[1] { false } else { true };
}