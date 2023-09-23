use crate::board::{board::Board, coord::Coord, piece::Piece};

use super::perspective::Perspective;



// Determines the number of pawns of the specified perspective
pub fn pawn_count(board: &Board, perspective: Perspective) -> i32 {
    board.get_piece_list(perspective.friendly_piece(Piece::PAWN)).count() as i32
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
    board.get_piece_list(perspective.friendly_piece(Piece::KNIGHT)).count() as i32
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
    board.get_piece_list(perspective.friendly_piece(Piece::BISHOP)).count() as i32
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
    board.get_piece_list(perspective.friendly_piece(Piece::ROOK)).count() as i32
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
    board.get_piece_list(perspective.friendly_piece(Piece::QUEEN)).count() as i32
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
    board.color_bitboards[perspective.color_idx()].count_ones() as i32
}
// Determines the number of pieces of the specified perspective on the given square
pub fn piece_count_sqr(board: &Board, perspective: Perspective, sqr: Coord) -> i32 {
    let piece = board.square[sqr.index()];
    let some = piece.piece_type() != Piece::NONE;
    return if some && perspective.is_color(piece.color()) { 1 } else { 0 };
}



// Determines if there are two bishops of opposite colors
pub fn opposite_bishops(board: &Board) -> bool {
    let mut color = [0, 0];
    let lst_1 = board.get_piece_list(Piece::new(Piece::BLACK_BISHOP));
    if lst_1.count() == 1 {
        let c1 = lst_1.occupied_squares[0];
        color[0] = (c1.file() + c1.rank()) % 2;
    } else { return false; }
    let lst_2 = board.get_piece_list(Piece::new(Piece::WHITE_BISHOP));
    if lst_2.count() == 1 {
        let c2 = lst_2.occupied_squares[0];
        color[1] = (c2.file() + c2.rank()) % 2;
    } else { return false; }
    return if color[0] == color[1] { false } else { true };
}