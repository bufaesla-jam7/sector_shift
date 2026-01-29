use bevy::prelude::*;

use crate::items::resources::ItemLibrary;

/// This function is used as a SystemCondition to wait for all items to be loaded during the asset loading state.
pub fn items_loaded(item_library: Res<ItemLibrary>) -> bool {
    item_library.loading.is_empty()
}
