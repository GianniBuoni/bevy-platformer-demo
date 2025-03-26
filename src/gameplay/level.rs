use crate::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins(LdtkPlugin);
    app.insert_resource(LevelSelection::index(0));
    app.add_systems(Startup, load_level);
}

fn load_level(mut commands: Commands, assets: Res<AssetServer>) {
    commands.spawn(LdtkWorldBundle {
        ldtk_handle: assets.load("platformer-demo.ldtk").into(),
        ..Default::default()
    });
}
