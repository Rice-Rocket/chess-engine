use bevy::prelude::*;

use crate::{game::{manager::{GameManager, GameResult, PlayerType}, player::Player}, ai::ai_player::{AIPlayer, AIVersion}, board::piece::Piece};

use super::text_input::TextInput;

#[derive(Component)]
pub struct StatMenuParentNode {}

#[derive(Component)]
pub struct GameOverSplash {}

impl GameOverSplash {
    pub const TITLE_COLOR: Color = Color::rgb(0.95, 0.95, 0.95);
    pub const SUBTITLE_COLOR: Color = Color::rgb(0.68, 0.68, 0.68);
}

pub enum MenuStatistic {
    MoveGenTime,
    AIDepth,
    AIPositionsEvaluated,
    AIMatesFound,
    AIEvaluation,
    AIThinkTime,
}

impl MenuStatistic {
    pub const DEFAULT_COLOR: Color = Color::rgb(0.53, 0.49, 0.48);
    pub const RED: Color = Color::rgb(0.82, 0.36, 0.37);
    pub const ORANGE: Color = Color::rgb(0.77, 0.55, 0.33);
    pub const GREEN: Color = Color::rgb(0.69, 0.92, 0.46);
    pub const BLUE: Color = Color::rgb(0.58, 0.76, 0.93);
    pub const PURPLE: Color = Color::rgb(0.70, 0.53, 0.90);
    pub const DARK_PURPLE: Color = Color::rgb(0.49, 0.24, 0.81);
}

pub enum MatchManagerStatistic {
    GameNumber,
    Player1Stats(AIVersion, bool),
    Player2Stats(AIVersion, bool),
    MaxThinkTime,
    MaxGameLength,
    TotalGames,
    BlackPlayer(AIVersion),
    WhitePlayer(AIVersion),
}

impl MatchManagerStatistic {
    pub const DEFAULT_COLOR: Color = Color::rgb(0.95, 0.95, 0.95);
    pub const MATCH_MANAGER_TITLE_COLOR: Color = Color::rgb(0.97, 0.26, 0.26);
    pub const MATCH_MANAGER_SUBTITLE_COLOR: Color = Color::rgb(0.12, 0.89, 0.23);
}

#[derive(Component)]
pub struct StatMenuText {
    pub stat: MenuStatistic,
}

#[derive(Component)]
pub struct MatchManagerText {
    pub stat: MatchManagerStatistic,
}

#[derive(Component)]
pub struct MatchManagerStartButton {}

#[derive(Resource)]
pub struct CalcStatistics {
    pub move_gen_time: f32,
    pub ai_depth: i32,
    pub ai_positions_evaled: u32,
    pub ai_eval: f32,
    pub ai_think_time: u32,
    pub ai_mates_found: i32,
}

impl Default for CalcStatistics {
    fn default() -> Self {
        CalcStatistics {
            move_gen_time: 0.0,
            ai_depth: 0,
            ai_positions_evaled: 0,
            ai_eval: 0.0,
            ai_think_time: 0,
            ai_mates_found: 0,
        }
    }
}


pub fn spawn_calc_stats(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    manager: Res<GameManager>,
) {
    if manager.white_player_type == PlayerType::AI && manager.black_player_type == PlayerType::AI {
        return;
    }
    commands.spawn((NodeBundle {
        style: Style {
            width: Val::Percent(100.0),
            align_items: AlignItems::FlexStart,
            justify_content: JustifyContent::Center,
            flex_direction: FlexDirection::Column,
            padding: UiRect {
                left: Val::Percent(2.0),
                ..default()
            },
            ..default()
        },
        ..default()
    }, StatMenuParentNode {}))
        .with_children(|parent| {
            // parent.spawn((TextBundle::from_section(
            //     "Move gen time: N/A",
            //     TextStyle {
            //         font: asset_server.load("ui/font/LiberationSans-Regular.ttf"),
            //         font_size: 20.0,
            //         color: MenuStatistic::MOVE_GEN_TIME_COLOR,
            //     }
            // ), StatMenuText { stat: MenuStatistic::MoveGenTime }));
            if manager.white_player_type != manager.black_player_type {
                parent.spawn((TextBundle::from_section(
                    "Search Depth: N/A",
                    TextStyle {
                        font: asset_server.load("ui/font/LiberationSans-Regular.ttf"),
                        font_size: 20.0,
                        color: MenuStatistic::RED,
                    }
                ), StatMenuText { stat: MenuStatistic::AIDepth }));
                parent.spawn((TextBundle::from_section(
                    "Evaluation: N/A",
                    TextStyle {
                        font: asset_server.load("ui/font/LiberationSans-Regular.ttf"),
                        font_size: 20.0,
                        color: MenuStatistic::ORANGE,
                    }
                ), StatMenuText { stat: MenuStatistic::AIEvaluation }));
                parent.spawn((TextBundle::from_section(
                    "Positions Evaluated: N/A",
                    TextStyle {
                        font: asset_server.load("ui/font/LiberationSans-Regular.ttf"),
                        font_size: 20.0,
                        color: MenuStatistic::GREEN,
                    }
                ), StatMenuText { stat: MenuStatistic::AIPositionsEvaluated }));
                parent.spawn((TextBundle::from_section(
                    "Checkmates Found: N/A",
                    TextStyle {
                        font: asset_server.load("ui/font/LiberationSans-Regular.ttf"),
                        font_size: 20.0,
                        color: MenuStatistic::BLUE,
                    }
                ), StatMenuText { stat: MenuStatistic::AIMatesFound }));
                parent.spawn((TextBundle::from_section(
                    "Think Time: N/A",
                    TextStyle {
                        font: asset_server.load("ui/font/LiberationSans-Regular.ttf"),
                        font_size: 20.0,
                        color: MenuStatistic::PURPLE,
                    }
                ), StatMenuText { stat: MenuStatistic::AIThinkTime }));
            }
        });
}

pub fn update_menu_stats(
    calc_stats: Res<CalcStatistics>,
    mut menu_text_query: Query<(&mut Text, &StatMenuText)>,
) {
    for (mut text, stat) in menu_text_query.iter_mut() {
        text.sections[0].value = match stat.stat {
            MenuStatistic::MoveGenTime => { format!("Move gen time: {} micros", calc_stats.move_gen_time) },
            MenuStatistic::AIDepth => { format!("Search Depth: {}", calc_stats.ai_depth) },
            MenuStatistic::AIEvaluation => { format!("Evaluation: {}", calc_stats.ai_eval) },
            MenuStatistic::AIPositionsEvaluated => { format!("Positions Evaluated: {}", calc_stats.ai_positions_evaled) },
            MenuStatistic::AIThinkTime => { format!("Think Time: {} ms", calc_stats.ai_think_time) },
            MenuStatistic::AIMatesFound => { format!("Checkmates Found: {}", calc_stats.ai_mates_found) },
        }
    }
}

pub fn spawn_game_over_splash(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    game_manager: Res<GameManager>,
) {
    // let 
    let (title_text, subtitle_text) = match game_manager.game_result {
        GameResult::None => ("", ""),
        GameResult::Playing => ("", ""),
        GameResult::WhiteIsMated => ("Checkmate", "Black Wins"),
        GameResult::BlackIsMated => ("Checkmate", "Wins Wins"),
        GameResult::Stalemate => ("Stalemate", "Draw"),
        GameResult::Repetition => ("Repitition", "Draw"),
        GameResult::FiftyMoveRule => ("Fifty Move Rule", "Draw"),
        GameResult::InsufficientMaterial => ("Insufficient Material", "Draw"),
        GameResult::DrawByArbiter => ("", ""),
        GameResult::WhiteTimeout => ("Timeout", "Black Wins"),
        GameResult::BlackTimeout => ("Timeout", "White Wins"),
    };
    commands.spawn((NodeBundle {
        style: Style {
            width: Val::Percent(100.0),
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            flex_direction: FlexDirection::Column,
            padding: UiRect {
                left: Val::Percent(50.0),
                right: Val::Percent(5.0),
                ..default()
            },
            ..default()
        },
        ..default()
    }, GameOverSplash {}))
        .with_children(|parent| {
            parent.spawn(TextBundle::from_section(
                title_text,
                TextStyle {
                    font: asset_server.load("ui/font/LiberationSans-Regular.ttf"),
                    font_size: 40.0,
                    color: GameOverSplash::TITLE_COLOR,
                }
            ));
        })
        .with_children(|parent| {
            parent.spawn(TextBundle::from_section(
                subtitle_text,
                TextStyle {
                    font: asset_server.load("ui/font/LiberationSans-Regular.ttf"),
                    font_size: 20.0,
                    color: GameOverSplash::SUBTITLE_COLOR,
                }
            ));
        });
}

pub fn despawn_game_over_splash(
    mut commands: Commands,
    game_over_splash_query: Query<Entity, With<GameOverSplash>>,
) {
    for game_over_splash_entity in game_over_splash_query.iter() {
        commands.entity(game_over_splash_entity).despawn();
    }
}

pub fn spawn_ai_vs_ai_menu(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    ai_player_query: Query<(&AIPlayer, &Player)>,    
) {
    let mut parent_node = commands.spawn((NodeBundle {
        style: Style {
            width: Val::Percent(100.0),
            align_items: AlignItems::FlexStart,
            justify_content: JustifyContent::FlexStart,
            flex_direction: FlexDirection::Column,
            padding: UiRect {
                left: Val::Percent(1.0),
                top: Val::Percent(1.0),
                ..default()
            },
            ..default()
        },
        ..default()
    }, StatMenuParentNode {}));
    parent_node.with_children(|parent| {
        parent.spawn(TextBundle::from_section(
            "Versus Manager",
            TextStyle {
                font: asset_server.load("ui/font/LiberationSans-Bold.ttf"),
                font_size: 30.0,
                color: MatchManagerStatistic::MATCH_MANAGER_TITLE_COLOR,
            }
        ).with_style(Style {
            padding: UiRect {
                bottom: Val::Percent(2.0),
                ..default()
            },
            ..default()
        }));
    });
    
    let (mut p1_version, mut p1_team) = (AIVersion::V0, Piece::WHITE);
    let mut p2_version = AIVersion::V0;
    for (i, (ai_player, player)) in ai_player_query.iter().enumerate() {
        if i == 0 {
            p1_version = ai_player.version;
            p1_team = player.team;
        } else {
            p2_version = ai_player.version;
        }
    }

    parent_node.with_children(|parent| {
        parent.spawn(TextBundle::from_sections([
            TextSection::new(
                format!("{}", p1_version.label()),
                TextStyle {
                    font: asset_server.load("ui/font/LiberationSans-Bold.ttf"),
                    font_size: 20.0,
                    color: MatchManagerStatistic::MATCH_MANAGER_SUBTITLE_COLOR,
                },
            ),
            TextSection::new(
                "  vs  ",
                TextStyle {
                    font: asset_server.load("ui/font/LiberationSans-Regular.ttf"),
                    font_size: 20.0,
                    color: GameOverSplash::SUBTITLE_COLOR,
                },
            ),
            TextSection::new(
                format!("{}", p2_version.label()),
                TextStyle {
                    font: asset_server.load("ui/font/LiberationSans-Bold.ttf"),
                    font_size: 20.0,
                    color: MatchManagerStatistic::MATCH_MANAGER_SUBTITLE_COLOR,
                },
            ),
        ]).with_style(Style {
            padding: UiRect {
                bottom: Val::Percent(5.0),
                ..default()
            },
            ..default()
        }));

        parent.spawn((TextBundle::from_section(
            "Game Number: 0 / 1000",
            TextStyle {
                font: asset_server.load("ui/font/LiberationSans-Regular.ttf"),
                font_size: 20.0,
                color: MatchManagerStatistic::DEFAULT_COLOR,
            }
        ).with_style(Style {
            padding: UiRect {
                bottom: Val::Percent(2.0),
                ..default()
            },
            ..default()
        }), MatchManagerText { stat: MatchManagerStatistic::GameNumber }));
        parent.spawn((TextBundle::from_section(
            format!("{} | Wins: 0  Losses: 0  Draws: 0", p1_version.label()),
            TextStyle {
                font: asset_server.load("ui/font/LiberationSans-Regular.ttf"),
                font_size: 20.0,
                color: MatchManagerStatistic::DEFAULT_COLOR,
            }
        ).with_style(Style {
            padding: UiRect {
                bottom: Val::Percent(2.0),
                ..default()
            },
            ..default()
        }), MatchManagerText { stat: MatchManagerStatistic::Player1Stats(p1_version, p1_team == Piece::WHITE) }));
        parent.spawn((TextBundle::from_section(
            format!("{} | Wins: 0  Losses: 0  Draws: 0", p2_version.label()),
            TextStyle {
                font: asset_server.load("ui/font/LiberationSans-Regular.ttf"),
                font_size: 20.0,
                color: MatchManagerStatistic::DEFAULT_COLOR,
            }
        ).with_style(Style {
            padding: UiRect {
                bottom: Val::Percent(5.0),
                ..default()
            },
            ..default()
        }), MatchManagerText { stat: MatchManagerStatistic::Player2Stats(p2_version, p1_team == Piece::BLACK) }));

        parent.spawn(TextBundle::from_section(
            "Settings",
            TextStyle {
                font: asset_server.load("ui/font/LiberationSans-Regular.ttf"),
                font_size: 20.0,
                color: MatchManagerStatistic::DEFAULT_COLOR,
            }
        ).with_style(Style {
            padding: UiRect {
                bottom: Val::Percent(2.0),
                ..default()
            },
            ..default()
        }));

        parent.spawn((ButtonBundle {
            style: Style {
                width: Val::Percent(25.0),
                height: Val::Percent(5.0),
                top: Val::Percent(0.0),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            background_color: BackgroundColor(Color::rgb(0.95, 0.95, 0.95)),
            ..default()
        }, TextInput::new("Max Think Time: ", "100", " ms", true),
        MatchManagerText { stat: MatchManagerStatistic::MaxThinkTime })).with_children(|parent| {
            parent.spawn(TextBundle::from_section(
                "Max Think Time: ",
                TextStyle {
                    font: asset_server.load("ui/font/LiberationSans-Regular.ttf"),
                    font_size: 20.0,
                    color: Color::rgb(0.4, 0.4, 0.4),
                }
            ));
        });
        parent.spawn((ButtonBundle {
            style: Style {
                width: Val::Percent(25.0),
                height: Val::Percent(5.0),
                top: Val::Percent(1.5),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            background_color: BackgroundColor(Color::rgb(0.95, 0.95, 0.95)),
            ..default()
        }, TextInput::new("Max Game Length: ", "100", " moves", true),
        MatchManagerText { stat: MatchManagerStatistic::MaxGameLength })).with_children(|parent| {
            parent.spawn(TextBundle::from_section(
                "Max Game Length: ",
                TextStyle {
                    font: asset_server.load("ui/font/LiberationSans-Regular.ttf"),
                    font_size: 20.0,
                    color: Color::rgb(0.4, 0.4, 0.4),
                }
            ));
        });
        parent.spawn((ButtonBundle {
            style: Style {
                width: Val::Percent(25.0),
                height: Val::Percent(5.0),
                top: Val::Percent(3.0),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            background_color: BackgroundColor(Color::rgb(0.95, 0.95, 0.95)),
            ..default()
        }, TextInput::new("Total Games: ", "1000", "", true),
        MatchManagerText { stat: MatchManagerStatistic::TotalGames })).with_children(|parent| {
            parent.spawn(TextBundle::from_section(
                "Total Games: ",
                TextStyle {
                    font: asset_server.load("ui/font/LiberationSans-Regular.ttf"),
                    font_size: 20.0,
                    color: Color::rgb(0.4, 0.4, 0.4),
                }
            ));
        });

        parent.spawn((ButtonBundle {
            style: Style {
                width: Val::Percent(30.0),
                height: Val::Percent(5.0),
                top: Val::Percent(7.0),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            background_color: BackgroundColor(Color::rgb(0.95, 0.95, 0.95)),
            ..default()
        }, MatchManagerStartButton {})).with_children(|parent| {
            parent.spawn(TextBundle::from_section(
                "Start Match",
                TextStyle {
                    font: asset_server.load("ui/font/LiberationSans-Regular.ttf"),
                    font_size: 30.0,
                    color: Color::rgb(0.4, 0.4, 0.4),
                }
            ));
        });

        parent.spawn((TextBundle::from_section(
            format!("Black: {}", if p1_team == Piece::BLACK { p1_version.label() } else { p2_version.label() }),
            TextStyle {
                font: asset_server.load("ui/font/LiberationSans-Regular.ttf"),
                font_size: 25.0,
                color: MatchManagerStatistic::DEFAULT_COLOR,
            }
        ).with_style(Style {
            bottom: Val::Percent(53.0),
            left: Val::Percent(43.0),
            width: Val::Percent(30.0),
            ..default()
        }), MatchManagerText { stat: MatchManagerStatistic::BlackPlayer(if p1_team == Piece::BLACK { p1_version } else { p2_version}) }));
        parent.spawn((TextBundle::from_section(
            format!("White: {}", if p1_team == Piece::WHITE { p1_version.label() } else { p2_version.label() }),
            TextStyle {
                font: asset_server.load("ui/font/LiberationSans-Regular.ttf"),
                font_size: 25.0,
                color: MatchManagerStatistic::DEFAULT_COLOR,
            }
        ).with_style(Style {
            top: Val::Percent(37.5),
            left: Val::Percent(43.0),
            width: Val::Percent(30.0),
            ..default()
        }), MatchManagerText { stat: MatchManagerStatistic::WhitePlayer(if p1_team == Piece::WHITE { p1_version } else { p2_version}) }));
    });
}