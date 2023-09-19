use bevy::prelude::*;

use crate::{state::AppState, ui::ingame_menu::{MatchManagerText, MatchManagerStatistic, MatchManagerStartButton}, board::{board::Board, zobrist::Zobrist}};

use super::manager::GameManager;


#[derive(Resource)]
pub struct VersusManager {
    pub max_think_time_ms: usize,
    pub max_game_length: usize,
    pub game_idx: usize,
    pub p1_wins: usize,
    pub p1_losses: usize,
    pub draws: usize,
    position_fens: Vec<String>,
}

pub fn spawn_versus_manager(
    mut commands: Commands,
) {
    let versus_positions_full = std::fs::read_to_string("assets/logic/versus_positions.txt").unwrap();
    let positions: Vec<String> = versus_positions_full.split("\n").map(|x| x.to_string()).collect();
    commands.insert_resource(VersusManager {
        max_think_time_ms: 1000,
        max_game_length: 100,
        game_idx: 0,
        p1_wins: 0,
        p1_losses: 0,
        draws: 0,
        position_fens: positions,
    }); 
}

pub fn start_versus_games(
    mut interaction_query: Query<(&Interaction, &mut BackgroundColor), (Changed<Interaction>, With<MatchManagerStartButton>)>,
    versus_manager: Res<VersusManager>,
    mut zobrist: ResMut<Zobrist>,
    mut board: ResMut<Board>,
    mut manager: ResMut<GameManager>,
) {
    for (interaction, mut _color) in interaction_query.iter_mut() {
        match *interaction {
            Interaction::Pressed => {
                board.load_position(Some(versus_manager.position_fens[0].clone()), zobrist.as_mut());
                manager.move_color = board.move_color;
            },
            Interaction::Hovered => (),
            Interaction::None => (),
        }
    }
}

pub fn versus_game_over(
    mut commands: Commands,
    mut versus_manager: ResMut<VersusManager>,
    mut board: ResMut<Board>,
    mut zobrist: ResMut<Zobrist>,
    mut match_manager_text_query: Query<(&mut Text, &MatchManagerText)>,
) {
    versus_manager.game_idx += 1;

    for (mut text, label) in match_manager_text_query.iter_mut() {
        text.sections[0].value = match label.stat {
            MatchManagerStatistic::GameNumber => format!("Game Number: {} / 1000", versus_manager.game_idx),
            MatchManagerStatistic::Player1Stats(version) => format!("{} | Wins: 0  Losses: 0  Draws: 0", version.label()),
            MatchManagerStatistic::Player2Stats(version) => format!("{} | Wins: 0  Losses: 0  Draws: 0", version.label()),
            MatchManagerStatistic::MaxThinkTime => format!("Max Think Time: {}", versus_manager.max_think_time_ms),
            MatchManagerStatistic::MaxGameLength => format!("Max Game Length: {}", versus_manager.max_game_length),
        }
    }

    board.load_position(Some(versus_manager.position_fens[versus_manager.game_idx].clone()), zobrist.as_mut());

    commands.insert_resource(NextState(Some(AppState::InGame)));
}