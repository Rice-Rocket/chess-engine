use bevy::prelude::*;

use crate::{board::moves::Move, game::{manager::{BoardMakeMove, GameManager}, player::Player}};

use super::stats::SearchStatistics;


#[derive(Clone, Copy)]
pub enum AIVersion {
    V0,
}

impl AIVersion {
    pub fn label(&self) -> &str {
        match self {
            Self::V0 => "V0 - Random Moves",
        }
    }
    // pub fn searcher(&self) -> 
}


#[derive(Event)]
pub struct BeginSearch {}

#[derive(Event)]
pub struct SearchComplete {
    pub depth: i32,
    pub chosen_move: Move,
    pub eval: i32,
    pub stats: SearchStatistics
}

#[derive(Component)]
pub struct AIPlayer {
    searching: bool,
    pub version: AIVersion
}

impl Default for AIPlayer {
    fn default() -> Self {
        AIPlayer {
            searching: false,
            version: AIVersion::V0,
        }
    }
}

pub fn ai_make_move(
    mut make_move_evw: EventWriter<BoardMakeMove>,
    mut player_query: Query<&mut AIPlayer>,
    mut search_complete_evr: EventReader<SearchComplete>,
) {
    
    for search_complete in search_complete_evr.iter() {
        for mut ai in player_query.iter_mut() {
            ai.searching = false;
        }
        make_move_evw.send(BoardMakeMove {
            mov: search_complete.chosen_move
        });
    }
}

pub fn ai_begin_search(
    mut begin_search_evw: EventWriter<BeginSearch>,
    mut player_query: Query<(&mut AIPlayer, &Player)>,
    manager: Res<GameManager>,
) {
    for (mut ai, player_data) in player_query.iter_mut() {
        if player_data.team == manager.move_color && !ai.searching {
            ai.searching = true;
            begin_search_evw.send(BeginSearch {});
        }
    }
}