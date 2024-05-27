use std::time::Instant;

use crate::{board::{moves::Move, piece::Piece, zobrist::Zobrist, Board}, color::{Black, White}, eval::Evaluation, move_gen::{magics::MagicBitBoards, move_generator::MoveGenerator}, precomp::Precomputed};

use self::{diagnostics::SearchDiagnostics, options::SearchOptions, ordering::MoveOrdering, repetition::RepetitionTable, transpositions::{TranspositionNodeType, TranspositionTable}};

pub mod options;
pub mod diagnostics;
pub mod repetition;
pub mod transpositions;
pub mod ordering;

pub struct Searcher<'a> {
    pub diagnostics: SearchDiagnostics,
    pub in_search: bool,
    pub transposition_table: TranspositionTable<4_194_304>, // 64 MB: 4_194_304
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
    const MAX_EXTENSIONS: u8 = 16;

    pub fn new() -> Self {
        Self {
            diagnostics: SearchDiagnostics::default(),
            transposition_table: TranspositionTable::new(),
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
    /// Assumes that that the position has valid moves (not stalemate or checkmate).
    pub fn begin_search(
        &mut self,
        opts: SearchOptions,
        board: &mut Board,
        precomp: &Precomputed,
        magics: &MagicBitBoards,
        zobrist: &Zobrist,
        movegen: &mut MoveGenerator,
    ) {
        self.opts = opts;
        self.init();

        movegen.generate_moves(board, precomp, magics, false);

        let moves = movegen.moves.clone();
        self.backup_move = moves[0];

        let mut repetition_table = RepetitionTable::new(board);
        let mut ordering = MoveOrdering::new();

        // Iterative Deepening
        for depth in 1..=255 {
            let mut best_move_this_iter = None;
            let mut alpha = Self::NEGATIVE_INFINITY;
            let mut beta = Self::POSITIVE_INFINITY;
            let mut eval_bound = TranspositionNodeType::UpperBound;

            let zobrist_key = board.current_state.zobrist_key;
            if let Some(tt_eval) = self.transposition_table.lookup(zobrist_key, depth, 0, alpha, beta) {
                let entry = self.transposition_table.get(zobrist_key);
                best_move_this_iter = Some(entry.m);
                alpha = entry.eval;

                self.best_move = best_move_this_iter;
                self.diagnostics.depth_searched = depth;
                self.diagnostics.evaluation = alpha;

                // Exit if mate was found
                if alpha.abs() > Self::IMMEDIATE_MATE_SCORE - 1000 
                && Self::IMMEDIATE_MATE_SCORE - alpha.abs() <= depth as i32 {
                    break;
                }

                continue;
            }

            // Order moves and ensure the best move from the previous search is considered first.
            // This way, partial searches can be used as they will either agree on the best move, 
            // or they will have found a better move.
            let ordered_moves = ordering.order(
                if let Some(m) = self.best_move { m } else { Move::NULL },
                &moves,
                board,
                movegen.enemy_attack_map,
                movegen.enemy_pawn_attack_map,
                0,
            );

            for m in ordered_moves {
                let captured_ptype = board.square[m.target()].piece_type();
                let is_capture = captured_ptype != Piece::NONE;

                board.make_move(m, true, zobrist);

                // let extension = if board.in_check(magics, precomp) { 1 } else { 0 };
                let extension = 0;

                let eval = -self.search(1, depth - 1 + extension, -beta, -alpha, extension, board, &mut ordering, &mut repetition_table, m, is_capture, precomp, magics, zobrist, movegen);
                board.unmake_move(m, true);

                if !self.in_search {
                    break;
                }

                if eval > alpha {
                    best_move_this_iter = Some(m);
                    alpha = eval;
                    eval_bound = TranspositionNodeType::Exact;
                }
            }

            if let Some(m) = best_move_this_iter {
                self.transposition_table.store(zobrist_key, depth, 0, alpha, eval_bound, m);
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
        depth: u8,
        depth_remaining: u8,
        mut alpha: i32,
        mut beta: i32,
        n_extensions: u8,
        board: &mut Board,
        ordering: &mut MoveOrdering,
        repetition_table: &mut RepetitionTable,
        prev_move: Move,
        prev_move_was_capture: bool,
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
        if board.current_state.fifty_move_counter >= 100 || repetition_table.contains(board.current_state.zobrist_key) {
            return 0;
        }

        // If a faster mating sequence is found, skip this position
        alpha = alpha.max(-Self::IMMEDIATE_MATE_SCORE + depth as i32);
        beta = beta.min(Self::IMMEDIATE_MATE_SCORE - depth as i32);
        if alpha >= beta {
            return alpha;
        }

        // Check if we've already come across this position, and if so retrieve the evaluation
        let zobrist_key = board.current_state.zobrist_key;
        if let Some(tt_eval) = self.transposition_table.lookup(zobrist_key, depth_remaining, depth, alpha, beta) {
            return tt_eval;
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

        // Update repetition table
        let was_pawn_move = board.square[prev_move.target()].piece_type() == Piece::PAWN;
        repetition_table.push(board.current_state.zobrist_key, prev_move_was_capture || was_pawn_move);

        let mut best_move = Move::NULL;
        let mut eval_bound = TranspositionNodeType::UpperBound;

        let ordered_moves = ordering.order(
            self.transposition_table.get_stored_move(zobrist_key),
            &moves,
            board,
            movegen.enemy_attack_map,
            movegen.enemy_pawn_attack_map,
            depth,
        );

        for m in ordered_moves {
            let captured_ptype = board.square[m.target()].piece_type();
            let is_capture = captured_ptype != Piece::NONE;

            board.make_move(m, true, zobrist);

            // If the move is a check, extend the search depth
            // let extension = if n_extensions < Self::MAX_EXTENSIONS {
            //     if board.in_check(magics, precomp) { 1 } else { 0 }
            // } else { 0 };
            let extension = 0;

            let eval = -self.search(depth + 1, depth_remaining - 1 + extension, -beta, -alpha, n_extensions + extension, board, ordering, repetition_table, m, is_capture, precomp, magics, zobrist, movegen);
            board.unmake_move(m, true);

            if !self.in_search {
                return 0;
            }

            // Beta cutoff / Fail high
            if eval >= beta {
                self.transposition_table.store(zobrist_key, depth_remaining, depth, beta, TranspositionNodeType::LowerBound, m);

                if !is_capture {
                    if (depth as usize) < MoveOrdering::MAX_KILLER_MOVE_DEPTH {
                        ordering.killers[depth as usize].add(m);
                    }
                    let history_score = depth_remaining as i32 * depth_remaining as i32;
                    ordering.history[board.move_color_idx][m.start()][m.target()] += history_score;
                }

                repetition_table.pop();
                return beta;
            }

            // Found a new best move
            if eval > alpha {
                alpha = eval;
                best_move = m;
                eval_bound = TranspositionNodeType::Exact;
            }
        }

        repetition_table.pop();
        self.transposition_table.store(zobrist_key, depth_remaining, depth, alpha, eval_bound, best_move);

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
