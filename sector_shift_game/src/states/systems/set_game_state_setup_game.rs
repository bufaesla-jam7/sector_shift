use bevy::prelude::*;

use crate::states::states::GameState;

pub fn set_game_state_setup_game(mut next_state: ResMut<NextState<GameState>>) {
    next_state.set(GameState::SetupGame);
}
