use bevy::prelude::*;
use crate::{board::{moves::Move, board::Board, zobrist::Zobrist}, move_gen::{move_generator::MoveGenerator, precomp_move_data::PrecomputedMoveData, bitboard::utils::BitBoardUtils, magics::MagicBitBoards}, ai::{ai_player::{BeginSearch, SearchComplete, AIVersion}, stats::SearchStatistics}};

use super::super::evaluation::eval::Evaluation;

#[derive(Resource)]
pub struct Searcher {
    pub current_depth: i32,
    pub best_move_so_far: Move,
    pub best_eval_so_far: f32,

    positions_evaled: u32,
    num_mates: i32,
    has_searched_one_move: bool,
}

impl Searcher {
    pub const SEARCH_DEPTH: i32 = 4;
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
        move_gen.generate_moves(board, precomp, bbutils, magic, false);
        let init_moves = &move_gen.moves;
        if init_moves.len() == 0 {
            self.best_move_so_far = Move::NULL;
            return;
        }

        self.positions_evaled = 0;
        self.best_eval_so_far = 0.0;
        self.best_move_so_far = Move::NULL;
        self.num_mates = 0;
        self.has_searched_one_move = false;

        
        self.search(
            Self::SEARCH_DEPTH, 0,
            board,
            move_gen,
            precomp,
            bbutils,
            magic,
            zobrist,
        );

        // board.white_to_move = white_to_move;
        // board.move_color = if white_to_move { Piece::WHITE } else { Piece::BLACK };
        // board.opponent_color = if white_to_move { Piece::BLACK } else { Piece::WHITE };
        // board.move_color_idx = if white_to_move { Board::WHITE_INDEX } else { Board::BLACK_INDEX };
        // board.opponent_color_idx = 1 - board.move_color_idx;
    }
    fn search(
        &mut self, depth_remaining: i32, current_depth: i32,
        board: &mut Board,
        move_gen: &mut MoveGenerator,
        precomp: &PrecomputedMoveData,
        bbutils: &BitBoardUtils,
        magic: &MagicBitBoards,
        zobrist: &Zobrist,
    ) -> f32 {
        if depth_remaining == 0 {
            self.positions_evaled += 1;
            return Evaluation::evaluate(board);
        };

        move_gen.generate_moves(board, precomp, bbutils, magic, false);
        let moves = move_gen.moves.clone();

        // check if position is terminal
        if moves.len() == 0 {
            if move_gen.in_check() {
                self.num_mates += 1;
                // favor faster mates
                let mate_score = Self::MATE_SCORE - current_depth as f32;
                return -mate_score;
            } else { // stalemate
                return 0.0;
            };
        };

        let mut best_eval = f32::MIN;
        // let mut best_move = Move::NULL;
        for mov in moves.iter() {
            board.make_move(*mov, true, zobrist);
            // negate evaluation, switiching sides
            let eval = -self.search(
                depth_remaining - 1,
                current_depth + 1,
                board,
                move_gen, 
                precomp,
                bbutils,
                magic,
                zobrist,
            );
            board.unmake_move(*mov, true);
            if eval > best_eval {
                best_eval = eval;
                // best_move = *mov;
                if current_depth == 0 {
                    self.best_move_so_far = *mov;
                    self.best_eval_so_far = eval;
                    self.has_searched_one_move = true;
                }
            }
        };
        return best_eval;
    }
}

impl Default for Searcher {
    fn default() -> Self {
        Self {
            current_depth: 0,
            best_move_so_far: Move::NULL,
            best_eval_so_far: 0.0,
            positions_evaled: 0,
            num_mates: 0,
            has_searched_one_move: false,
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
        if begin_search_event.version != AIVersion::V1 {
            continue;
        }
        let time_start = std::time::Instant::now();
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
            depth: Searcher::SEARCH_DEPTH,
            chosen_move: searcher.best_move_so_far,
            eval: searcher.best_eval_so_far as i32,
            stats: SearchStatistics {
                num_position_evals: searcher.positions_evaled,
                num_cutoffs: 0,
                think_time_ms: think_time as u32,
                num_checks: 0,
                num_mates: searcher.num_mates,
                is_book: false,
                ..default()
            }
        });
    }
}
