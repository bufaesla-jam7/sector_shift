use bevy::{platform::collections::HashMap, prelude::*};

use crate::enemies::{
    assets::EnemyAsset,
    resources::{EnemyDefinition, enemy_model_data::EnemyModelData},
};

/// A library of enemies which can be spawned
#[derive(Resource, Default, Reflect)]
#[reflect(Resource)]
pub struct EnemyLibrary {
    /// A map of loaded enemy definitions and model data, sorted by their id
    pub map: HashMap<String, (EnemyDefinition, EnemyModelData)>,
    /// A list of enemy assets currently being loaded
    pub loading_assets: Vec<Handle<EnemyAsset>>,
    pub loading_models: Vec<EnemyDefinition>,
}

impl EnemyLibrary {
    /// Get an enemy definition and model data by its unique ID
    pub fn get(&self, id: &str) -> Option<&(EnemyDefinition, EnemyModelData)> {
        self.map.get(id)
    }

    /// Get an enemy definition by its unique ID
    pub fn get_definition(&self, id: &str) -> Option<&EnemyDefinition> {
        self.map.get(id).map(|(d, _)| d)
    }

    /// Add a new enemy definition to the library
    pub fn add(&mut self, definition: EnemyDefinition, model_data: EnemyModelData) {
        self.map.insert(definition.id.clone(), (definition, model_data));
    }

    pub fn is_fully_loaded(&self) -> bool {
        self.loading_assets.is_empty() && self.loading_models.is_empty()
    }
}
