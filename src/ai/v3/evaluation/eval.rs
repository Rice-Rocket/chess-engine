use crate::board::{board::Board, piece::Piece};



pub struct Evaluation {}

impl Evaluation {
    pub const PAWN_VALUE: f32 = 100.0;
    pub const KNIGHT_VALUE: f32 = 300.0;
    pub const BISHOP_VALUE: f32 = 320.0;
    pub const ROOK_VALUE: f32 = 500.0;
    pub const QUEEN_VALUE: f32 = 900.0;

    // Performs evaluation of the board
    // A positive evaluation is in white's favor, while negative is in black's
    pub fn evaluate(board: &Board) -> f32 {
        let white_pawns = board.get_piece_list(Piece::new(Piece::WHITE_PAWN)).count() as f32;
        let black_pawns = board.get_piece_list(Piece::new(Piece::BLACK_PAWN)).count() as f32;
        let white_knights = board.get_piece_list(Piece::new(Piece::WHITE_KNIGHT)).count() as f32;
        let black_knights = board.get_piece_list(Piece::new(Piece::BLACK_KNIGHT)).count() as f32;
        let white_bishops = board.get_piece_list(Piece::new(Piece::WHITE_BISHOP)).count() as f32;
        let black_bishops = board.get_piece_list(Piece::new(Piece::BLACK_BISHOP)).count() as f32;
        let white_rooks = board.get_piece_list(Piece::new(Piece::WHITE_ROOK)).count() as f32;
        let black_rooks = board.get_piece_list(Piece::new(Piece::BLACK_ROOK)).count() as f32;
        let white_queens = board.get_piece_list(Piece::new(Piece::WHITE_QUEEN)).count() as f32;
        let black_queens = board.get_piece_list(Piece::new(Piece::BLACK_QUEEN)).count() as f32;

        let pawn_eval = white_pawns * Self::PAWN_VALUE - black_pawns * Self::PAWN_VALUE;
        let knight_eval = white_knights * Self::KNIGHT_VALUE - black_knights * Self::KNIGHT_VALUE;
        let bishop_eval = white_bishops * Self::BISHOP_VALUE - black_bishops * Self::BISHOP_VALUE;
        let rook_eval = white_rooks * Self::ROOK_VALUE - black_rooks * Self::ROOK_VALUE;
        let queen_eval = white_queens * Self::QUEEN_VALUE - black_queens * Self::QUEEN_VALUE;

        let perspective = if board.white_to_move { 1.0 } else { -1.0 };
        let eval = pawn_eval + knight_eval + bishop_eval + rook_eval + queen_eval;
        return eval * perspective;
    }
}