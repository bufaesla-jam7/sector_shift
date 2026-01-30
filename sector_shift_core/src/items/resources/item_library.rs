use bevy::{platform::collections::HashMap, prelude::*};

use crate::items::{assets::ItemAsset, resources::ItemDefinition};

/// A library of items which can be spawned
#[derive(Resource, Default)]
pub struct ItemLibrary {
    /// A map of loaded item definitions sorted by their id
    pub map: HashMap<String, ItemDefinition>,
    /// A list of item assets currently being loaded
    pub loading: Vec<Handle<ItemAsset>>,
    /// Needed to prevent marking the library as ready when it hadn't even had the chance to load
    /// anything.
    loading_initialized: bool,
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

    /// Add an asset handle to `Self.loading` and mark the loading process as initialized.
    pub fn load(&mut self, handle: Handle<ItemAsset>) {
        self.loading.push(handle);
        self.loading_initialized = true;
    }

    /// Are all assets of this library loaded?
    pub fn is_ready(&self) -> bool {
        self.loading_initialized && self.loading.is_empty()
    }
}
