use crate::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins(PhysicsPlugins::default());
    #[cfg(debug_assertions)]
    app.add_plugins(PhysicsDebugPlugin::default());
}
