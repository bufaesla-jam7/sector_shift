use crate::states::states::DebugHudState;
use bevy::{
    prelude::*,
    window::{CursorGrabMode, CursorOptions},
};
use bevy_egui::EguiPlugin;
use bevy_inspector_egui::quick::WorldInspectorPlugin;

pub struct DebugHudPlugin;
impl Plugin for DebugHudPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            EguiPlugin::default(),
            WorldInspectorPlugin::new().run_if(in_state(DebugHudState::Enabled)),
        ))
        .add_systems(Update, toggle_debug_hud);
    }
}

fn toggle_debug_hud(
    input: Res<ButtonInput<KeyCode>>,
    state: Res<State<DebugHudState>>,
    mut next_state: ResMut<NextState<DebugHudState>>,
    mut cursor_options: Single<&mut CursorOptions>,
) {
    if input.just_pressed(KeyCode::F3) {
        next_state.set(match state.get() {
            DebugHudState::Enabled => {
                cursor_options.grab_mode = CursorGrabMode::Locked;
                cursor_options.visible = false;
                DebugHudState::Disabled
            },
            DebugHudState::Disabled => {
                cursor_options.grab_mode = CursorGrabMode::None;
                cursor_options.visible = true;
                DebugHudState::Enabled
            },
        });
    }
}
