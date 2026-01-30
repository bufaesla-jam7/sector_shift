use std::marker::PhantomData;

use bevy::{asset::LoadedFolder, prelude::*};

use crate::utils::generic_directory_loader::{DataLibrary, directory_loader_plugin::DirectoryLoadingHandle};

pub fn load_directory<T: DataLibrary>(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.insert_resource(DirectoryLoadingHandle {
        handle: asset_server.load_folder(T::ASSET_PATH),
        _phantom: PhantomData::<T>,
    });
}

pub fn poll_directory_loading_state<T: DataLibrary>(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    directories: Res<Assets<LoadedFolder>>,
    directory_handle: Res<DirectoryLoadingHandle<T>>,
    mut library: ResMut<T>,
) {
    if asset_server.is_loaded_with_dependencies(&directory_handle.handle) {
        let directory = directories.get(&directory_handle.handle).expect("It was marked as loaded");
        for handle in &directory.handles {
            if let Some(path) = handle.path().map(|p| p.path())
                && path.extension().and_then(|s| s.to_str()) == Some(T::EXTENSION)
            {
                match handle.clone().try_typed::<T::Asset>() {
                    Ok(asset) => library.add_asset_handle(asset),
                    Err(e) => warn!("asset with path {path:?} is not the expected asset: {e}"),
                }
            }
        }
        commands.remove_resource::<DirectoryLoadingHandle<T>>();
    }
}
