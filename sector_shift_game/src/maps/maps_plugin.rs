use bevy::prelude::*;

use crate::{
    maps::{
        functions::{SpawnLevel, process_spawn_level},
        systems::{rotate_skybox, start_game},
    },
    states::{states::GameState, system_sets::GameSystems},
};

pub struct MapsPlugin;
impl Plugin for MapsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::SetupGame), start_game)
            .add_systems(
                Update,
                (process_spawn_level, rotate_skybox).in_set(GameSystems::Running),
            )
            .add_message::<SpawnLevel>();
    }
}
