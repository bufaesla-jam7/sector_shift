use bevy::prelude::*;

use crate::environment::resources::EnvObjLibrary;

/// This function is used as a SystemCondition to wait for all items to be loaded during the asset loading state.
pub fn env_objs_loaded(env_library: Res<EnvObjLibrary>) -> bool {
    env_library.loading.is_empty()
}
