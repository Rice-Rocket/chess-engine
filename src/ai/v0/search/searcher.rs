use bevy::prelude::*;
use rand::Rng;

use crate::{board::{moves::Move, board::Board}, move_gen::{move_generator::MoveGenerator, precomp_move_data::PrecomputedMoveData, bitboard::utils::BitBoardUtils, magics::MagicBitBoards}, ai::{ai_player::{BeginSearch, SearchComplete}, stats::SearchStatistics}};


#[derive(Resource)]
pub struct SearcherV0 {
    pub current_depth: i32,
    pub best_move_so_far: Move,
    pub best_eval_so_far: i32,
}

impl SearcherV0 {
    // const TRANSPOSITION_TABLE_SIZE: usize = 64;
    // const MAX_EXTENSIONS: usize = 16;

    // const IMMEDIATE_MATE_SCORE: i32 = 100000;
    // const POS_INF: i32 = 9999999;
    // const NEG_INF: i32 = -Self::POS_INF;

    pub fn start_search(&mut self,
        board: &mut Board, 
        move_gen: &mut MoveGenerator,
        precomp: &PrecomputedMoveData,
        bbutils: &BitBoardUtils,
        magic: &MagicBitBoards,
    ) {
        // move_gen.generate_moves(board, precomp, bbutils, magic, false);
        let moves = &move_gen.moves;
        if moves.len() == 0 {
            self.best_move_so_far = Move::NULL;
            return;
        }
        let rand_idx = rand::thread_rng().gen_range(0..moves.len());
        self.best_move_so_far = moves[rand_idx];
    }
}

impl Default for SearcherV0 {
    fn default() -> Self {
        Self {
            current_depth: 0,
            best_move_so_far: Move::NULL,
            best_eval_so_far: 0,
        }
    }
}

pub fn spawn_searcher_v0(
    mut commands: Commands,
) {
    commands.insert_resource(SearcherV0::default());
}

pub fn start_search_v0(
    mut searcher: ResMut<SearcherV0>,
    mut begin_search_evr: EventReader<BeginSearch>,
    mut search_complete_evw: EventWriter<SearchComplete>,

    mut board: ResMut<Board>,
    mut move_gen: ResMut<MoveGenerator>,
    precomp: Res<PrecomputedMoveData>,
    bbutils: Res<BitBoardUtils>,
    magic: Res<MagicBitBoards>,
) {
    for _begin_search_event in begin_search_evr.iter() {
        searcher.start_search(
            board.as_mut(),
            move_gen.as_mut(),
            precomp.as_ref(),
            bbutils.as_ref(),
            magic.as_ref(),
        );
        search_complete_evw.send(SearchComplete {
            depth: 0,
            chosen_move: searcher.best_move_so_far,
            eval: 0,
            stats: SearchStatistics {
                num_iters: 0,
                num_position_evals: 0,
                num_cutoffs: 0,
                eval: 0,
                num_checks: 0,
                num_mates: 0,
                is_book: false,
            }
        });
    }
}
