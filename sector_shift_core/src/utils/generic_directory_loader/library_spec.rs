use bevy::prelude::*;

use crate::{
    environment::{assets::EnvObjAsset, resources::EnvObjLibrary},
    items::assets::ItemAsset,
    prelude::ItemLibrary,
};

pub trait DataLibrary: Resource + Send + Sync + 'static {
    /// The type of the asset
    type Asset: Asset;
    /// Where these assets are stored
    const ASSET_PATH: &'static str;
    /// The file extension for these assets
    const EXTENSION: &'static str;

    fn add_asset_handle(&mut self, handle: Handle<Self::Asset>);
}

impl DataLibrary for ItemLibrary {
    type Asset = ItemAsset;
    const ASSET_PATH: &'static str = "items";
    const EXTENSION: &'static str = "item";

    fn add_asset_handle(&mut self, handle: Handle<Self::Asset>) {
        self.load(handle);
    }
}

impl DataLibrary for EnvObjLibrary {
    type Asset = EnvObjAsset;
    const ASSET_PATH: &'static str = "environment";
    const EXTENSION: &'static str = "environment";

    fn add_asset_handle(&mut self, handle: Handle<Self::Asset>) {
        self.load(handle);
    }
}
