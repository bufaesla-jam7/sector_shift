use bevy::prelude::*;

use crate::states::states::AppState;

pub fn set_app_state_running(mut next_state: ResMut<NextState<AppState>>) {
    next_state.set(AppState::InEditor);
}
