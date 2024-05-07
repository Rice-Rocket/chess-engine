use crate::board::Board;

mod state;
mod attack;
mod utils;
mod imbalance;
mod king;
mod material;
mod mobility;
mod passed_pawns;
mod pawns;
mod pieces;
mod space;
mod threats;
mod winnable;


impl Board {
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
