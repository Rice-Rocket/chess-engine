use std::time::Instant;

use crate::{board::{moves::Move, zobrist::Zobrist, Board}, eval::Evaluation, move_gen::{magics::MagicBitBoards, move_generator::MoveGenerator}, precomp::Precomputed};

use self::options::SearchOptions;

pub mod options;


pub struct Searcher<'a> {
    pub in_search: bool,
    best_move: Option<Move>,
    backup_move: Move,
    eval: Option<Evaluation<'a>>,
    start_time: Instant,
    opts: SearchOptions,
}

impl<'a> Searcher<'a> {
    pub fn new() -> Self {
        Self {
            best_move: None,
            backup_move: Move::NULL,
            in_search: false,
            eval: None,
            start_time: Instant::now(),
            opts: SearchOptions::default(),
        }
    }

    /// Starts searching for the best move in the position depending on whose turn it is to move. 
    ///
    /// Expects that `movegen.generate_moves()` has been called beforehand.
    pub fn begin_search(&mut self, opts: SearchOptions, board: Board, precomp: &Precomputed, magics: &MagicBitBoards, zobrist: &Zobrist, movegen: MoveGenerator) {
        self.opts = opts;
        self.init();

        let moves = &movegen.moves;
        self.backup_move = moves[0];
        self.best_move = Some(moves[0]);
        self.in_search = true;
    }

    fn init(&mut self) {
        self.best_move = None;
        self.in_search = true;
        self.start_time = Instant::now();
    }

    pub fn best_move(&mut self) -> Option<Move> {
        if let Some(time) = self.opts.movetime {
            if time <= Instant::now().duration_since(self.start_time).as_millis() as u32 {
                self.in_search = false;
            }
        }

        if self.in_search {
            None
        } else {
            self.best_move
        }
    }

    pub fn abort(&mut self) {
        
    }

    // TODO: Exit search here
    pub fn force_best_move(&mut self) -> Move {
        self.abort();
        self.best_move.unwrap_or(self.backup_move)
    }
}

impl<'a> Default for Searcher<'a> {
    fn default() -> Self {
        Self::new()
    }
}
