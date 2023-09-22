use std::time::SystemTime;

use bevy::prelude::*;
use crate::{board::{moves::Move, board::Board, zobrist::Zobrist, piece::Piece}, move_gen::{move_generator::MoveGenerator, precomp_move_data::PrecomputedMoveData, bitboard::utils::BitBoardUtils, magics::MagicBitBoards}, ai::{ai_player::{BeginSearch, SearchComplete, AIVersion}, stats::SearchStatistics}};

use super::{super::evaluation::eval::Evaluation, transpositions::{TranspositionTable, EvaluationType}, move_ordering::MoveOrdering};

#[derive(Resource)]
pub struct Searcher {
    pub current_depth: i32,
    pub best_move_so_far: Move,
    pub best_eval_so_far: i32,
    pub max_think_time_ms: u32,
    best_move_this_iter: Move,
    best_eval_this_iter: i32,

    repetition_table: Vec<u64>,
    transposition_table: TranspositionTable,
    move_ordering: MoveOrdering,

    positions_evaled: u32,
    num_mates: i32,
    num_cutoffs: i32,
    num_transpositions: i32,
    has_searched_one_move: bool,
    search_cancelled: bool,

    search_iteration_time: SystemTime,
    search_total_time: SystemTime,
    current_iter_depth: i32,
    move_is_from_partial_search: bool,
}

impl Searcher {
    const MATE_SCORE: i32 = 100000;
    const POS_INF: i32 = 9999999;
    const NEG_INF: i32 = -Self::POS_INF;

    const TRANSPOSITION_TABLE_SIZE_MB: usize = 64;
    const MAX_MATE_DEPTH: i32 = 1000;

    pub fn start_search(&mut self,
        board: &mut Board, 
        move_gen: &mut MoveGenerator,
        precomp: &PrecomputedMoveData,
        bbutils: &BitBoardUtils,
        magic: &MagicBitBoards,
        zobrist: &Zobrist,
    ) {
        let init_moves = move_gen.moves.clone();
        if init_moves.len() == 0 {
            self.best_move_so_far = Move::NULL;
            return;
        }

        self.move_ordering.clear_history();
        self.repetition_table = board.repeat_position_history.clone();

        self.best_move_so_far = Move::NULL;
        self.best_eval_so_far = 0;

        self.positions_evaled = 0;
        self.num_mates = 0;
        self.num_cutoffs = 0;
        self.num_transpositions = 0;

        self.has_searched_one_move = false;
        self.move_is_from_partial_search = false;
        self.search_cancelled = false;

        self.search_iteration_time = SystemTime::now();
        self.search_total_time = SystemTime::now();

        self.start_iterative_deepening(
            board,
            move_gen,
            precomp,
            bbutils,
            magic,
            zobrist,
        );

        if self.best_move_so_far == Move::NULL {
            self.best_move_so_far = init_moves[0];
        }
    }

    fn start_iterative_deepening(
        &mut self, 
        board: &mut Board,
        move_gen: &mut MoveGenerator,
        precomp: &PrecomputedMoveData,
        bbutils: &BitBoardUtils,
        magic: &MagicBitBoards,
        zobrist: &Zobrist, 
    ) {
        for search_depth in 1u8..=255u8 {
            self.has_searched_one_move = false;
            self.search_iteration_time = SystemTime::now();
            self.current_iter_depth = search_depth as i32;
            self.search(
                search_depth, 0, Self::NEG_INF, Self::POS_INF,
                Move::NULL, false,
                board,
                move_gen,
                precomp,
                bbutils,
                magic,
                zobrist,
            );
            
            if self.search_cancelled {
                if self.has_searched_one_move {
                    self.best_move_so_far = self.best_move_this_iter;
                    self.best_eval_so_far = self.best_eval_this_iter;
                    self.move_is_from_partial_search = true;
                }
                break;
            } else {
                self.current_depth = search_depth as i32;
                self.best_move_so_far = self.best_move_this_iter;
                self.best_eval_so_far = self.best_eval_this_iter;

                self.best_eval_this_iter = Self::NEG_INF;
                self.best_move_this_iter = Move::NULL;
            }
        }
    }
    
    fn search(
        &mut self, depth_remaining: u8, current_depth: u8, mut alpha: i32, mut beta: i32,
        prev_move: Move, prev_was_capture: bool,
        board: &mut Board,
        move_gen: &mut MoveGenerator,
        precomp: &PrecomputedMoveData,
        bbutils: &BitBoardUtils,
        magic: &MagicBitBoards,
        zobrist: &Zobrist,
    ) -> i32 {
        // Cancel search if over max think time
        if SystemTime::now().duration_since(self.search_total_time).unwrap().as_millis() as u32 > self.max_think_time_ms {
            self.search_cancelled = true;
            return 0;
        }

        if current_depth > 0 {
            // Punish repeated positions
            if board.current_state.fifty_move_counter >= 100 || self.repetition_table.contains(&board.current_state.zobrist_key) {
                return 0;
            }

            // Prune if shorter mating sequence has been found
            alpha = alpha.max(-Self::MATE_SCORE + current_depth as i32);
            beta = beta.min(Self::MATE_SCORE - current_depth as i32);
            if alpha >= beta {
                return alpha;
            }
        }

        // Try getting the position from the transposition table
        if let Some(tt_val) = self.transposition_table.get_evaluation(depth_remaining, current_depth, alpha, beta, board) {
            self.num_transpositions += 1;
            if current_depth == 0 {
                self.best_move_this_iter = self.transposition_table.get_stored_move(board).unwrap();
                self.best_eval_this_iter = self.transposition_table.entries[self.transposition_table.index(board)].clone().unwrap().value;
            }
            return tt_val;
        }

        // If leaf node is reached, evaluate the board
        if depth_remaining == 0 {
            return self.quiescence_search(alpha, beta, board, move_gen, precomp, bbutils, magic, zobrist);
        };

        move_gen.generate_moves(board, precomp, bbutils, magic, false);
        let mut moves = move_gen.moves.clone();
        let tt_stored_move = self.transposition_table.get_stored_move(board);
        let prev_best_move = if current_depth == 0 { self.best_move_so_far } else { match tt_stored_move { None => Move::NULL, Some(mov) => mov } };
        self.move_ordering.order_moves(prev_best_move, &mut moves, board, bbutils, move_gen.enemy_attack_map, move_gen.enemy_pawn_attack_map, false, current_depth as usize);

        // Check if position is terminal
        if moves.len() == 0 {
            if move_gen.in_check() {
                self.num_mates += 1;
                // Favor faster mates
                let mate_score = Self::MATE_SCORE - current_depth as i32;
                return -mate_score;
            } else { // Stalemate
                return 0;
            };
        };

        if current_depth > 0 {
            let was_pawn_move = board.square[prev_move.target().index()].piece_type() == Piece::PAWN;
            if was_pawn_move || prev_was_capture { self.repetition_table.clear() };
            self.repetition_table.push(board.current_state.zobrist_key);
        }

        let mut evaluation_bound = EvaluationType::UpperBound;
        let mut best_move_this_position = Move::NULL;

        // Loop through legal moves
        for mov in moves.iter() {
            let captured_ptype = board.square[mov.target().index()].piece_type();
            let is_capture = captured_ptype != Piece::NONE;
            board.make_move(*mov, true, zobrist);
            // Negate evaluation -- A bad position for the opponent is good for us and vice versa
            let eval = -self.search(
                depth_remaining - 1,
                current_depth + 1,
                -beta,
                -alpha,
                mov.clone(), 
                is_capture,
                board,
                move_gen, 
                precomp,
                bbutils,
                magic,
                zobrist,
            );
            board.unmake_move(*mov, true);
            // Exit early if search is cancelled
            if self.search_cancelled {
                return 0;
            }

            // Beta cutoff / Fail high
            if eval >= beta {
                // Very good move but not the best, store as lower bound
                self.transposition_table.store_evaluation(depth_remaining, current_depth, beta, EvaluationType::LowerBound, mov.clone(), board);
                
                if !is_capture {
                    // Favor killer moves (moves that cause branches to be pruned) when ordering moves
                    if current_depth < MoveOrdering::MAX_KILLER_MOVE_PLY as u8 {
                        self.move_ordering.killers[current_depth as usize].add(mov.clone());
                    }

                    let history_score = depth_remaining as i32 * depth_remaining as i32;
                    self.move_ordering.history[board.move_color_idx][mov.start().index()][mov.target().index()] += history_score;
                }

                if current_depth > 0 {
                    self.repetition_table.pop();
                }

                self.num_cutoffs += 1;
                return beta;
            }

            // New best move for this position
            if eval > alpha {
                evaluation_bound = EvaluationType::Exact;
                best_move_this_position = mov.clone();
                alpha = eval;
                if current_depth == 0 {
                    self.best_eval_this_iter = eval;
                    self.best_move_this_iter = *mov;
                    self.has_searched_one_move = true;
                }
            }
        };

        if current_depth > 0 {
            self.repetition_table.pop();
        }
        self.transposition_table.store_evaluation(depth_remaining, current_depth, alpha, evaluation_bound, best_move_this_position, board);

        return alpha;
    }

    fn quiescence_search(
        &mut self, mut alpha: i32, beta: i32,
        board: &mut Board,
        move_gen: &mut MoveGenerator,
        precomp: &PrecomputedMoveData,
        bbutils: &BitBoardUtils,
        magic: &MagicBitBoards,
        zobrist: &Zobrist,
    ) -> i32 {
        if self.search_cancelled { return 0; }

        let mut eval = Evaluation::evaluate(board);
        self.positions_evaled += 1;

        if eval > beta {
            self.num_cutoffs += 1;
            return beta;
        }
        if eval > alpha {
            alpha = eval;
        }

        move_gen.generate_moves(board, precomp, bbutils, magic, true);
        let mut moves = move_gen.moves.clone();
        self.move_ordering.order_moves(Move::NULL, &mut moves, board, bbutils, move_gen.enemy_attack_map, move_gen.enemy_pawn_attack_map, true, 0);
        for mov in moves.iter() {
            board.make_move(mov.clone(), true, zobrist);
            eval = -self.quiescence_search(-beta, -alpha, board, move_gen, precomp, bbutils, magic, zobrist);
            board.unmake_move(mov.clone(), true);

            if eval >= beta {
                self.num_cutoffs += 1;
                return beta;
            }
            if eval > alpha {
                alpha = eval;
            }
        }
        return alpha;
    }

    pub fn is_mate_score(score: i32) -> bool {
        if score == i32::MIN { return false; };
        return score.abs() > Self::MATE_SCORE - Self::MAX_MATE_DEPTH;
    }
}

impl Default for Searcher {
    fn default() -> Self {
        Self {
            repetition_table: Vec::new(),
            transposition_table: TranspositionTable::new(Self::TRANSPOSITION_TABLE_SIZE_MB),
            move_ordering: MoveOrdering::new(),

            current_depth: 0,
            positions_evaled: 0,
            best_eval_so_far: 0,
            best_move_so_far: Move::NULL,
            num_mates: 0,
            num_cutoffs: 0,
            num_transpositions: 0,
            has_searched_one_move: false,
            search_cancelled: false,
            max_think_time_ms: 1000,
            best_eval_this_iter: 0,
            best_move_this_iter: Move::NULL,
            current_iter_depth: 0,
            move_is_from_partial_search: false,

            search_iteration_time: SystemTime::now(),
            search_total_time: SystemTime::now(),
        }
    }
}

pub fn start_search(
    mut searcher: ResMut<Searcher>,
    mut begin_search_evr: EventReader<BeginSearch>,
    mut search_complete_evw: EventWriter<SearchComplete>,

    mut board: ResMut<Board>,
    mut move_gen: ResMut<MoveGenerator>,
    precomp: Res<PrecomputedMoveData>,
    bbutils: Res<BitBoardUtils>,
    magic: Res<MagicBitBoards>,
    zobrist: Res<Zobrist>,
) {
    for begin_search_event in begin_search_evr.iter() {
        if begin_search_event.version != AIVersion::V8 {
            continue;
        }
        let time_start = std::time::SystemTime::now();
        searcher.max_think_time_ms = begin_search_event.think_time;
        searcher.start_search(
            board.as_mut(),
            move_gen.as_mut(),
            precomp.as_ref(),
            bbutils.as_ref(),
            magic.as_ref(),
            zobrist.as_ref(),
        );
        let think_time = std::time::SystemTime::now().duration_since(time_start).unwrap().as_millis();
        search_complete_evw.send(SearchComplete {
            depth: searcher.current_depth,
            chosen_move: searcher.best_move_so_far,
            eval: searcher.best_eval_so_far,
            stats: SearchStatistics {
                num_position_evals: searcher.positions_evaled,
                num_cutoffs: searcher.num_cutoffs,
                num_transpositions: searcher.num_transpositions,
                think_time_ms: think_time as u32,
                num_checks: 0,
                num_mates: searcher.num_mates,
                is_book: false,
            }
        });
    }
}
