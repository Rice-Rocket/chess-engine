use crate::{board::{zobrist::Zobrist, Board}, eval::Evaluation, move_gen::{magics::MagicBitBoards, move_generator::MoveGenerator}, precomp::Precomputed};

pub struct Searcher<'a> {
    board: &'a Board,
    precomp: &'a Precomputed,
    magics: &'a MagicBitBoards,
    movegen: &'a MoveGenerator,
    zobrist: &'a Zobrist,
    eval: Evaluation<'a>,
}

impl<'a> Searcher<'a> {
    pub fn new(board: &'a Board, precomp: &'a Precomputed, magics: &'a MagicBitBoards, zobrist: &'a Zobrist, movegen: &'a MoveGenerator) -> Self {
        Self {
            board,
            precomp,
            magics,
            movegen,
            zobrist,
            eval: Evaluation::new(board, precomp, magics),
        }
    }

    pub fn search(&mut self) {

    }
}
