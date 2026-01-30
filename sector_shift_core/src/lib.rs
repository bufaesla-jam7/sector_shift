pub mod enemies;
pub mod environment;
pub mod items;
pub mod maps;
pub mod utils;

mod sector_shift_core_plugin;
pub use self::sector_shift_core_plugin::*;

pub mod prelude {
    // Enemies
    pub use crate::enemies::components::Enemy;
    pub use crate::enemies::resources::EnemyLibrary;

    // Items
    pub use crate::items::components::Item;
    pub use crate::items::data::ItemEffect;
    pub use crate::items::resources::ItemLibrary;

    // Environment objects
    pub use crate::environment::resources::EnvObjLibrary;

    // Maps
    pub use crate::maps::DoorAxis;
    pub use crate::maps::Level;
    pub use crate::maps::MapObject;
    pub use crate::maps::TileType;

    // Utils
    pub use crate::utils::all_assets_loaded;
    pub use crate::utils::billboard::components::Billboard;
    pub use crate::utils::direction::CardinalDirection;
    pub use crate::utils::direction::Direction;
    pub use crate::utils::direction::OrdinalDirection;
    pub use crate::utils::direction::VerticalDirection;
    pub use crate::utils::generic_asset_loader::GenericAssetLoader;
    pub use crate::utils::generic_asset_loader::GenericAssetLoaderError;
    pub use crate::utils::grid;

    // Plugin
    pub use crate::SectorShiftCorePlugin;
}
