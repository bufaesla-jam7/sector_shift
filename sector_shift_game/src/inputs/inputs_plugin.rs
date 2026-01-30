use bevy::prelude::*;

#[cfg(feature = "dev")]
use crate::states::states::DebugHudState;
use crate::{
    inputs::systems::{player_look, player_movement},
    states::system_sets::GameSet,
};

pub struct InputsPlugin;
impl Plugin for InputsPlugin {
    fn build(&self, app: &mut App) {
        #[cfg(feature = "dev")]
        let player_look = player_look.run_if(in_state(DebugHudState::Disabled));
        app.add_systems(
            Update,
            (player_look, player_movement).in_set(GameSet::Running),
        );
    }
}
