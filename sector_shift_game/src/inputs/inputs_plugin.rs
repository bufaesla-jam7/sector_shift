use bevy::prelude::*;

use crate::{
    inputs::systems::{player_look, player_movement},
    states::system_sets::GameSet,
};

pub struct InputsPlugin;
impl Plugin for InputsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (player_look, player_movement).in_set(GameSet::Running),
        );
    }
}
