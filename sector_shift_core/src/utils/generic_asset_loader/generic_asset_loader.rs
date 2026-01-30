use std::marker::PhantomData;

use bevy::{
    asset::{AssetLoader, AsyncReadExt, LoadContext, io::Reader},
    prelude::*,
};
use serde::Deserialize;

use crate::utils::generic_asset_loader::{GenericAssetLoaderError, LoadAssetDependencies};

#[derive(TypePath)]
pub struct GenericAssetLoader<T> {
    pub extensions: &'static [&'static str],
    phantom: PhantomData<T>,
}

impl<T> Default for GenericAssetLoader<T> {
    fn default() -> Self {
        Self::new(&[])
    }
}

impl<T> GenericAssetLoader<T> {
    pub const fn new(extensions: &'static [&'static str]) -> Self {
        Self {
            extensions,
            phantom: PhantomData,
        }
    }
}

impl<T: Asset + for<'de> Deserialize<'de> + LoadAssetDependencies> AssetLoader for GenericAssetLoader<T> {
    type Asset = T;

    type Settings = ();

    type Error = GenericAssetLoaderError;

    async fn load(
        &self,
        reader: &mut dyn Reader,
        _settings: &Self::Settings,
        load_context: &mut LoadContext<'_>,
    ) -> Result<Self::Asset, Self::Error> {
        let mut contents = String::new();
        reader.read_to_string(&mut contents).await?;
        let mut asset = ron::de::from_str::<T>(&contents)?;
        asset.load_dependencies(load_context);
        Ok(asset)
    }

    fn extensions(&self) -> &[&str] {
        self.extensions
    }
}
