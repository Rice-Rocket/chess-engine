use crate::board::board::Board;

use super::perspective::Perspective;
use super::material::*;


pub struct Evaluation {}

impl Evaluation {
    pub const PAWN_VALUE: i32 = 100;
    pub const KNIGHT_VALUE: i32 = 300;
    pub const BISHOP_VALUE: i32 = 320;
    pub const ROOK_VALUE: i32 = 500;
    pub const QUEEN_VALUE: i32 = 900;

    // Performs evaluation of the board
    // A positive value means a better position for the player to move
    pub fn evaluate(board: &Board) -> i32 {
        let mg = Self::midgame_eval(board, false);
        // let mut eg = Self::endgame_eval(board, false);
        // let p = Self::phase(board);
        // let rule50 = Self::rule50(board);

        // eg = eg * Self::scale_factor(board, eg) / 64;
        // let mut eval = ((mg * p + (eg * (128 - p))) as f32 / 128.0).floor() as i32;

        // eval += Self::tempo(board);
        // eval = ((eval * (100 - rule50)) as f32 / 100.0).floor() as i32;

        let perspective = if board.white_to_move { 1 } else { -1 };
        return mg * perspective;
    }

    // Evaluates position during opening and middle game stages
    fn midgame_eval(board: &Board, _nowinnable: bool) -> i32 {
        let mut eval = 0;

        eval += piece_value_bonus(board, Perspective::White, true) - piece_value_bonus(board, Perspective::Black, true);
        eval += psqt_bonus(board, Perspective::White, true) - psqt_bonus(board, Perspective::Black, true);

        return eval;
    }

    // fn endgame_eval(board: &Board, _nowinnable: bool) -> i32 {
    //     let mut eval = 0;

    //     eval += piece_value_bonus(board, Perspective::White, false) - piece_value_bonus(board, Perspective::Black, false);
    //     eval += psqt_bonus(board, Perspective::White, false) - psqt_bonus(board, Perspective::Black, false);

    //     return eval;
    // }
    
    // // For tapered evaluation
    // const PHASE_LIMIT_MG: i32 = 15258;
    // const PHASE_LIMIT_EG: i32 = 3915;
    // fn phase(board: &Board) -> i32 {
    //     let mut npm = non_pawn_material(board, Perspective::White) + non_pawn_material(board, Perspective::Black);
    //     npm = Self::PHASE_LIMIT_EG.max(Self::PHASE_LIMIT_MG.min(npm));
    //     return (((npm - Self::PHASE_LIMIT_EG) * 128) as f32 / (Self::PHASE_LIMIT_MG - Self::PHASE_LIMIT_EG) as f32) as i32;
    // }

    // fn rule50(board: &Board) -> i32 {
    //     board.current_state.fifty_move_counter as i32
    // }

    // fn scale_factor(board: &Board, eg: i32) -> i32 {
    //     let mut sf = 64;
    //     let perspective = if eg > 0 { Perspective::White } else { Perspective::Black };
    //     let (pc_w, pc_b) = (pawn_count(board, perspective), pawn_count(board, perspective.other()));
    //     let (qc_w, qc_b) = (queen_count(board, perspective), queen_count(board, perspective.other()));
    //     let (bc_w, bc_b) = (bishop_count(board, perspective), bishop_count(board, perspective.other()));
    //     let (nc_w, nc_b) = (knight_count(board, perspective), knight_count(board, perspective.other()));
    //     let (npm_w, npm_b) = (non_pawn_material(board, perspective), non_pawn_material(board, perspective.other()));
        
    //     const BISHOP_VALUE_MG: i32 = PIECE_VALUE_BONUSES_MG[2];
    //     const ROOK_VALUE_MG: i32 = PIECE_VALUE_BONUSES_MG[3];

    //     if pc_w == 0 && npm_w - npm_b <= BISHOP_VALUE_MG {
    //         sf = if npm_w < ROOK_VALUE_MG { 0 } else { if npm_b <= BISHOP_VALUE_MG { 4 } else { 14 } };
    //     };
    //     if sf == 64 {
    //         let ob = opposite_bishops(board);
    //         if ob && npm_w == BISHOP_VALUE_MG && npm_b == BISHOP_VALUE_MG {
    //             sf = 22 + 4 * candidate_passed(board, perspective);
    //         } else if ob {
    //             sf = 22 + 3 * piece_count(board, perspective);
    //         } else {
    //             if npm_w == ROOK_VALUE_MG && npm_b == ROOK_VALUE_MG && pc_w - pc_b <= 1 {
    //                 let (mut pawn_king_b, mut pc_w_flank) = (false, [0, 0]);
    //                 for sqr in Coord::iterate_squares() {
    //                     if board.square[sqr.index()] == perspective.friendly_piece(Piece::PAWN) {
    //                         pc_w_flank[if sqr.file() < 4 { 1 } else { 0 }] = 1;
    //                     }
    //                     if board.square[sqr.index()] == perspective.friendly_piece(Piece::KING) {
    //                         for x in -1..=1 {
    //                             for y in -1..=1 {
    //                                 let new_sqr = Coord::new(sqr.file() + x, sqr.rank() + y);
    //                                 if new_sqr.is_valid() {
    //                                     if board.square[new_sqr.index()].piece_type() == Piece::BLACK_PAWN {
    //                                         pawn_king_b = true;
    //                                     }
    //                                 }
    //                             }
    //                         }
    //                     }
    //                 }
    //                 if pc_w_flank[0] != pc_w_flank[1] && pawn_king_b { return 36; }
    //             }
    //             if qc_w + qc_b == 1 {
    //                 sf = 37 + 3 * (if qc_w == 1 { bc_b + nc_b } else { bc_w + nc_w });
    //             } else {
    //                 sf = sf.min(36 + 7 * pc_w);
    //             }
    //         }
    //     }
    //     return sf;
    // }

    // fn tempo(board: &Board) -> i32 {
    //     return 28 * if board.white_to_move { 1 } else { -1 }
    // }
}