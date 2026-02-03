use bevy::{
    prelude::*,
    window::{CursorGrabMode, CursorOptions},
};

use crate::maps::functions::SpawnLevel;

pub fn start_game(
    mut spawn_level: MessageWriter<SpawnLevel>,
    mut cursor_options: Single<&mut CursorOptions>,
) {
    spawn_level.write(SpawnLevel {
        level_id: String::from("level_1"),
    });

    cursor_options.grab_mode = CursorGrabMode::Locked;
    cursor_options.visible = false;
}
