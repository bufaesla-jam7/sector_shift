use bevy::prelude::*;
use bevy_egui::EguiPlugin;
use sector_shift_core::{SectorShiftCorePlugin, utils::asset_plugin_with_fixed_path};

use crate::{
    resources::{BrushData, MapData, UiState},
    states::{states::AppState, system_sets::AppSet},
    systems::{draw_grid, draw_ui, handle_keyboard_input, handle_mouse_input, spawn_camera},
};

pub mod data;
pub mod resources;
pub mod states;
pub mod systems;

mod constants;
pub use self::constants::*;

fn main() {
    let mut app = App::new();

    app.add_plugins(
        DefaultPlugins
            .set(WindowPlugin {
                primary_window: Some(Window {
                    title: "SectorShift".to_string(),
                    ..Default::default()
                }),
                ..Default::default()
            })
            .set(asset_plugin_with_fixed_path()),
    );

    app.add_plugins(EguiPlugin::default());

    app.add_plugins(states::StatesPlugin);

    app.add_plugins(SectorShiftCorePlugin::new(AppState::LoadAssets));

    app.init_resource::<BrushData>();
    app.init_resource::<MapData>();
    app.init_resource::<UiState>();

    app.add_systems(Startup, spawn_camera);
    app.add_systems(
        Update,
        (draw_grid, handle_keyboard_input, handle_mouse_input).in_set(AppSet::Running),
    );
    app.add_systems(bevy_egui::EguiPrimaryContextPass, draw_ui);

    app.run();
}
