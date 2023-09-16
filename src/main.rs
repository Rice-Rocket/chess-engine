use bevy::prelude::*;

pub mod game_logic;
pub mod ui;

use ui::*;
use game_logic::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_state::<menu::AppState>()
        .add_plugins(GameLogicPlugin)
        .add_plugins(UIPlugin)
        .run()
}
