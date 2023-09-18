pub mod move_generator;
pub mod precomp_move_data;
pub mod bitboard;
pub mod magics;

use bevy::prelude::*;
use crate::state::AppState;
use precomp_move_data::*;
use bitboard::utils::*;
use bitboard::precomp_bits::*;
use magics::*;
use move_generator::*;

fn finalize_precomp(
    mut commands: Commands,
) {
    commands.insert_resource(NextState(Some(AppState::LoadZobrist)))
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
                spawn_bitboard_utils,
                finalize_precomp,
            ).chain())
            .add_systems(OnEnter(AppState::LoadMoveGen), (
                spawn_magic_bitboards,
                spawn_precomp_bits,
                spawn_movegen,
                finalize_move_gen,
            ).chain())
        ;
    }
}