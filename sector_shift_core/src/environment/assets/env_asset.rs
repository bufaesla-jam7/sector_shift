use bevy::prelude::*;
use serde::{Deserialize, Serialize};

/// This gets loaded by the AssetManager
/// Represents a generic environment object model
#[derive(Serialize, Deserialize, Asset, TypePath)]
pub struct EnvObjAsset {
    /// This must be unique as it is used as the key when looking up from the [`EnvObjLibrary`]
    /// resource
    pub id: String,
    /// This is the path to the gltf model to be loaded
    pub gltf: String,
    #[serde(default = "f_true")]
    /// Whether this object can be set on the map in the editor
    /// This should be set to `false` for global environment like the skybox
    pub placeable: bool,
    #[serde(skip)]
    #[dependency]
    /// Asset dependency, the gltf has to be loaded before we can convert [`EnvObjAsset`] to
    /// [`EnvObjDefinition`]
    pub gltf_handle: Option<Handle<Gltf>>,
}

impl EnvObjAsset {
    /// Where these assets are stored
    pub const ASSET_PATH: &'static str = "../assets/environment";
    /// The file extension for these assets
    pub const EXTENSION: &'static str = "environment";
}

/// The serde derive does not allow providing default values inline, they have to be set via
/// [`Default::default()`] or a function.
fn f_true() -> bool {
    true
}
