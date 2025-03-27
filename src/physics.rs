use crate::prelude::*;
use bevy_tnua_avian2d::TnuaAvian2dPlugin;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins((
        PhysicsPlugins::new(PostUpdate),
        TnuaControllerPlugin::new(Update),
        TnuaAvian2dPlugin::new(Update),
    ));
    app.insert_resource(Gravity(Vec2::new(0., -100.)));
    #[cfg(debug_assertions)]
    app.add_plugins(PhysicsDebugPlugin::default());
}
