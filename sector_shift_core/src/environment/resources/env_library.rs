use bevy::{platform::collections::HashMap, prelude::*};

use crate::environment::{assets::EnvObjAsset, resources::EnvObjDefinition};

/// A library of environment objects which can be spawned
#[derive(Resource, Default)]
pub struct EnvObjLibrary {
    /// A map of loaded environment definitions sorted by their id
    pub map: HashMap<String, EnvObjDefinition>,
    /// A list of environment object assets currently being loaded
    pub loading: Vec<Handle<EnvObjAsset>>,
    /// Indicates that all handles in `Self.loading` are fully loaded
    pub loading_finished: bool,
}

impl EnvObjLibrary {
    /// Get an environment definition by its unique ID
    pub fn get(&self, id: &str) -> Option<&EnvObjDefinition> {
        self.map.get(id)
    }

    /// Add a new environment object definition to the library
    pub fn add(&mut self, definition: EnvObjDefinition) {
        self.map.insert(definition.id.clone(), definition);
    }

    pub fn is_ready(&self) -> bool {
        self.loading_finished && self.loading.is_empty()
    }
}
