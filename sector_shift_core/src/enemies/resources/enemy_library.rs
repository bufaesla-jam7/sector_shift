use bevy::{platform::collections::HashMap, prelude::*};

use crate::enemies::{assets::EnemyAsset, resources::EnemyDefinition};

#[derive(Resource, Default)]
pub struct EnemyLibrary {
    pub map: HashMap<String, EnemyDefinition>,
    pub loading: Vec<Handle<EnemyAsset>>,
}

impl EnemyLibrary {
    pub fn get(&self, id: &str) -> Option<&EnemyDefinition> {
        self.map.get(id)
    }

    pub fn add(&mut self, definition: EnemyDefinition) {
        self.map.insert(definition.id.clone(), definition);
    }
}
