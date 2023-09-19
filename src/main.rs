use bevy::prelude::*;

pub mod board;
pub mod game;
pub mod move_gen;
pub mod ui;
pub mod state;
pub mod utils;
pub mod ai;

use ui::*;
use state::*;
use board::*;
use move_gen::*;
use utils::*;
use game::*;
use ai::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                // resolution: (600.0, 400.0).into(),
                resolution: (1280.0, 720.0).into(),
                title: "Chess Engine".to_string(),
                ..default()
            }),
            ..default()
        }))
        .add_state::<AppState>()
        .add_state::<AppMode>()
        .add_plugins(BoardPlugin)
        .add_plugins(MoveGenPlugin)
        .add_plugins(GamePlugin)
        .add_plugins(UIPlugin)
        .add_plugins(AIPlugin)
        .run()
}
