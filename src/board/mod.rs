pub mod board;
pub mod coord;
pub mod piece;
pub mod piece_list;
pub mod zobrist;
pub mod moves;
pub mod game_state;

use bevy::prelude::*;
use crate::state::AppState;
use board::*;
use zobrist::*;

fn finalize_zobrist(
    mut commands: Commands
) {
    commands.insert_resource(NextState(Some(AppState::LoadBoard)))
}

fn finalize(
    mut commands: Commands,
) {
    commands.insert_resource(NextState(Some(AppState::LoadMoveGen)))
}

pub struct BoardPlugin;

impl Plugin for BoardPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(OnEnter(AppState::LoadZobrist), (
                spawn_zobrist,
                finalize_zobrist,
            ).chain())
            .add_systems(OnEnter(AppState::LoadBoard), (
                spawn_main_board,
                finalize,
            ).chain())
            // .add_systems(Update, (
            //     make_move,
            // ).run_if(in_state(AppState::InGame)))
        ;
    }
}