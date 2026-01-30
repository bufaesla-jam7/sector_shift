use bevy::prelude::*;

use crate::{
    environment::{assets::EnvObjAsset, resources::EnvObjLibrary, systems::process_env_objs},
    utils::{generic_asset_loader::GenericAssetLoader, generic_directory_loader::DirectoryLoaderPlugin},
};

#[derive(Clone)]
pub struct EnvObjsPlugin<T: States + Copy> {
    asset_load_state: T,
}

impl<T: States + Copy> EnvObjsPlugin<T> {
    pub fn new(asset_load_state: T) -> Self {
        Self { asset_load_state }
    }
}

impl<T: States + Copy> Plugin for EnvObjsPlugin<T> {
    fn build(&self, app: &mut App) {
        app.init_asset::<EnvObjAsset>();
        app.register_asset_loader(GenericAssetLoader::<EnvObjAsset>::new(&[
            EnvObjAsset::EXTENSION,
        ]));

        app.init_resource::<EnvObjLibrary>();
        app.add_plugins(DirectoryLoaderPlugin::<EnvObjLibrary, _>::new(
            self.asset_load_state,
        ));

        app.add_systems(
            Update,
            process_env_objs.run_if(in_state(self.asset_load_state)),
        );
    }
}
