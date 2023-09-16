use bevy::prelude::*;

pub mod board;
pub mod clock;
pub mod coord;
pub mod fen;
pub mod manager;
pub mod move_generator;
pub mod moves;
pub mod piece_list;
pub mod piece_sqr_table;
pub mod piece;
pub mod player;
pub mod human_player;
pub mod precomp_move_data;
pub mod pseudo_legal_moves;
pub mod representation;
pub mod transposition_table;
pub mod utils;
pub mod zobrist;

use board::*;
use player::*;
use human_player::*;
use precomp_move_data::*;
use pseudo_legal_moves::*;

use crate::ui::menu::AppState;

pub fn spawn_game_logic_resources(
    mut commands: Commands,
) {
    commands.insert_resource(PrecomputedMoveData::default());
    commands.insert_resource(PseudoLegalMoveGenerator::default());
}

pub fn finish_load_game_logic(
    mut commands: Commands,
    // mut pseudo_move_gen: ResMut<PseudoLegalMoveGenerator>,
    // board_query: Query<&Board, With<MainBoard>>,
    // precomp: Res<PrecomputedMoveData>,
) {
    // if let Ok(board) = board_query.get_single() {
    //     pseudo_move_gen.generate_moves(board, &precomp);
    // }
    commands.insert_resource(NextState(Some(AppState::LoadUI)));
}

pub struct GameLogicPlugin;

impl Plugin for GameLogicPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_event::<BoardMakeMove>()

            .add_systems(OnEnter(AppState::LoadGameLogic), (
                spawn_game_logic_resources,
                spawn_main_board,
                spawn_players,
                finish_load_game_logic,
            ).chain())
            
            .add_systems(Update, (
                handle_player_input,
                make_move.before(generate_pseudo_legal_moves),
                generate_pseudo_legal_moves,
            ).run_if(in_state(AppState::InGame)));
    }
}