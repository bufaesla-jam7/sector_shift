use bevy::prelude::*;

use crate::states::app_state::AppState;

pub fn set_app_state_running(mut next_state: ResMut<NextState<AppState>>) {
    next_state.set(AppState::InEditor);
}
