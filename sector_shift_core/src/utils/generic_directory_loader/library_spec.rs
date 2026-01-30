use bevy::prelude::*;

use crate::{
    enemies::assets::EnemyAsset, environment::assets::EnvObjAsset, items::assets::ItemAsset, prelude::*,
};

pub trait DataLibrary: Resource + Send + Sync + 'static {
    /// The type of the asset
    type Asset: Asset;
    /// Where these assets are stored
    const ASSET_PATH: &'static str;
    /// The file extension for these assets
    const EXTENSION: &'static str;

    fn add_asset_handle(&mut self, handle: Handle<Self::Asset>);
    fn mark_finished(&mut self);
}

impl DataLibrary for EnemyLibrary {
    type Asset = EnemyAsset;
    const ASSET_PATH: &'static str = "enemies";
    const EXTENSION: &'static str = Self::Asset::EXTENSION;

    fn add_asset_handle(&mut self, handle: Handle<Self::Asset>) {
        self.loading.push(handle);
    }
    fn mark_finished(&mut self) {
        self.loading_finished = true;
    }
}

impl DataLibrary for ItemLibrary {
    type Asset = ItemAsset;
    const ASSET_PATH: &'static str = "items";
    const EXTENSION: &'static str = Self::Asset::EXTENSION;

    fn add_asset_handle(&mut self, handle: Handle<Self::Asset>) {
        self.loading.push(handle);
    }
    fn mark_finished(&mut self) {
        self.loading_finished = true;
    }
}

impl DataLibrary for EnvObjLibrary {
    type Asset = EnvObjAsset;
    const ASSET_PATH: &'static str = "environment";
    const EXTENSION: &'static str = Self::Asset::EXTENSION;

    fn add_asset_handle(&mut self, handle: Handle<Self::Asset>) {
        self.loading.push(handle);
    }
    fn mark_finished(&mut self) {
        self.loading_finished = true;
    }
}
