use bevy::prelude::*;

pub struct UiPlugin;
impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        #[cfg(feature = "dev")]
        app.add_plugins(super::debug_hud::DebugHudPlugin);
    }
}
