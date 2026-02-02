use crate::states::states::DebugHudState;
use avian3d::prelude::*;
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
            PhysicsDebugPlugin,
        ))
        .insert_gizmo_config(PhysicsGizmos::none(), GizmoConfig::default())
        .add_systems(Update, toggle_debug_hud);
    }
}

fn toggle_debug_hud(
    input: Res<ButtonInput<KeyCode>>,
    state: Res<State<DebugHudState>>,
    mut next_state: ResMut<NextState<DebugHudState>>,
    mut cursor_options: Single<&mut CursorOptions>,
    mut gizmo_config: ResMut<GizmoConfigStore>,
) {
    if input.just_pressed(KeyCode::F3) {
        let config: &mut PhysicsGizmos = gizmo_config.config_mut().1;
        next_state.set(match state.get() {
            DebugHudState::Enabled => {
                cursor_options.grab_mode = CursorGrabMode::Locked;
                cursor_options.visible = false;
                *config = PhysicsGizmos::none();
                DebugHudState::Disabled
            },
            DebugHudState::Disabled => {
                cursor_options.grab_mode = CursorGrabMode::None;
                cursor_options.visible = true;
                *config = PhysicsGizmos::default();
                DebugHudState::Enabled
            },
        });
    }
}
