use bevy::prelude::*;

#[derive(States, Reflect, Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub enum GameState {
    #[default]
    LoadAssets,
    MainMenu,
    SetupGame,
    Running,
}
