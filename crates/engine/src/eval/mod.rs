use crate::{board::{coord::Coord, Board}, move_gen::magics::MagicBitBoards, color::Color, precomp::PrecomputedData, prelude::BitBoard};

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
pub mod macros;


pub struct Evaluation<'a> {
    pub board: &'a Board,
    pub precomp: &'a PrecomputedData,
    pub magics: &'a MagicBitBoards,

    pin_rays: [BitBoard; 2],
    king_ring: [BitBoard; 2],
}

impl<'a> Evaluation<'a> {
    pub fn new(
        board: &'a Board,
        precomp: &'a PrecomputedData,
        magics: &'a MagicBitBoards,
    ) -> Self {
        Self {
            board,
            precomp,
            magics,

            pin_rays: [BitBoard(0); 2],
            king_ring: [BitBoard(0); 2],
        }
    }

    pub fn init<W: Color, B: Color>(&mut self) {
        self.pin_rays[W::index()] = self.pin_rays::<W, B>();
        self.pin_rays[B::index()] = self.pin_rays::<B, W>();
        self.king_ring[W::index()] = self.king_ring::<W, B>(false);
        self.king_ring[B::index()] = self.king_ring::<B, W>(false);
    }

    /// Evaluation function adapted from the [Stockfish Evaluation Guide](https://hxim.github.io/Stockfish-Evaluation-Guide/).
    ///
    /// Parameterized by <Friendly, Enemy>
    ///
    /// Notes to self: 
    ///
    /// - Capital letters represent white pieces.
    /// - The ranks are inverted, meaning when it says `y + 1`, it should be `Color::down()`.
    /// - Castling rights go in order: [white kingside, white queenside, black kingside, black
    /// queenside]
    pub fn evaluate<W: Color, B: Color>(&mut self) -> i32 {
        self.init::<W, B>();

        let mg = self.middle_game_eval() as f32;
        let mut eg = self.end_game_eval() as f32;
        let p = self.phase() as f32;
        let rule50 = self.rule50() as f32;

        eg = eg * self.scale_factor(eg as i32) as f32 / 64.0;
        let mut v = (((mg * p + ((eg * (128.0 - p)).trunc())) / 128.0).trunc());
        v = ((v / 16.0).trunc()) * 16.0;
        v += self.tempo() as f32;
        v = (v * (100.0 - rule50) / 100.0).trunc();

        v as i32
    }
}


impl<'a> Evaluation<'a> {
    fn middle_game_eval(&self) -> i32 {
        todo!()
    }

    fn end_game_eval(&self) -> i32 {
        todo!()
    }


    fn phase(&self) -> i32 {
        todo!();
    }

    fn rule50(&self) -> i32 {
        todo!()
    }

    fn scale_factor(&self, v: i32) -> i32 {
        todo!()
    }

    fn tempo(&self) -> i32 {
        todo!()
    }
}


pub(super) mod test_prelude {
    pub use crate::precomp::PrecomputedData;
    pub use crate::board::Board;
    pub use crate::board::zobrist::Zobrist;
    pub use crate::color::Color;
    pub use crate::move_gen::magics::MagicBitBoards;
    pub use crate::eval::Evaluation;
    pub use crate::assert_eval;
    pub use crate::sum_sqrs;
    pub use proc_macro_utils::evaluation_test;
}
