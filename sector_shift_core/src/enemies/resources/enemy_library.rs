use bevy::{platform::collections::HashMap, prelude::*};

use crate::enemies::{assets::EnemyAsset, resources::EnemyDefinition};

/// A library of enemies which can be spawned
#[derive(Resource, Default)]
pub struct EnemyLibrary {
    /// A map of loaded enemy definitions, sorted by their id
    pub map: HashMap<String, EnemyDefinition>,
    /// A list of enemy assets currently being loaded
    pub loading: Vec<Handle<EnemyAsset>>,
    /// Indicates that all handles in `Self.loading` are fully loaded
    pub loading_finished: bool,
}

impl EnemyLibrary {
    /// Get an enemy definition by its unique ID
    pub fn get(&self, id: &str) -> Option<&EnemyDefinition> {
        self.map.get(id)
    }

    /// Add a new enemy definition to the library
    pub fn add(&mut self, definition: EnemyDefinition) {
        self.map.insert(definition.id.clone(), definition);
    }

    pub fn is_ready(&self) -> bool {
        self.loading_finished && self.loading.is_empty()
    }
}
