pub mod ai_player;
pub mod stats;
use bevy::prelude::*;
use crate::state::{AppState, AppMode};
use self::ai_player::*;

pub mod v0;
pub mod v1;
pub mod v2;
pub mod v3;
pub mod v4;
pub mod v5;
pub mod v6;


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
            // .add_plugins(v0::AIPluginV0)
            // .add_plugins(v1::AIPluginV1)
            // .add_plugins(v2::AIPluginV2)
            // .add_plugins(v3::AIPluginV3)
            .add_plugins(v4::AIPluginV4)
            .add_plugins(v5::AIPluginV5)
            .add_plugins(v6::AIPluginV6)
            .add_systems(OnEnter(AppState::LoadAI), (
                finalize,
            ))
            .add_systems(Update, (
                // v0::search::searcher::start_search,
                // v1::search::searcher::start_search,
                // v2::search::searcher::start_search,
                // v3::search::searcher::start_search,
                v4::search::searcher::start_search,
                v5::search::searcher::start_search,
                v6::search::searcher::start_search,

                ai_begin_search,
                ai_make_move,
            ).chain().run_if(in_state(AppState::InGame)).run_if(in_state(AppMode::GameHumanAI).or_else(in_state(AppMode::GameAIAI))))
        ;
    }
}