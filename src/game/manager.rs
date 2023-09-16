use bevy::prelude::*;

use crate::{board::{moves::Move, board::Board}, move_gen::{pseudo_legal_moves::PseudoLegalMoveGenerator, precomp_move_data::PrecomputedMoveData}};

pub enum GameResult {
    Playing,
    WhiteIsMated,
    BlackIsMated,
    Stalemate,
    Repetition,
    FiftyMoveRule,
    InsufficientMaterial,
}

pub enum PlayerType {
    Human, AI
}

#[derive(Component)]
pub struct GameManager {
    pub load_custom_position: bool,
    pub custom_position: String,
    pub white_player_type: PlayerType,
    pub black_player_type: PlayerType,
    pub game_result: GameResult,
    pub game_moves: Vec<Move>,
}

#[derive(Event)]
pub struct BoardMakeMove {
    pub mov: Move
}

pub fn on_make_move(
    mut make_move_evr: EventReader<BoardMakeMove>,
    mut board: ResMut<Board>,
    mut pseudo_move_gen: ResMut<PseudoLegalMoveGenerator>,
    precomp: Res<PrecomputedMoveData>
) {
    for make_move_event in make_move_evr.iter() {
        let mov = make_move_event.mov;
        board.make_move(mov, false);
        pseudo_move_gen.generate_moves(&board.into(), &precomp);
        break;
    }
}
