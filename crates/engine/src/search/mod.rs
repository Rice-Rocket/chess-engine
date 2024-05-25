use crate::{board::{moves::Move, zobrist::Zobrist, Board}, eval::Evaluation, move_gen::{magics::MagicBitBoards, move_generator::MoveGenerator}, precomp::Precomputed};

#[derive(Default)]
pub struct Searcher<'a> {
    pub in_search: bool,
    best_move: Move,
    eval: Option<Evaluation<'a>>,
}

impl<'a> Searcher<'a> {
    pub fn new() -> Self {
        Self {
            best_move: Move::NULL,
            in_search: false,
            eval: None,
        }
    }

    /// Starts searching for the best move in the position depending on whose turn it is to move. 
    ///
    /// Expects that `movegen.generate_moves()` has been called beforehand.
    pub fn begin_search(&mut self, board: Board, precomp: &Precomputed, magics: &MagicBitBoards, zobrist: &Zobrist, movegen: MoveGenerator) {
        self.init();

        let moves = &movegen.moves;
        self.best_move = moves[0];
        self.in_search = false;
    }

    fn init(&mut self) {
        self.best_move = Move::NULL;
        self.in_search = true;
    }

    pub fn best_move(&self) -> Option<Move> {
        if self.in_search {
            None
        } else {
            Some(self.best_move)
        }
    }

    // TODO: Exit search here
    pub fn force_best_move(&self) -> Move {
        self.best_move
    }
}
