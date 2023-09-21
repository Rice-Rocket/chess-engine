use crate::board::{board::Board, piece::Piece};



pub struct Evaluation {}

impl Evaluation {
    pub const PAWN_VALUE: i32 = 100;
    pub const KNIGHT_VALUE: i32 = 300;
    pub const BISHOP_VALUE: i32 = 320;
    pub const ROOK_VALUE: i32 = 500;
    pub const QUEEN_VALUE: i32 = 900;

    // Performs evaluation of the board
    // A positive evaluation is in white's favor, while negative is in black's
    pub fn evaluate(board: &Board) -> i32 {
        let white_pawns = board.get_piece_list(Piece::new(Piece::WHITE_PAWN)).count() as i32;
        let black_pawns = board.get_piece_list(Piece::new(Piece::BLACK_PAWN)).count() as i32;
        let white_knights = board.get_piece_list(Piece::new(Piece::WHITE_KNIGHT)).count() as i32;
        let black_knights = board.get_piece_list(Piece::new(Piece::BLACK_KNIGHT)).count() as i32;
        let white_bishops = board.get_piece_list(Piece::new(Piece::WHITE_BISHOP)).count() as i32;
        let black_bishops = board.get_piece_list(Piece::new(Piece::BLACK_BISHOP)).count() as i32;
        let white_rooks = board.get_piece_list(Piece::new(Piece::WHITE_ROOK)).count() as i32;
        let black_rooks = board.get_piece_list(Piece::new(Piece::BLACK_ROOK)).count() as i32;
        let white_queens = board.get_piece_list(Piece::new(Piece::WHITE_QUEEN)).count() as i32;
        let black_queens = board.get_piece_list(Piece::new(Piece::BLACK_QUEEN)).count() as i32;

        let pawn_eval = white_pawns * Self::PAWN_VALUE - black_pawns * Self::PAWN_VALUE;
        let knight_eval = white_knights * Self::KNIGHT_VALUE - black_knights * Self::KNIGHT_VALUE;
        let bishop_eval = white_bishops * Self::BISHOP_VALUE - black_bishops * Self::BISHOP_VALUE;
        let rook_eval = white_rooks * Self::ROOK_VALUE - black_rooks * Self::ROOK_VALUE;
        let queen_eval = white_queens * Self::QUEEN_VALUE - black_queens * Self::QUEEN_VALUE;

        let perspective = if board.white_to_move { 1 } else { -1 };
        let eval = pawn_eval + knight_eval + bishop_eval + rook_eval + queen_eval;
        return eval * perspective;
    }
}