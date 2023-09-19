pub mod ai_player;
pub mod stats;

pub mod v0;

use bevy::prelude::*;

use crate::state::{AppState, AppMode};

use self::ai_player::*;
use self::v0::search::searcher::*;

fn finalize(
    mut commands: Commands,
) {
    commands.insert_resource(NextState(Some(AppState::InGame)));
}

pub struct AIPlugin;

impl Plugin for AIPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_event::<SearchComplete>()
            .add_event::<BeginSearch>()
            .add_systems(OnEnter(AppState::LoadAI), (
                spawn_searcher_v0,
                finalize,
            ))
            .add_systems(Update, (
                ai_begin_search,
                start_search_v0,
                ai_make_move,
            ).chain().run_if(in_state(AppState::InGame)).run_if(in_state(AppMode::GameHumanAI).or_else(in_state(AppMode::GameAIAI))))
        ;
    }
}