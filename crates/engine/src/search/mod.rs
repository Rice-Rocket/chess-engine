use std::time::Instant;

use crate::{board::{coord::Coord, moves::Move, piece::Piece, zobrist::Zobrist, Board}, color::{Black, White}, eval::Evaluation, move_gen::{magics::Magics, move_generator::MoveGenerator}, precomp::Precomputed};

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
    const ASPIRATION_WINDOW_SIZE: i32 = 40;

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
        zobrist: &Zobrist,
        movegen: &mut MoveGenerator,
    ) {
        self.opts = opts;
        self.init();

        let moves = movegen.generate_moves(board, false);
        self.backup_move = moves[0];

        let mut repetition_table = RepetitionTable::new(board);
        let mut ordering = MoveOrdering::new();
        let mut score: i32;
        let mut alpha = Self::NEGATIVE_INFINITY;
        let mut beta = Self::POSITIVE_INFINITY;
        let mut left_window = Self::ASPIRATION_WINDOW_SIZE;
        let mut right_window = Self::ASPIRATION_WINDOW_SIZE;
        let mut best_move_this_iter = None;
        let mut depth = 1;

        // Iterative Deepening + Aspiration Windows
        while depth < 255 {
            (score, best_move_this_iter) = self.search_root(
                depth,
                alpha,
                beta,
                &moves,
                board,
                &mut ordering,
                &mut repetition_table,
                zobrist,
                movegen,
            );

            // Search was cancelled
            if !self.in_search {
                // Even if we end with a partial search, since the best move was considered first
                // we can trust the best move from the partial search is equal or better.
                if let Some(m) = best_move_this_iter {
                    self.best_move = Some(m);
                    self.diagnostics.evaluation = score;
                } 

                if self.best_move.is_none() {
                    self.best_move = Some(self.backup_move);
                }

                break;
            }

            // If the score is outside the current aspiration window, search again with wider
            // window
            if score <= alpha {
                alpha -= left_window;
                left_window *= 2;
                best_move_this_iter = None;
                continue;
            } else if score >= beta {
                beta += right_window;
                right_window *= 2;
                best_move_this_iter = None;
                continue;
            }

            self.best_move = best_move_this_iter;
            self.diagnostics.depth_searched = depth;
            self.diagnostics.evaluation = score;

            best_move_this_iter = None;
            left_window = Self::ASPIRATION_WINDOW_SIZE;
            right_window = Self::ASPIRATION_WINDOW_SIZE;
            alpha = score - left_window;
            beta = score + right_window;

            // Exit if mate was found
            if score.abs() > Self::IMMEDIATE_MATE_SCORE - 1000 
            && Self::IMMEDIATE_MATE_SCORE - score.abs() <= depth as i32 {
                break;
            }

            depth += 1;
        }

        self.in_search = false;
    }

    #[allow(clippy::too_many_arguments)]
    fn search_root(
        &mut self,
        depth: u8,
        mut alpha: i32,
        mut beta: i32,
        moves: &[Move],
        board: &mut Board,
        ordering: &mut MoveOrdering,
        repetition_table: &mut RepetitionTable,
        zobrist: &Zobrist,
        movegen: &mut MoveGenerator,
    ) -> (i32, Option<Move>) {
        // If this position was already searched and is stored in the transposition table,
        // retrieve the value and immediately search at the next depth.
        let zobrist_key = board.current_state.zobrist_key;
        if let Some(tt_eval) = self.transposition_table.lookup(zobrist_key, depth, 0, alpha, beta) {
            let entry = self.transposition_table.get(zobrist_key);
            return (tt_eval, Some(entry.m));
        }

        // Order moves and ensure the best move from the previous search is considered first.
        // This way, partial searches can be used as they will either agree on the best move, 
        // or they will have found a better move.
        let ordered_moves = ordering.order(
            if let Some(m) = self.best_move { m } else { Move::NULL },
            moves,
            board,
            movegen.enemy_attack_map,
            movegen.enemy_pawn_attack_map,
            0,
            false,
        );

        let mut best_move = None;
        let mut best_score = Self::NEGATIVE_INFINITY;
        let mut eval_bound = TranspositionNodeType::UpperBound;

        for (i, m) in ordered_moves.into_iter().enumerate() {
            let captured_ptype = board.square[m.target()].piece_type();
            let is_capture = captured_ptype != Piece::NONE;

            board.make_move(m, true, zobrist);

            // Check extensions
            let extension = if board.in_check() { 1 } else { 0 };

            let mut eval = 0;
            let mut needs_full_search = true;

            // Late move reductions: reduce search depth when searching later moves because they
            // are likely to be bad.
            if extension == 0 && depth >= 3 && i >= 3 && !is_capture {
                eval = -self.search(1, depth - 2, -alpha - 1, -alpha, extension, board, ordering, repetition_table, m, is_capture, zobrist, movegen);
                needs_full_search = eval > alpha;
            }

            if needs_full_search {
                eval = -self.search(1, depth - 1 + extension, -beta, -alpha, extension, board, ordering, repetition_table, m, is_capture, zobrist, movegen);
            }

            board.unmake_move(m, true);

            if !self.in_search {
                break;
            }

            // Found a new best move
            if eval > best_score {
                best_score = eval;

                if eval > alpha {
                    best_move = Some(m);
                    alpha = eval;
                    eval_bound = TranspositionNodeType::Exact;
                }
            }
        }

        // Store this position in the transposition table
        if let Some(m) = best_move {
            self.transposition_table.store(zobrist_key, depth, 0, best_score, eval_bound, m);
        }

        (best_score, best_move)
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
        zobrist: &Zobrist,
        movegen: &mut MoveGenerator,
    ) -> i32 {
        // Abort search if we've run out of time
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

        // Check if we've already come across this position. If so, retrieve the evaluation and
        // continue to the next position
        let zobrist_key = board.current_state.zobrist_key;
        if let Some(tt_eval) = self.transposition_table.lookup(zobrist_key, depth_remaining, depth, alpha, beta) {
            return tt_eval;
        }

        // Once we hit a leaf node, perform static evaluation of the position
        if depth_remaining == 0 {
            return Self::quiescence_search(alpha, beta, board, ordering, zobrist, movegen);
        }

        let moves = movegen.generate_moves(board, false);

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
        let mut best_score = Self::NEGATIVE_INFINITY;
        let mut eval_bound = TranspositionNodeType::UpperBound;

        // Order moves
        let ordered_moves = ordering.order(
            self.transposition_table.get_stored_move(zobrist_key),
            &moves,
            board,
            movegen.enemy_attack_map,
            movegen.enemy_pawn_attack_map,
            depth,
            false,
        );

        for (i, m) in ordered_moves.into_iter().enumerate() {
            let captured_ptype = board.square[m.target()].piece_type();
            let is_capture = captured_ptype != Piece::NONE;

            board.make_move(m, true, zobrist);

            // If the move is a check, extend the search depth
            let extension = if n_extensions < Self::MAX_EXTENSIONS {
                if board.in_check() { 1 } else { 0 }
            } else { 0 };

            let mut eval = 0;
            let mut needs_full_search = true;

            // Late move reductions: reduce search depth when searching later moves because they
            // are likely to be bad.
            if extension == 0 && depth_remaining >= 3 && i >= 3 && !is_capture {
                eval = -self.search(depth + 1, depth_remaining - 2, -alpha - 1, -alpha, n_extensions, board, ordering, repetition_table, m, is_capture, zobrist, movegen);
                needs_full_search = eval > alpha;
            }

            if needs_full_search {
                eval = -self.search(depth + 1, depth_remaining - 1 + extension, -beta, -alpha, n_extensions + extension, board, ordering, repetition_table, m, is_capture, zobrist, movegen);
            }

            board.unmake_move(m, true);

            if !self.in_search {
                return 0;
            }

            // Found a new best move
            if eval > best_score {
                best_score = eval;

                if eval > alpha {
                    alpha = eval;
                    best_move = m;
                    eval_bound = TranspositionNodeType::Exact;

                    // Beta cutoff / Fail high
                    if eval >= beta {
                        self.transposition_table.store(zobrist_key, depth_remaining, depth, beta, TranspositionNodeType::LowerBound, m);

                        // Update killer moves and history heuristic for move ordering
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
                }
            }
        }

        repetition_table.pop();

        // Store this evaluation in the transposition table
        self.transposition_table.store(zobrist_key, depth_remaining, depth, best_score, eval_bound, best_move);

        best_score
    }

    fn quiescence_search(
        mut alpha: i32,
        mut beta: i32,
        board: &mut Board,
        ordering: &mut MoveOrdering,
        zobrist: &Zobrist,
        movegen: &mut MoveGenerator,
    ) -> i32 {
        let mut eval = Evaluation::new(board).evaluate::<White, Black>() * if board.white_to_move { 1 } else { -1 };

        // Check for beta cutoff
        if eval >= beta {
            return beta;
        }

        // TODO: Delta pruning

        if eval > alpha {
            alpha = eval;
        }

        let moves = movegen.generate_moves(board, true);

        // Order moves
        let ordered_moves = ordering.order(
            Move::NULL,
            &moves,
            board,
            movegen.enemy_attack_map,
            movegen.enemy_pawn_attack_map,
            0,
            true,
        );

        for (i, m) in ordered_moves.into_iter().enumerate() {
            board.make_move(m, true, zobrist);
            eval = -Self::quiescence_search(-beta, -alpha, board, ordering, zobrist, movegen);
            board.unmake_move(m, true);

            // Found a new best move
            if eval > alpha {
                alpha = eval;

                // Beta cutoff / Fail high
                if eval >= beta {
                    return beta;
                }
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
