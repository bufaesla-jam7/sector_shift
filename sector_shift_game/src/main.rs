use bevy::{asset::UnapprovedPathMode, prelude::*};

use sector_shift_core::prelude::*;

use crate::states::states::GameState;

pub mod actors;
pub mod inputs;
pub mod items;
pub mod maps;
pub mod states;
pub mod ui;

mod constants;
pub use self::constants::*;

fn main() {
    let asset_path = env!("CARGO_MANIFEST_DIR")
        .rsplit_once("/sector_shift_game")
        .map(|(p, _)| p.to_string())
        .unwrap_or_else(|| ".".to_string())
        + "/assets";

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
            .set(AssetPlugin {
                file_path: asset_path,
                // Allow scenes to be loaded from anywhere on disk
                unapproved_path_mode: UnapprovedPathMode::Allow,
                ..default()
            }),
    );

    app.add_plugins(avian3d::PhysicsPlugins::default());

    app.add_plugins(states::StatesPlugin);

    app.add_plugins(SectorShiftCorePlugin::new(GameState::LoadAssets));

    app.add_plugins(actors::ActorsPlugin);
    app.add_plugins(inputs::InputsPlugin);
    app.add_plugins(items::ItemsPlugin);
    app.add_plugins(maps::MapsPlugin);
    app.add_plugins(ui::UiPlugin);

    app.run();
}
