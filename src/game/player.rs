use bevy::prelude::*;

use crate::{
    ui::menu::GameType,
    game::manager::PlayerType,
    game::human_player::HumanPlayer,
    board::piece::{WHITE, BLACK},
};


#[derive(Component)]
pub struct Player {
    pub team: u32,
}

pub fn spawn_players(
    mut commands: Commands,
    game_type_query: Query<&GameType>
) {
    if let Ok(game_type) = game_type_query.get_single() {
        match game_type.white {
            PlayerType::Human => { commands.spawn((HumanPlayer::default(), Player { team: WHITE })); },
            PlayerType::AI => (),
        }
        match game_type.black {
            PlayerType::Human => { commands.spawn((HumanPlayer::default(), Player { team: BLACK })); },
            PlayerType::AI => (),
        }
    }
}