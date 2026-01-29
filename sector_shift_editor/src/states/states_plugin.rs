use bevy::prelude::*;
use sector_shift_core::prelude::{enemies_loaded, items_loaded};

use crate::states::{states::AppState, system_sets::AppSet, systems::set_app_state_running};

pub struct StatesPlugin;
impl Plugin for StatesPlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<AppState>();

        app.configure_sets(
            Update,
            AppSet::LoadAssets.run_if(in_state(AppState::LoadAssets)),
        );

        app.configure_sets(Update, AppSet::Running.run_if(in_state(AppState::InEditor)));

        app.add_systems(
            Update,
            set_app_state_running.in_set(AppSet::LoadAssets).run_if(items_loaded.and(enemies_loaded)),
        );
    }
}
