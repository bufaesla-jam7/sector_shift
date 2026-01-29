use bevy::prelude::*;

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
    let mut app = App::new();

    app.add_plugins(DefaultPlugins.set(WindowPlugin {
        primary_window: Some(Window {
            title: "SectorShift".to_string(),
            ..Default::default()
        }),
        ..Default::default()
    }));

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
