use std::time::Instant;

use bevy::prelude::*;
use crate::{board::{moves::Move, board::Board, zobrist::Zobrist}, move_gen::{move_generator::MoveGenerator, precomp_move_data::PrecomputedMoveData, bitboard::utils::BitBoardUtils, magics::MagicBitBoards}, ai::{ai_player::{BeginSearch, SearchComplete, AIVersion}, stats::SearchStatistics}};

use super::super::evaluation::eval::Evaluation;

#[derive(Resource)]
pub struct Searcher {
    pub current_depth: i32,
    pub best_move_so_far: Move,
    pub best_eval_so_far: f32,
    pub max_think_time_ms: u32,
    best_move_this_iter: Move,
    best_eval_this_iter: f32,

    positions_evaled: u32,
    num_mates: i32,
    num_cutoffs: i32,
    has_searched_one_move: bool,
    search_cancelled: bool,

    search_iteration_time: Instant,
    search_total_time: Instant,
    current_iter_depth: i32,
    move_is_from_partial_search: bool,
}

impl Searcher {
    pub const MATE_SCORE: f32 = 100000.0;
    pub const POS_INF: f32 = 9999999.0;
    pub const NEG_INF: f32 = -Self::POS_INF;

    pub fn start_search(&mut self,
        board: &mut Board, 
        move_gen: &mut MoveGenerator,
        precomp: &PrecomputedMoveData,
        bbutils: &BitBoardUtils,
        magic: &MagicBitBoards,
        zobrist: &Zobrist,
    ) {
        let init_moves = &move_gen.moves;
        if init_moves.len() == 0 {
            self.best_move_so_far = Move::NULL;
            return;
        }

        self.positions_evaled = 0;
        self.best_eval_so_far = 0.0;
        self.best_move_so_far = Move::NULL;
        self.num_mates = 0;
        self.num_cutoffs = 0;
        self.has_searched_one_move = false;
        self.move_is_from_partial_search = false;
        self.search_cancelled = false;

        self.search_iteration_time = Instant::now();
        self.search_total_time = Instant::now();

        self.start_iterative_deepening(
            board,
            move_gen,
            precomp,
            bbutils,
            magic,
            zobrist,
        );
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
        for search_depth in 1..=256 {
            self.has_searched_one_move = false;
            self.search_iteration_time = Instant::now();
            self.current_iter_depth = search_depth;
            self.search(
                search_depth, 0, Self::NEG_INF, Self::POS_INF,
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
                self.current_depth = search_depth;
                self.best_move_so_far = self.best_move_this_iter;
                self.best_eval_so_far = self.best_eval_this_iter;

                self.best_eval_this_iter = Self::NEG_INF;
                self.best_move_this_iter = Move::NULL;
            }
        }
    }
    
    fn search(
        &mut self, depth_remaining: i32, current_depth: i32, mut alpha: f32, mut beta: f32,
        board: &mut Board,
        move_gen: &mut MoveGenerator,
        precomp: &PrecomputedMoveData,
        bbutils: &BitBoardUtils,
        magic: &MagicBitBoards,
        zobrist: &Zobrist,
    ) -> f32 {
        // Cancel search if over max think time
        if Instant::now().duration_since(self.search_total_time).as_millis() as u32 > self.max_think_time_ms {
            self.search_cancelled = true;
            return 0.0;
        }
        if current_depth > 0 {
            // Prune if shorter mating sequence has been found
            alpha = alpha.max(-Self::MATE_SCORE + current_depth as f32);
            beta = beta.min(Self::MATE_SCORE - current_depth as f32);
            if alpha >= beta {
                return alpha;
            }
        }
        // If leaf node is reached, evaluate the board
        if depth_remaining == 0 {
            self.positions_evaled += 1;
            return Evaluation::evaluate(board);
        };

        move_gen.generate_moves(board, precomp, bbutils, magic, false);
        let moves = move_gen.moves.clone();

        // Check if position is terminal
        if moves.len() == 0 {
            if move_gen.in_check() {
                self.num_mates += 1;
                // Favor faster mates
                let mate_score = Self::MATE_SCORE - current_depth as f32;
                return -mate_score;
            } else { // Stalemate
                return 0.0;
            };
        };

        // Loop through legal moves
        for mov in moves.iter() {
            board.make_move(*mov, true, zobrist);
            // Negate evaluation -- A bad position for the opponent is good for us and vice versa
            let eval = -self.search(
                depth_remaining - 1,
                current_depth + 1,
                -beta,
                -alpha,
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
                return 0.0;
            }
            // Beta cutoff / Fail high
            if eval >= beta {
                self.num_cutoffs += 1;
                return beta;
            }
            if eval > alpha {
                alpha = eval;
                if current_depth == 0 {
                    self.best_eval_this_iter = eval;
                    self.best_move_this_iter = *mov;
                    self.has_searched_one_move = true;
                }
            }
        };
        return alpha;
    }

    // fn is_mate_score(score: f32) -> bool {
    //     if score == f32::MIN { return false; }
    //     const MAX_MATE_DEPTH: f32 = 1000.0;
    //     return score.abs() > Self::MATE_SCORE - MAX_MATE_DEPTH;
    // }
}

impl Default for Searcher {
    fn default() -> Self {
        Self {
            current_depth: 0,
            best_move_so_far: Move::NULL,
            best_eval_so_far: 0.0,
            positions_evaled: 0,
            max_think_time_ms: 1000,
            num_mates: 0,
            num_cutoffs: 0,
            has_searched_one_move: false,
            search_cancelled: false,
            best_move_this_iter: Move::NULL,
            best_eval_this_iter: 0.0,
            search_iteration_time: Instant::now(),
            search_total_time: Instant::now(),
            current_iter_depth: 0,
            move_is_from_partial_search: false,
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
        if begin_search_event.version != AIVersion::V3 {
            continue;
        }
        let time_start = std::time::Instant::now();
        searcher.max_think_time_ms = begin_search_event.think_time;
        searcher.start_search(
            board.as_mut(),
            move_gen.as_mut(),
            precomp.as_ref(),
            bbutils.as_ref(),
            magic.as_ref(),
            zobrist.as_ref(),
        );
        let think_time = std::time::Instant::now().duration_since(time_start).as_millis();
        search_complete_evw.send(SearchComplete {
            depth: searcher.current_depth,
            chosen_move: searcher.best_move_so_far,
            eval: searcher.best_eval_so_far as i32,
            stats: SearchStatistics {
                num_position_evals: searcher.positions_evaled,
                num_cutoffs: searcher.num_cutoffs,
                think_time_ms: think_time as u32,
                num_checks: 0,
                num_mates: searcher.num_mates,
                is_book: false,
                ..default()
            }
        });
    }
}
