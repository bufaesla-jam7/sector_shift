use bevy::prelude::*;

#[derive(States, Reflect, Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub enum AppState {
    #[default]
    LoadAssets,
    InEditor,
}
