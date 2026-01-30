use bevy::{platform::collections::HashMap, prelude::*};

use crate::items::{assets::ItemAsset, resources::ItemDefinition};

/// A library of items which can be spawned
#[derive(Resource, Default)]
pub struct ItemLibrary {
    /// A map of loaded item definitions sorted by their id
    pub map: HashMap<String, ItemDefinition>,
    /// A list of item assets currently being loaded
    pub loading: Vec<Handle<ItemAsset>>,
    /// Indicates that all handles in `Self.loading` are fully loaded
    pub loading_finished: bool,
}

impl ItemLibrary {
    /// Get an item definition by its unique ID
    pub fn get(&self, id: &str) -> Option<&ItemDefinition> {
        self.map.get(id)
    }

    /// Add a new item definition to the library
    pub fn add(&mut self, definition: ItemDefinition) {
        self.map.insert(definition.id.clone(), definition);
    }

    pub fn is_ready(&self) -> bool {
        self.loading_finished && self.loading.is_empty()
    }
}
