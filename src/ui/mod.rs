use bevy::{prelude::*, window::PrimaryWindow};

pub mod board;
pub mod theme;
pub mod menu;

use board::*;
use theme::*;
use menu::*;


pub fn spawn_camera(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>
) {
    let window = window_query.get_single().unwrap();

    commands.spawn(
        Camera2dBundle {
            transform: Transform::from_xyz(window.width() / 2.0, window.height() / 2.0, 0.0),
            ..default()
        }
    );
}

pub fn finish_load_ui(
    mut commands: Commands,
) {
    commands.insert_resource(NextState(Some(AppState::InGame)));
}


pub struct UIPlugin;

impl Plugin for UIPlugin {
    fn build(&self, app: &mut App) {
        app
            .insert_resource(ClearColor(Color::rgb_u8(40, 39, 40)))
            .init_resource::<BoardTheme>()
            .init_resource::<PieceTheme>()
            .init_resource::<BoardUITransform>()
            .add_event::<BoardUIResetPiecePosition>()
            .add_event::<BoardSetSquareColor>()
            .add_systems(Startup, spawn_camera)
            .add_systems(Startup, spawn_main_menu)
            .add_systems(Update, update_menu_buttons.run_if(in_state(AppState::MainMenu)))
            .add_systems(OnExit(AppState::MainMenu), despawn_main_menu)
        
            .add_systems(OnEnter(AppState::LoadUI), (
                init_board_ui_transform,
                init_piece_theme,
                spawn_board_ui,
                finish_load_ui,
            ).chain())
            
            .add_systems(Update, (
                update_pieces,
                reset_piece_position,
                set_square_color,
                update_board_ui,
            ).run_if(in_state(AppState::InGame)));
    }
}