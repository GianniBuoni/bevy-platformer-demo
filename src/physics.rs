use crate::prelude::*;
use bevy_tnua_avian2d::TnuaAvian2dPlugin;

#[allow(unused_imports)]
pub(crate) mod prelude {
    pub(crate) use super::GRAVITY;
}

pub const GRAVITY: f32 = 100.;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins((
        PhysicsPlugins::new(FixedPostUpdate),
        TnuaControllerPlugin::new(FixedUpdate),
        TnuaAvian2dPlugin::new(FixedUpdate),
    ));
    app.insert_resource(Gravity(Vec2::new(0., -GRAVITY)));
    #[cfg(debug_assertions)]
    app.add_plugins(PhysicsDebugPlugin::default());
}
