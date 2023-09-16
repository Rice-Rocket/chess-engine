use bevy::prelude::*;

pub mod board;
pub mod game;
pub mod move_gen;
pub mod ui;
pub mod state;
pub mod utils;

use ui::*;
use state::*;
use board::*;
use move_gen::*;
use utils::*;
use game::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_state::<AppState>()
        .add_state::<AppMode>()
        .add_plugins(BoardPlugin)
        .add_plugins(MoveGenPlugin)
        .add_plugins(GamePlugin)
        .add_plugins(UIPlugin)
        .run()
}
