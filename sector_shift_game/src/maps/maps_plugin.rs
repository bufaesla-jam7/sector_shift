use bevy::prelude::*;

use crate::{
    maps::systems::{rotate_skybox, start_game},
    states::{states::GameState, system_sets::GameSet},
};

pub struct MapsPlugin;
impl Plugin for MapsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::SetupGame), start_game)
            .add_systems(Update, rotate_skybox.in_set(GameSet::Running));
    }
}
