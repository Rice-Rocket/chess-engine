pub mod clock;
pub mod human_player;
pub mod manager;
pub mod player;
pub mod representation;

use bevy::prelude::*;
use crate::state::AppState;
use player::*;
use human_player::*;
use manager::*;

use self::manager::BoardMakeMove;

fn finalize(
    mut commands: Commands,
) {
    commands.insert_resource(NextState(Some(AppState::LoadUI)))
}

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app
            .add_event::<BoardMakeMove>()
            .add_systems(OnEnter(AppState::LoadGame), (
                spawn_players,
                spawn_game_manager,
                finalize,
            ).chain())
            .add_systems(OnEnter(AppState::InGame), initialize_game)
            .add_systems(Update, (
                handle_player_input,
                execute_board_move.before(on_make_move),
                on_make_move,
            ).run_if(in_state(AppState::InGame)))
        ;
    }
}