use crate::board::{board::Board, piece::Piece, coord::Coord};
use crate::move_gen::magics::MagicBitBoards;
use crate::move_gen::move_generator::MoveGenerator;
use crate::move_gen::precomp_move_data::PrecomputedMoveData;

use super::perspective::Perspective;
use super::pos::PositionEvaluation;
use super::{
    helpers::*,
    material::*,
    passed_pawns::*,
    mobility::*,
};



pub struct Evaluation {}

impl Evaluation {
    /// Performs evaluation of the board
    /// A positive value means a better position for the player to move
    pub fn evaluate(board: &Board, move_gen: &MoveGenerator, precomp_move_data: &PrecomputedMoveData, magic: &MagicBitBoards) -> i32 {
        let pos = PositionEvaluation::new(&board, &move_gen, &precomp_move_data, &magic);

        let mg = Self::midgame_eval(&pos, false);
        let eg = Self::endgame_eval(&pos, false);
        let p = Self::phase(&pos);
        let rule50 = Self::rule50(&pos);

        // eg = eg * Self::scale_factor(&pos, Some(eg)) / 64;
        let mut eval = (mg * p + (eg * (128 - p))) / 128;

        eval += Self::tempo(&pos);
        eval = (eval * (100 - rule50)) / 100;

        let perspective = if board.white_to_move { 1 } else { -1 };
        return eval * perspective;
    }

    /// Evaluates position during opening and middle game stages
    fn midgame_eval(pos: &PositionEvaluation, _nowinnable: bool) -> i32 {
        let mut eval = 0;

        eval += pos.material_data().white_material.0 - pos.material_data().black_material.0;
        eval += pos.material_data().white_psqt_bonuses.0 - pos.material_data().black_psqt_bonuses.0;
        eval += pos.material_data().imbalance_total;
        eval += mobility(pos, Perspective::White, true) - mobility(pos, Perspective::Black, true);

        return eval;
    }

    fn endgame_eval(pos: &PositionEvaluation, _nowinnable: bool) -> i32 {
        let mut eval = 0;

        eval += pos.material_data().white_material.1 - pos.material_data().black_material.1;
        eval += pos.material_data().white_psqt_bonuses.1 - pos.material_data().black_psqt_bonuses.1;
        eval += pos.material_data().imbalance_total;
        eval += mobility(pos, Perspective::White, false) - mobility(pos, Perspective::Black, false);

        return eval;
    }
    
    // For tapered evaluation
    const PHASE_LIMIT_MG: i32 = 15258;
    const PHASE_LIMIT_EG: i32 = 3915;
    pub fn phase(pos: &PositionEvaluation) -> i32 {
        let mut npm = pos.material_data().white_non_pawn_material + pos.material_data().black_non_pawn_material;
        npm = Self::PHASE_LIMIT_EG.max(Self::PHASE_LIMIT_MG.min(npm));
        return (((npm - Self::PHASE_LIMIT_EG) * 128) as f32 / (Self::PHASE_LIMIT_MG - Self::PHASE_LIMIT_EG) as f32) as i32;
    }

    pub fn rule50(pos: &PositionEvaluation) -> i32 {
        pos.board.current_state.fifty_move_counter as i32
    }

    pub fn scale_factor(pos: &PositionEvaluation, eg: Option<i32>) -> i32 {
        let mut sf = 64;
        let eg_eval = match eg {
            Some(eval) => eval,
            None => Self::endgame_eval(pos, false)
        };
        let per = if eg_eval > 0 { Perspective::White } else { Perspective::Black };
        let (pc_w, pc_b) = (pos.material_data().pcount(per, Piece::PAWN), pos.material_data().pcount(per.other(), Piece::PAWN));
        let (qc_w, qc_b) = (pos.material_data().pcount(per, Piece::QUEEN), pos.material_data().pcount(per.other(), Piece::QUEEN));
        let (bc_w, bc_b) = (pos.material_data().pcount(per, Piece::BISHOP), pos.material_data().pcount(per.other(), Piece::BISHOP));
        let (nc_w, nc_b) = (pos.material_data().pcount(per, Piece::KNIGHT), pos.material_data().pcount(per.other(), Piece::KNIGHT));
        let (npm_w, npm_b) = (pos.material_data().get_non_pawn_material(per), pos.material_data().get_non_pawn_material(per));

        const BISHOP_VALUE_MG: i32 = PIECE_VALUE_BONUSES_MG[2];
        const ROOK_VALUE_MG: i32 = PIECE_VALUE_BONUSES_MG[3];

        if pc_w == 0 && npm_w - npm_b <= BISHOP_VALUE_MG {
            sf = if npm_w < ROOK_VALUE_MG { 0 } else { if npm_b <= BISHOP_VALUE_MG { 4 } else { 14 } };
        };
        if sf == 64 {
            let ob = opposite_bishops(pos);
            if ob && npm_w == BISHOP_VALUE_MG && npm_b == BISHOP_VALUE_MG {
                sf = 22 + 4 * candidate_passed(pos, per);
            } else if ob {
                sf = 22 + 3 * pos.material_data().pcount_total as i32;
            } else {
                if npm_w == ROOK_VALUE_MG && npm_b == ROOK_VALUE_MG && pc_w - pc_b <= 1 {
                    let (mut pawn_king_b, mut pc_w_flank) = (false, [0, 0]);
                    for sqr in Coord::iterate_squares() {
                        if pos.square(sqr) == per.friendly_piece(Piece::PAWN) {
                            pc_w_flank[if sqr.file() < 4 { 1 } else { 0 }] = 1;
                        }
                        if pos.square(sqr) == per.friendly_piece(Piece::KING) {
                            for x in -1..=1 {
                                for y in -1..=1 {
                                    if pos.square(Coord::new(sqr.file() + x, sqr.rank() + y)).piece_type() == Piece::BLACK_PAWN {
                                        pawn_king_b = true;
                                    }
                                }
                            }
                        }
                    }
                    if pc_w_flank[0] != pc_w_flank[1] && pawn_king_b { return 36; }
                }
                if qc_w + qc_b == 1 {
                    sf = 37 + 3 * (if qc_w == 1 { bc_b + nc_b } else { bc_w + nc_w }) as i32;
                } else {
                    sf = sf.min(36 + 7 * pc_w as i32);
                }
            }
        }
        return sf;
    }

    fn tempo(pos: &PositionEvaluation) -> i32 {
        return 28 * if pos.board.white_to_move { 1 } else { -1 }
    }
}