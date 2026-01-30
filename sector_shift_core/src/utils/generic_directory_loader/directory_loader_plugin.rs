use std::marker::PhantomData;

use bevy::{asset::LoadedFolder, prelude::*};

use crate::utils::generic_directory_loader::{
    DataLibrary,
    systems::{load_directory, poll_directory_loading_state},
};

pub struct DirectoryLoaderPlugin<T, S> {
    _phantom: PhantomData<T>,
    asset_load_state: S,
}

impl<T, S> DirectoryLoaderPlugin<T, S> {
    pub fn new(asset_load_state: S) -> Self {
        Self {
            _phantom: PhantomData,
            asset_load_state,
        }
    }
}

impl<T: DataLibrary, S: States + Copy> Plugin for DirectoryLoaderPlugin<T, S> {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(self.asset_load_state), load_directory::<T>).add_systems(
            Update,
            poll_directory_loading_state::<T>
                .run_if(in_state(self.asset_load_state).and(resource_exists::<DirectoryLoadingHandle<T>>)),
        );
    }
}

#[derive(Resource)]
pub struct DirectoryLoadingHandle<T> {
    pub handle: Handle<LoadedFolder>,
    pub _phantom: PhantomData<T>,
}
