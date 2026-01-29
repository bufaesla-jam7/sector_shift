use bevy::prelude::*;

use crate::items::resources::ItemLibrary;

pub fn items_loaded(item_library: Res<ItemLibrary>) -> bool {
    item_library.loading.is_empty()
}
