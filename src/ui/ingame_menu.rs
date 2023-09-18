use bevy::prelude::*;

use crate::game::manager::{GameManager, GameResult};

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
}

impl MenuStatistic {
    pub const DEFAULT_COLOR: Color = Color::rgb(0.53, 0.49, 0.48);
    pub const MOVE_GEN_TIME_COLOR: Color = Color::rgb(0.82, 0.36, 0.37);
    pub const COLOR_2: Color = Color::rgb(0.77, 0.55, 0.33);
    pub const COLOR_3: Color = Color::rgb(0.69, 0.92, 0.46);
    pub const COLOR_4: Color = Color::rgb(0.58, 0.76, 0.93);
    pub const COLOR_5: Color = Color::rgb(0.70, 0.53, 0.90);
    pub const COLOR_6: Color = Color::rgb(0.49, 0.24, 0.81);
}

#[derive(Component)]
pub struct StatMenuText {
    pub stat: MenuStatistic,
}

#[derive(Resource)]
pub struct CalcStatistics {
    pub move_gen_time: f32,
}

impl Default for CalcStatistics {
    fn default() -> Self {
        CalcStatistics {
            move_gen_time: 0.0,
        }
    }
}


pub fn spawn_calc_stats(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
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
            parent.spawn((TextBundle::from_section(
                "Move gen time: N/A",
                TextStyle {
                    font: asset_server.load("ui/font/Monaco.ttf"),
                    font_size: 15.0,
                    color: MenuStatistic::MOVE_GEN_TIME_COLOR,
                }
            ), StatMenuText { stat: MenuStatistic::MoveGenTime }));
        });
}

pub fn update_menu_stats(
    calc_stats: Res<CalcStatistics>,
    mut menu_text_query: Query<(&mut Text, &StatMenuText)>,
) {
    for (mut text, stat) in menu_text_query.iter_mut() {
        text.sections[0].value = match stat.stat {
            MenuStatistic::MoveGenTime => { format!("Move gen time: {} micros", calc_stats.move_gen_time) }
        }
    }
}

pub fn spawn_game_over_splash(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    game_manager: Res<GameManager>,
) {
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
                    font: asset_server.load("ui/font/Monaco.ttf"),
                    font_size: 40.0,
                    color: GameOverSplash::TITLE_COLOR,
                }
            ));
        })
        .with_children(|parent| {
            parent.spawn(TextBundle::from_section(
                subtitle_text,
                TextStyle {
                    font: asset_server.load("ui/font/Monaco.ttf"),
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