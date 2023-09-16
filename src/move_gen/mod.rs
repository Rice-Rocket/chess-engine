pub mod move_generator;
pub mod precomp_move_data;
pub mod pseudo_legal_moves;

use bevy::prelude::*;
use crate::state::AppState;
use precomp_move_data::*;
use pseudo_legal_moves::*;

fn finalize_precomp(
    mut commands: Commands,
) {
    commands.insert_resource(NextState(Some(AppState::LoadBoard)))
}

fn finalize_move_gen(
    mut commands: Commands,
) {
    commands.insert_resource(NextState(Some(AppState::LoadGame)))
}

pub struct MoveGenPlugin;

impl Plugin for MoveGenPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(OnEnter(AppState::LoadPrecomp), (
                spawn_precomp,
                finalize_precomp,
            ).chain())
            .add_systems(OnEnter(AppState::LoadMoveGen), (
                spawn_pseudo_move_gen,
                finalize_move_gen,
            ).chain())
        ;
    }
}