use bevy::prelude::*;

#[derive(States, Reflect, Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DebugHudState {
    #[default]
    Disabled,
    Enabled,
}
