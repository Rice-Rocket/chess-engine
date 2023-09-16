use bevy::prelude::*;
use crate::{AppState, game::manager::PlayerType};


#[derive(Component)]
pub struct GameType {
    pub white: PlayerType,
    pub black: PlayerType,
}


const BUTTON_COLOR: Color = Color::rgb(0.95, 0.93, 0.9);
const BUTTON_BORDER_COLOR: Color = Color::rgb(0.8, 0.79, 0.77);
const BUTTON_TEXT_COLOR: Color = Color::rgb(0.53, 0.49, 0.48);
const BUTTON_REST_LENGTH: f32 = 300.0;
const BUTTON_HOVER_LENGTH: f32 = 450.0;
const BUTTON_STRETCH_TIME_SECS: f32 = 0.25;

pub enum MainMenuButtonLabel {
    HumanVsHuman,
    HumanVsAI,
    AIVsAI,
}

#[derive(Component)]
pub struct MainMenuButton {
    anim_time: f32,
    width: f32,
    label: MainMenuButtonLabel,
}

#[derive(Component)]
pub struct MainMenuParentNode {}


pub fn spawn_main_menu(
    mut commands: Commands,
    asset_server: Res<AssetServer>
) {
    commands.spawn((NodeBundle {
        style: Style {
            width: Val::Percent(100.0),
            align_items: AlignItems::FlexStart,
            justify_content: JustifyContent::Center,
            flex_direction: FlexDirection::Column,
            padding: UiRect {
                left: Val::Px(20.0),
                ..default()
            },
            ..default()
        },
        ..default()
    }, MainMenuParentNode {}))
        .with_children(|parent| {
            parent.spawn((ButtonBundle {
                    style: Style {
                        width: Val::Px(BUTTON_REST_LENGTH),
                        height: Val::Px(60.0),
                        border: UiRect::all(Val::Px(5.0)),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    border_color: BorderColor(BUTTON_BORDER_COLOR),
                    background_color: BUTTON_COLOR.into(),
                    ..default()
                }, MainMenuButton { anim_time: 0.0, width: BUTTON_REST_LENGTH, label: MainMenuButtonLabel::HumanVsHuman }))
                .with_children(|parent| {
                    parent.spawn(TextBundle::from_section(
                        "Human vs Human",
                        TextStyle {
                            font: asset_server.load("ui/font/Monaco.ttf"),
                            font_size: 30.0,
                            color: BUTTON_TEXT_COLOR
                        }
                    ));
                });
            parent.spawn((ButtonBundle {
                    style: Style {
                        width: Val::Px(BUTTON_REST_LENGTH),
                        height: Val::Px(60.0),
                        border: UiRect::all(Val::Px(5.0)),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        margin: UiRect::top(Val::Px(10.0)),
                        ..default()
                    },
                    border_color: BorderColor(BUTTON_BORDER_COLOR),
                    background_color: BUTTON_COLOR.into(),
                    ..default()
                }, MainMenuButton { anim_time: 0.0, width: BUTTON_REST_LENGTH, label: MainMenuButtonLabel::HumanVsAI }))
                .with_children(|parent| {
                    parent.spawn(TextBundle::from_section(
                        "Human vs AI",
                        TextStyle {
                            font: asset_server.load("ui/font/Monaco.ttf"),
                            font_size: 30.0,
                            color: BUTTON_TEXT_COLOR
                        }
                    ));
                });
            parent.spawn((ButtonBundle {
                    style: Style {
                        width: Val::Px(BUTTON_REST_LENGTH),
                        height: Val::Px(60.0),
                        border: UiRect::all(Val::Px(5.0)),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        margin: UiRect::top(Val::Px(10.0)),
                        ..default()
                    },
                    border_color: BorderColor(BUTTON_BORDER_COLOR),
                    background_color: BUTTON_COLOR.into(),
                    ..default()
                }, MainMenuButton { anim_time: 0.0, width: BUTTON_REST_LENGTH, label: MainMenuButtonLabel::AIVsAI }))
                .with_children(|parent| {
                    parent.spawn(TextBundle::from_section(
                        "AI vs AI",
                        TextStyle {
                            font: asset_server.load("ui/font/Monaco.ttf"),
                            font_size: 30.0,
                            color: BUTTON_TEXT_COLOR
                        }
                    ));
                });
        });
}

pub fn despawn_main_menu(
    mut commands: Commands,
    menu_button_query: Query<Entity, With<MainMenuParentNode>>,
) {
    for parent_node_entity in menu_button_query.iter() {
        commands.entity(parent_node_entity).despawn();
    }
}

pub fn update_menu_buttons(
    mut commands: Commands,
    mut interaction_query: Query<(&Interaction, &mut Style, &mut MainMenuButton), With<Button>>,
    time: Res<Time>
) {
    for (interaction, mut style, mut button_data) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                if button_data.anim_time < 1.0 {
                    button_data.anim_time += time.delta_seconds() / BUTTON_STRETCH_TIME_SECS;
                }

                match button_data.label {
                    MainMenuButtonLabel::HumanVsHuman => {
                        commands.spawn(GameType {
                            white: PlayerType::Human,
                            black: PlayerType::Human,
                        });
                    },
                    MainMenuButtonLabel::HumanVsAI => {
                        commands.spawn(GameType {
                            white: PlayerType::Human,
                            black: PlayerType::AI,
                        });
                    },
                    MainMenuButtonLabel::AIVsAI => {
                        commands.spawn(GameType {
                            white: PlayerType::AI,
                            black: PlayerType::AI,
                        });
                    },
                }

                commands.insert_resource(NextState(Some(AppState::LoadPrecomp)));
            },
            Interaction::Hovered => {
                if button_data.anim_time < 1.0 {
                    button_data.anim_time += time.delta_seconds() / BUTTON_STRETCH_TIME_SECS;
                }
            },
            Interaction::None => {
                if button_data.anim_time > 0.0 {
                    button_data.anim_time -= time.delta_seconds() / BUTTON_STRETCH_TIME_SECS;
                } else {
                    button_data.anim_time = 0.0;
                }
            }
        }
        let step = button_data.anim_time * button_data.anim_time * (3.0 - 2.0 * button_data.anim_time);
        button_data.width = (BUTTON_HOVER_LENGTH - BUTTON_REST_LENGTH) * step + BUTTON_REST_LENGTH;
        style.width = Val::Px(button_data.width);
    }
}