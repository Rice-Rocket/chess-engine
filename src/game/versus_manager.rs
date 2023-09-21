use bevy::prelude::*;

use crate::{state::{AppState, AppMode}, ui::{ingame_menu::{MatchManagerText, MatchManagerStatistic, MatchManagerStartButton}, text_input::TextInput}, board::{board::Board, zobrist::Zobrist, piece::Piece}, ai::ai_player::AIPlayer};

use super::{manager::{GameManager, GameResult, CanMakeMove}, player::Player};


#[derive(Resource)]
pub struct VersusManager {
    pub max_think_time_ms: usize,
    pub max_game_length: usize,
    pub total_games: usize,
    pub game_idx: usize,
    pub white_wins: usize,
    pub white_losses: usize,
    pub draws: usize,
    position_fens: Vec<String>,
}

pub fn spawn_versus_manager(
    mut commands: Commands,
    app_mode: Res<State<AppMode>>,
    mut board: ResMut<Board>,
    mut zobrist: ResMut<Zobrist>,
) {
    if app_mode.clone() == AppMode::GameAIAI {
        let versus_positions_full = std::fs::read_to_string("assets/logic/versus_positions.txt").unwrap();
        let positions: Vec<String> = versus_positions_full.split("\n").map(|x| x.to_string()).collect();
        board.load_position(Some(positions[0].clone()), zobrist.as_mut());
        commands.insert_resource(VersusManager {
            max_think_time_ms: 1000,
            max_game_length: 100,
            total_games: 1000,
            game_idx: 0,
            white_wins: 0,
            white_losses: 0,
            draws: 0,
            position_fens: positions,
        }); 
    } else {
        commands.insert_resource(VersusManager {
            max_think_time_ms: 0,
            max_game_length: 0,
            total_games: 0,
            game_idx: 0,
            white_wins: 0,
            white_losses: 0,
            draws: 0,
            position_fens: Vec::new(),
        });
    }
}

pub fn start_versus_games(
    mut interaction_query: Query<(&Interaction, &mut BackgroundColor), (Changed<Interaction>, With<MatchManagerStartButton>)>,
    versus_manager: ResMut<VersusManager>,
    mut match_manager_text_query: Query<(&MatchManagerText, &mut Text)>,
    mut match_text_input_query: Query<&mut TextInput, (With<Text>, With<Button>)>,
    mut zobrist: ResMut<Zobrist>,
    mut board: ResMut<Board>,
    mut manager: ResMut<GameManager>,
    mut can_make_move_evw: EventWriter<CanMakeMove>,
    mut ai_player_query: Query<&mut AIPlayer>,
) {
    for (interaction, mut _color) in interaction_query.iter_mut() {
        match *interaction {
            Interaction::Pressed => {
                for (label, mut text) in match_manager_text_query.iter_mut() {
                    match label.stat {
                        MatchManagerStatistic::GameNumber => {
                            text.sections[0].value = format!("Game Number: 1 / {}", versus_manager.total_games)
                        },
                        _ => (),
                    }
                }
                for mut match_text_input in match_text_input_query.iter_mut() {
                    match_text_input.inactive = true;
                }
                
                for mut ai_player in ai_player_query.iter_mut() {
                    ai_player.think_time_ms = versus_manager.max_think_time_ms as u32;
                }

                board.load_position(Some(versus_manager.position_fens[0].clone()), zobrist.as_mut());
                manager.move_color = board.move_color;
                can_make_move_evw.send(CanMakeMove {});
            },
            Interaction::Hovered => (),
            Interaction::None => (),
        }
    }
}

pub fn versus_update(
    mut commands: Commands,
    mut manager: ResMut<GameManager>,
    mut match_manager_text_query: Query<(&MatchManagerText, &TextInput)>,
    mut versus_manager: ResMut<VersusManager>,
) {
    if manager.game_moves.len() > versus_manager.max_game_length {
        manager.game_result = GameResult::DrawByArbiter;
        commands.insert_resource(NextState(Some(AppState::GameOver)));
    }
    for (label, text_input) in match_manager_text_query.iter_mut() {
        match label.stat {
            MatchManagerStatistic::MaxThinkTime => {
                versus_manager.max_think_time_ms = text_input.value.parse().unwrap();
            },
            MatchManagerStatistic::MaxGameLength => {
                versus_manager.max_game_length = text_input.value.parse().unwrap();
            },
            MatchManagerStatistic::TotalGames => {
            versus_manager.total_games = text_input.value.parse().unwrap();
            },
            _ => (),
        }
    }
}

pub fn versus_game_over(
    mut commands: Commands,
    mut versus_manager: ResMut<VersusManager>,
    mut board: ResMut<Board>,
    mut zobrist: ResMut<Zobrist>,
    mut match_manager_text_query: Query<(&mut Text, &mut MatchManagerText, Option<&TextInput>)>,
    mut manager: ResMut<GameManager>,
    mut can_make_move_evw: EventWriter<CanMakeMove>,
    mut ai_player_query: Query<&mut Player, With<AIPlayer>>,
) {
    versus_manager.game_idx += 1;
    if versus_manager.game_idx == versus_manager.total_games {
        return;
    }
    let mut swapped_colors = false;
    if versus_manager.game_idx == versus_manager.total_games / 2 {
        for mut player in ai_player_query.iter_mut() {
            if player.team == Piece::WHITE {
                player.team = Piece::BLACK;
                if !swapped_colors {
                    let temp = versus_manager.white_wins;
                    versus_manager.white_wins = versus_manager.white_losses;
                    versus_manager.white_losses = temp;
                    swapped_colors = true;
                }
            } else if player.team == Piece::BLACK {
                player.team = Piece::WHITE;
                if !swapped_colors {
                    let temp = versus_manager.white_wins;
                    versus_manager.white_wins = versus_manager.white_losses;
                    versus_manager.white_losses = temp;
                    swapped_colors = true;
                }
            }
        }
    }

    match manager.game_result {
        GameResult::None | GameResult::Playing | GameResult::BlackTimeout | GameResult::WhiteTimeout => (),
        GameResult::WhiteIsMated => { versus_manager.white_losses += 1; },
        GameResult::BlackIsMated => { versus_manager.white_wins += 1; },
        GameResult::Stalemate | GameResult::Repetition | GameResult::FiftyMoveRule | GameResult::InsufficientMaterial | GameResult::DrawByArbiter => { versus_manager.draws += 1; },
    }

    for (mut text, mut label, text_input) in match_manager_text_query.iter_mut() {
        match label.stat {
            MatchManagerStatistic::GameNumber => {
                text.sections[0].value = format!("Game Number: {} / {}", versus_manager.game_idx + 1, versus_manager.total_games);
            },
            MatchManagerStatistic::Player1Stats(version, mut is_white) => {
                is_white = if swapped_colors { !is_white } else { is_white };
                label.stat = MatchManagerStatistic::Player1Stats(version, is_white);
                let wins = if is_white { versus_manager.white_wins } else { versus_manager.white_losses };
                let losses = if is_white { versus_manager.white_losses } else { versus_manager.white_wins };
                text.sections[0].value = format!("{} | Wins: {}  Losses: {}  Draws: {}", version.label(), wins, losses, versus_manager.draws);
            },
            MatchManagerStatistic::Player2Stats(version, mut is_white) => {
                is_white = if swapped_colors { !is_white } else { is_white };
                label.stat = MatchManagerStatistic::Player2Stats(version, is_white);
                let wins = if is_white { versus_manager.white_wins } else { versus_manager.white_losses };
                let losses = if is_white { versus_manager.white_losses } else { versus_manager.white_wins };
                text.sections[0].value = format!("{} | Wins: {}  Losses: {}  Draws: {}", version.label(), wins, losses, versus_manager.draws);
            },
            MatchManagerStatistic::MaxThinkTime => {
                versus_manager.max_think_time_ms = text_input.unwrap().value.parse().unwrap();
            },
            MatchManagerStatistic::MaxGameLength => {
                versus_manager.max_game_length = text_input.unwrap().value.parse().unwrap();
            },
            MatchManagerStatistic::TotalGames => {
                versus_manager.total_games = text_input.unwrap().value.parse().unwrap();
            },
            MatchManagerStatistic::BlackPlayer(mut version) => {
                if swapped_colors {
                    label.stat = MatchManagerStatistic::BlackPlayer(version.opponent().unwrap());
                    version = version.opponent().unwrap();
                }
                text.sections[0].value = format!("Black: {}", version.label());
            },
            MatchManagerStatistic::WhitePlayer(mut version) => {
                if swapped_colors {
                    label.stat = MatchManagerStatistic::WhitePlayer(version.opponent().unwrap());
                    version = version.opponent().unwrap();
                }
                text.sections[0].value = format!("White: {}", version.label());

            },
        };
    }

    board.load_position(Some(versus_manager.position_fens[versus_manager.game_idx % (versus_manager.total_games / 2)].clone()), zobrist.as_mut());
    manager.move_color = board.move_color;
    manager.game_moves.clear();
    commands.insert_resource(NextState(Some(AppState::InGame)));
    can_make_move_evw.send(CanMakeMove {});
}