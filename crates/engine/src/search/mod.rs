use std::time::Instant;

use crate::{board::{moves::Move, zobrist::Zobrist, Board}, color::{Black, White}, eval::Evaluation, move_gen::{magics::MagicBitBoards, move_generator::MoveGenerator}, precomp::Precomputed};

use self::{diagnostics::SearchDiagnostics, options::SearchOptions};

pub mod options;
pub mod diagnostics;


pub struct Searcher<'a> {
    pub diagnostics: SearchDiagnostics,
    pub in_search: bool,
    best_move: Option<Move>,
    backup_move: Move,
    eval: Option<Evaluation<'a>>,
    start_time: Instant,
    opts: SearchOptions,
}

impl<'a> Searcher<'a> {
    const IMMEDIATE_MATE_SCORE: i32 = 1000000;
    const POSITIVE_INFINITY: i32 = i32::MAX;
    const NEGATIVE_INFINITY: i32 = -Self::POSITIVE_INFINITY;

    pub fn new() -> Self {
        Self {
            diagnostics: SearchDiagnostics::default(),
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
    /// Assumes that `movegen.generate_moves()` has been called beforehand and that the position
    /// has valid moves (not stalemate or checkmate).
    pub fn begin_search(
        &mut self,
        opts: SearchOptions,
        mut board: Board,
        precomp: &Precomputed,
        magics: &MagicBitBoards,
        zobrist: &Zobrist,
        mut movegen: MoveGenerator,
    ) {
        self.opts = opts;
        self.init();

        let moves = movegen.moves.clone();
        self.backup_move = moves[0];

        // Iterative Deepening
        for depth in 1..=256 {
            let mut best_move_this_iter = None;
            let mut alpha = Self::NEGATIVE_INFINITY;
            let mut beta = Self::POSITIVE_INFINITY;

            // Ensure the best move from the previous search is considered first.
            // This way, partial searches can be used as they will either agree on the best move, 
            // or they will have found a better move.
            let ordered_moves = if let Some(m) = self.best_move {
                let mut m_clone = moves.clone();
                let m_index = moves.iter().position(|mov| *mov == m).unwrap();
                m_clone.swap(0, m_index);
                m_clone
            } else {
                moves.clone()
            };

            for m in ordered_moves {
                board.make_move(m, true, zobrist);
                let eval = -self.search(1, depth - 1, -beta, -alpha, &mut board, precomp, magics, zobrist, &mut movegen);
                board.unmake_move(m, true);

                if !self.in_search {
                    break;
                }

                if eval > alpha {
                    best_move_this_iter = Some(m);
                    alpha = eval;
                }
            }

            // Search was cancelled
            if !self.in_search {
                // Even if we end with a partial search, since the best move was considered first
                // we can trust the best move from the partial search is equal or better.
                if let Some(m) = best_move_this_iter {
                    self.best_move = Some(m);
                    self.diagnostics.evaluation = alpha;
                } 

                if self.best_move.is_none() {
                    self.best_move = Some(self.backup_move);
                }

                break;
            // Search not cancelled
            } else {
                self.best_move = best_move_this_iter;
                self.diagnostics.depth_searched = depth;
                self.diagnostics.evaluation = alpha;

                // Exit if mate was found
                if alpha.abs() > Self::IMMEDIATE_MATE_SCORE - 1000 
                && Self::IMMEDIATE_MATE_SCORE - alpha.abs() <= depth as i32 {
                    break;
                }
            }
        }

        self.in_search = false;
    }

    #[allow(clippy::too_many_arguments)]
    fn search(
        &mut self,
        depth: u16,
        depth_remaining: u16,
        mut alpha: i32,
        mut beta: i32,
        board: &mut Board,
        precomp: &Precomputed,
        magics: &MagicBitBoards,
        zobrist: &Zobrist,
        movegen: &mut MoveGenerator,
    ) -> i32 {
        if let Some(time) = self.opts.movetime {
            if time <= Instant::now().duration_since(self.start_time).as_millis() as u32 {
                self.in_search = false;
                return 0;
            }
        }

        // Consider draw cases
        // TODO: Also consider three-fold repetition inside the search
        if board.current_state.fifty_move_counter >= 100 {
            return 0;
        }

        // If a faster mating sequence is found, skip this position
        alpha = alpha.max(-Self::IMMEDIATE_MATE_SCORE + depth as i32);
        beta = beta.min(Self::IMMEDIATE_MATE_SCORE - depth as i32);
        if alpha >= beta {
            return alpha;
        }

        // Once we hit a leaf node, perform static evaluation of the position
        if depth_remaining == 0 {
            let mut eval = Evaluation::new(board, precomp, magics);
            return eval.evaluate::<White, Black>() * if board.white_to_move { 1 } else { -1 };
        }

        movegen.generate_moves(board, precomp, magics, false);
        let moves = movegen.moves.clone();

        // Consider checkmate and stalemate cases
        if moves.is_empty() {
            if movegen.in_check() {
                return -(Self::IMMEDIATE_MATE_SCORE - depth as i32);
            } else {
                return 0;
            }
        }

        let mut best_move = Move::NULL;
        for m in moves {
            board.make_move(m, true, zobrist);
            let eval = -self.search(depth + 1, depth_remaining - 1, -beta, -alpha, board, precomp, magics, zobrist, movegen);
            board.unmake_move(m, true);

            if !self.in_search {
                return 0;
            }

            // Beta cutoff / Fail high
            if eval >= beta {
                return beta;
            }

            // Found a new best move
            if eval > alpha {
                alpha = eval;
                best_move = m;
            }
        }

        alpha
    }

    fn init(&mut self) {
        self.best_move = None;
        self.in_search = true;
        self.start_time = Instant::now();
        self.diagnostics = SearchDiagnostics::default();
    }

    pub fn best_move(&mut self) -> Option<Move> {
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
