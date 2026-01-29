use bevy::prelude::*;

use crate::{maps::systems::start_game, states::states::GameState};

pub struct MapsPlugin;
impl Plugin for MapsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::SetupGame), start_game);
    }
}
