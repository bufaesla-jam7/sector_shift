use bevy::prelude::*;
use sector_shift_core::prelude::*;

#[cfg(feature = "dev")]
use crate::states::states::DebugHudState;
use crate::states::{
    states::GameState,
    system_sets::GameSystems,
    systems::{set_game_state_running, set_game_state_setup_game},
};

pub struct StatesPlugin;
impl Plugin for StatesPlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<GameState>();
        #[cfg(feature = "dev")]
        app.init_state::<DebugHudState>();

        app.configure_sets(
            Update,
            GameSystems::LoadAssets.run_if(in_state(GameState::LoadAssets)),
        );

        app.configure_sets(
            Update,
            GameSystems::SetupGame.run_if(in_state(GameState::SetupGame)),
        );

        app.configure_sets(
            Update,
            GameSystems::Running.run_if(in_state(GameState::Running)),
        );

        app.add_systems(
            Update,
            set_game_state_setup_game.in_set(GameSystems::LoadAssets).run_if(all_assets_loaded),
        );

        app.add_systems(
            Update,
            set_game_state_running.in_set(GameSystems::SetupGame),
        );
    }
}
