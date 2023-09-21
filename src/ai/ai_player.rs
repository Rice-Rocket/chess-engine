use bevy::prelude::*;
use crate::{board::moves::Move, game::{manager::{BoardMakeMove, GameManager, CanMakeMove}, player::Player}, ui::ingame_menu::CalcStatistics, state::AppMode};
use super::stats::SearchStatistics;


pub const DEFAULT_AI_THINK_TIME_MS: u32 = 1000;


#[derive(PartialEq, Default, Clone, Copy)]
pub enum AIVersion {
    V0,
    V1,
    V2,
    V3,
    V4,
    #[default]
    V5,
}

impl AIVersion {
    // Newest version (version to test)
    pub fn primary_version() -> Self {
        AIVersion::V5
    }
    // Version for primary version to fight
    pub fn secondary_version() -> Self {
        AIVersion::V4
    }
    pub fn label(&self) -> &str {
        match self {
            Self::V0 => "V0 - Random Moves",
            Self::V1 => "V1 - Minimax",
            Self::V2 => "V2 - Iterative Deepening",
            Self::V3 => "V3 - Alphabeta Pruning",
            Self::V4 => "V4 - Repetitions and Transpositions",
            Self::V5 => "V5 - Move Ordering",
        }
    }

    pub fn opponent(self) -> Option<Self> {
        if self == Self::primary_version() {
            return Some(Self::secondary_version());
        } else if self == Self::secondary_version() {
            return Some(Self::primary_version());
        }
        None
    }
}

#[derive(Component)]
pub struct AIPlayer {
    searching: bool,
    pub think_time_ms: u32,
    pub version: AIVersion
}

impl AIPlayer {
    pub fn versus_p1() -> Self {
        AIPlayer {
            version: AIVersion::primary_version(),
            think_time_ms: DEFAULT_AI_THINK_TIME_MS,
            searching: false,
        }
    }
    pub fn versus_p2() -> Self {
        AIPlayer {
            version: AIVersion::secondary_version(),
            think_time_ms: DEFAULT_AI_THINK_TIME_MS,
            searching: false,
        }
    }
}

impl Default for AIPlayer {
    fn default() -> Self {
        AIPlayer {
            version: AIVersion::primary_version(),
            think_time_ms: DEFAULT_AI_THINK_TIME_MS,
            searching: false,
        }
    }
}

#[derive(Event)]
pub struct BeginSearch {
    pub version: AIVersion,
    pub think_time: u32,
}

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
    mut calc_stats: ResMut<CalcStatistics>,
) {
    
    for search_complete in search_complete_evr.iter() {
        for mut ai in player_query.iter_mut() {
            ai.searching = false;
        }
        calc_stats.ai_depth = search_complete.depth;
        calc_stats.ai_eval = search_complete.eval;
        calc_stats.ai_positions_evaled = search_complete.stats.num_position_evals;
        calc_stats.ai_mates_found = search_complete.stats.num_mates;
        calc_stats.ai_num_cutoffs = search_complete.stats.num_cutoffs;
        calc_stats.ai_think_time = search_complete.stats.think_time_ms;
        make_move_evw.send(BoardMakeMove {
            mov: search_complete.chosen_move
        });
    }
}

pub fn ai_begin_search(
    mut begin_search_evw: EventWriter<BeginSearch>,
    mut player_query: Query<(&mut AIPlayer, &Player)>,
    manager: Res<GameManager>,
    app_mode: Res<State<AppMode>>,
    mut can_make_move_evr: EventReader<CanMakeMove>,
) {
    for _can_make_move_ev in can_make_move_evr.iter() {
        for (mut ai, player_data) in player_query.iter_mut() {
            if player_data.team == manager.move_color && !ai.searching {
                match app_mode.clone() {
                    AppMode::None | AppMode::GameHumanHuman => (),
                    AppMode::GameHumanAI => {
                        if ai.version == AIVersion::primary_version() {
                            ai.searching = true;
                            begin_search_evw.send(BeginSearch {
                                version: ai.version,
                                think_time: DEFAULT_AI_THINK_TIME_MS,
                            });
                        }
                    },
                    AppMode::GameAIAI => {
                        ai.searching = true;
                        begin_search_evw.send(BeginSearch {
                            version: ai.version,
                            think_time: ai.think_time_ms,
                        });
                    }

                }
            }
        }
    }
}