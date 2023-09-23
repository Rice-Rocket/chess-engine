use crate::board::board::Board;

use super::perspective::Perspective;
use super::material::*;


pub struct Evaluation {}

impl Evaluation {
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
}