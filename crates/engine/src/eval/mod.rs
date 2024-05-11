use crate::board::Board;

pub mod state;
pub mod attack;
pub mod utils;
pub mod imbalance;
pub mod king;
pub mod material;
pub mod mobility;
pub mod passed_pawns;
pub mod pawns;
pub mod pieces;
pub mod space;
pub mod threats;
pub mod winnable;


impl Board {
    /// Evaluation function adapted from the [Stockfish Evaluation Guide](https://hxim.github.io/Stockfish-Evaluation-Guide/).
    ///
    /// Notes to self: 
    ///
    /// - Capital letters represent white pieces.
    /// - The ranks are inverted, meaning when it says `y + 1`, it should be `Color::down()`.
    pub fn evaluate(&self) -> i32 {
        let mg = middle_game_eval(self) as f32;
        let mut eg = end_game_eval(self) as f32;
        let p = phase(self) as f32;
        let rule50 = rule50(self) as f32;

        eg = eg * scale_factor(self, eg as i32) as f32 / 64.0;
        let mut v = (((mg * p + ((eg * (128.0 - p)).trunc())) / 128.0).trunc());
        v = ((v / 16.0).trunc()) * 16.0;
        v += tempo(self) as f32;
        v = (v * (100.0 - rule50) / 100.0).trunc();

        v as i32
    }
}


fn middle_game_eval(board: &Board) -> i32 {
    todo!()
}

fn end_game_eval(board: &Board) -> i32 {
    todo!()
}


fn phase(board: &Board) -> i32 {
    todo!();
}

fn rule50(board: &Board) -> i32 {
    todo!()
}

fn scale_factor(board: &Board, eg: i32) -> i32 {
    todo!()
}

fn tempo(board: &Board) -> i32 {
    todo!()
}
