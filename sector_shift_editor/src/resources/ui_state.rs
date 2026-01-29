use bevy::prelude::*;
use sector_shift_core::prelude::*;

#[derive(Resource)]
pub struct UiState {
    pub status_message: String,
    pub level_name: String,
    pub selected_door_axis: DoorAxis,
    pub player_start_direction: Direction,
    pub enemy_name: String,
    pub exit_name: String,
    pub item_name: String,
}

impl Default for UiState {
    fn default() -> Self {
        UiState {
            status_message: "Welcome to SectorShift Editor!".to_string(),
            level_name: "Level_1".to_string(),
            selected_door_axis: DoorAxis::Horizontal,
            player_start_direction: Direction::NORTH,
            enemy_name: "slime".to_string(),
            exit_name: "level_2".to_string(),
            item_name: "medkit".to_string(),
        }
    }
}
