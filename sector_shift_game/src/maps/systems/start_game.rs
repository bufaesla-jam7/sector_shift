use bevy::{
    prelude::*,
    window::{CursorGrabMode, CursorOptions},
};
use sector_shift_core::prelude::*;

use crate::maps::functions::spawn_level;

pub fn start_game(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut cursor_options: Single<&mut CursorOptions>,
) {
    let level = Level::load("level_1");
    spawn_level(&mut commands, &mut meshes, &mut materials, &level);

    cursor_options.grab_mode = CursorGrabMode::Locked;
    cursor_options.visible = false;
}
