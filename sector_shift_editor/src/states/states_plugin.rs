use bevy::prelude::*;
use sector_shift_core::prelude::all_assets_loaded;

use crate::states::{app_state::AppState, system_sets::AppSystems, systems::set_app_state_running};

pub struct StatesPlugin;
impl Plugin for StatesPlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<AppState>();

        app.configure_sets(
            Update,
            AppSystems::LoadAssets.run_if(in_state(AppState::LoadAssets)),
        );

        app.configure_sets(
            Update,
            AppSystems::Running.run_if(in_state(AppState::InEditor)),
        );

        app.add_systems(
            Update,
            set_app_state_running.in_set(AppSystems::LoadAssets).run_if(all_assets_loaded),
        );
    }
}
