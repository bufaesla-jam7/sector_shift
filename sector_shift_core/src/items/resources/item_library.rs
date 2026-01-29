use bevy::{platform::collections::HashMap, prelude::*};

use crate::items::{assets::ItemAsset, resources::ItemDefinition};

#[derive(Resource, Default)]
pub struct ItemLibrary {
    pub map: HashMap<String, ItemDefinition>,
    pub loading: Vec<Handle<ItemAsset>>,
}

impl ItemLibrary {
    pub fn get(&self, id: &str) -> Option<&ItemDefinition> {
        self.map.get(id)
    }

    pub fn add(&mut self, definition: ItemDefinition) {
        self.map.insert(definition.id.clone(), definition);
    }
}
