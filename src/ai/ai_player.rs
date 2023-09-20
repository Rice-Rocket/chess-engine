use bevy::prelude::*;
use crate::{board::moves::Move, game::{manager::{BoardMakeMove, GameManager, CanMakeMove}, player::Player}};
use super::stats::SearchStatistics;


#[derive(Default, Clone, Copy)]
pub enum AIVersion {
    #[default]
    V0,
}

impl AIVersion {
    // Newest version (version to test)
    pub fn primary_version() -> Self {
        AIVersion::V0
    }
    // Version for primary version to fight
    pub fn secondary_version() -> Self {
        AIVersion::V0
    }
    pub fn label(&self) -> &str {
        match self {
            Self::V0 => "V0 - Random Moves",
        }
    }
}

#[derive(Component)]
pub struct AIPlayer {
    searching: bool,
    pub version: AIVersion
}

impl AIPlayer {
    pub fn versus_p1() -> Self {
        AIPlayer {
            version: AIVersion::primary_version(),
            searching: false,
        }
    }
    pub fn versus_p2() -> Self {
        AIPlayer {
            version: AIVersion::secondary_version(),
            searching: false,
        }
    }
}

impl Default for AIPlayer {
    fn default() -> Self {
        AIPlayer {
            version: AIVersion::default(),
            searching: false,
        }
    }
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
    mut can_make_move_evr: EventReader<CanMakeMove>,
) {
    for _can_make_move_ev in can_make_move_evr.iter() {
        for (mut ai, player_data) in player_query.iter_mut() {
            if player_data.team == manager.move_color && !ai.searching {
                ai.searching = true;
                begin_search_evw.send(BeginSearch {});
            }
        }
    }
}