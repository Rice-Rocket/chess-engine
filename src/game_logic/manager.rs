use bevy::prelude::*;

use super::{player::Player, moves::Move};

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

