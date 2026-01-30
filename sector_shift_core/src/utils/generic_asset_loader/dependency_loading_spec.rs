use bevy::asset::LoadContext;

use crate::{enemies::assets::EnemyAsset, environment::assets::EnvObjAsset, items::assets::ItemAsset};

pub trait LoadAssetDependencies {
    fn load_dependencies(&mut self, _load_context: &mut LoadContext) {}
}

impl LoadAssetDependencies for EnemyAsset {
    fn load_dependencies(&mut self, load_context: &mut LoadContext) {
        self.gltf_handle = Some(load_context.load(&self.gltf));
    }
}
impl LoadAssetDependencies for ItemAsset {}
impl LoadAssetDependencies for EnvObjAsset {
    fn load_dependencies(&mut self, load_context: &mut LoadContext) {
        self.gltf_handle = Some(load_context.load(&self.gltf));
    }
}
