use bevy::prelude::*;

use crate::{
    game::manager::PlayerType,
    game::human_player::HumanPlayer,
    board::piece::*, ai::ai_player::AIPlayer, state::AppMode,
};


#[derive(Component)]
pub struct Player {
    pub team: u8,
}

pub fn spawn_players(
    mut commands: Commands,
    app_mode: Res<State<AppMode>>
) {
    let (white, black) = match app_mode.clone() {
        AppMode::GameHumanHuman => (PlayerType::Human, PlayerType::Human),
        AppMode::GameHumanAI => (PlayerType::Human, PlayerType::AI),
        AppMode::GameAIAI => (PlayerType::AI, PlayerType::AI),
        AppMode::None => (PlayerType::Human, PlayerType::Human),
    };
    if app_mode.clone() != AppMode::GameAIAI {
        match white {
            PlayerType::Human => { commands.spawn((HumanPlayer::default(), Player { team: Piece::WHITE })); },
            PlayerType::AI => { commands.spawn((AIPlayer::default(), Player { team: Piece::WHITE })); },
        }
        match black {
            PlayerType::Human => { commands.spawn((HumanPlayer::default(), Player { team: Piece::BLACK })); },
            PlayerType::AI => { commands.spawn((AIPlayer::default(), Player { team: Piece::BLACK })); },
        }
    } else {
        commands.spawn((AIPlayer::versus_p1(), Player { team: Piece::WHITE }));
        commands.spawn((AIPlayer::versus_p1(), Player { team: Piece::BLACK }));
    }
}