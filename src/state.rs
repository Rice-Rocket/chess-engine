use bevy::prelude::*;


#[derive(States, Debug, Hash, Eq, PartialEq, Clone, Copy, Default)]
pub enum AppState {
    #[default]
    MainMenu,
    LoadPrecomp,
    LoadZobrist,
    LoadBoard,
    LoadMoveGen,
    LoadGame,
    LoadUI,
    LoadAI,
    InGame,
    GameOver,
}


#[derive(States, Debug, Hash, Eq, PartialEq, Clone, Copy, Default)]
pub enum AppMode {
    #[default]
    None,
    GameHumanHuman,
    GameHumanAI,
    GameAIAI,
}