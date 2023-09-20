use bevy::prelude::*;
use rand::Rng;

use crate::{board::moves::Move, move_gen::move_generator::MoveGenerator, ai::{ai_player::{BeginSearch, SearchComplete}, stats::SearchStatistics}};


#[derive(Resource)]
pub struct SearcherV0 {
    pub current_depth: i32,
    pub best_move_so_far: Move,
    pub best_eval_so_far: i32,
}

impl SearcherV0 {
    pub fn start_search(&mut self,
        move_gen: &mut MoveGenerator,
    ) {
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

pub fn start_search(
    mut searcher: ResMut<SearcherV0>,
    mut begin_search_evr: EventReader<BeginSearch>,
    mut search_complete_evw: EventWriter<SearchComplete>,

    mut move_gen: ResMut<MoveGenerator>,
) {
    for _begin_search_event in begin_search_evr.iter() {
        searcher.start_search(
            move_gen.as_mut(),
        );
        search_complete_evw.send(SearchComplete {
            depth: 0,
            chosen_move: searcher.best_move_so_far,
            eval: 0.0,
            stats: SearchStatistics {
                num_position_evals: 0,
                num_cutoffs: 0,
                think_time_ms: 0,
                num_checks: 0,
                num_mates: 0,
                is_book: false,
            }
        });
    }
}
