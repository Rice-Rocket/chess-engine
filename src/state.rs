use bevy::prelude::*;


#[derive(States, Debug, Hash, Eq, PartialEq, Clone, Copy, Default)]
pub enum AppState {
    #[default]
    MainMenu,
    LoadPrecomp,
    LoadBoard,
    LoadMoveGen,
    LoadGame,
    LoadUI,
    InGame,
}


#[derive(States, Debug, Hash, Eq, PartialEq, Clone, Copy, Default)]
pub enum AppMode {
    #[default]
    GameHumanHuman,
    GameHumanAI,
    GameAIAI,
}