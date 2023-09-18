use bevy::prelude::*;

use crate::{board::{moves::Move, board::Board, zobrist::Zobrist}, move_gen::{move_generator::MoveGenerator, precomp_move_data::PrecomputedMoveData, bitboard::utils::BitBoardUtils, magics::MagicBitBoards}};

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
    mut move_gen: ResMut<MoveGenerator>,
    mut board: ResMut<Board>,
    precomp: Res<PrecomputedMoveData>,
    bbutils: Res<BitBoardUtils>,
    magic: Res<MagicBitBoards>,
    zobrist: Res<Zobrist>,
) {
    for make_move_event in make_move_evr.iter() {
        let mov = make_move_event.mov;
        board.make_move(mov, false, &zobrist);
        move_gen.generate_moves(&board.into(), &precomp, &bbutils, &magic, false);
        break;
    }
}
