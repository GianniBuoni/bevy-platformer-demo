use crate::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins(LdtkPlugin);
    app.insert_resource(LdtkSettings {
        set_clear_color: SetClearColor::FromLevelBackground,
        ..default()
    });
    app.insert_resource(LevelSelection::index(0));
    app.register_ldtk_entity::<LevelBgBundle>("BG");
    app.register_type::<EntityInstance>();
    app.add_systems(Startup, load_level);
    app.add_systems(Update, load_bg);
}

fn load_level(mut commands: Commands, assets: Res<AssetServer>) {
    commands.spawn(LdtkWorldBundle {
        ldtk_handle: assets.load("platformer-demo.ldtk").into(),
        ..Default::default()
    });
}

fn load_bg(
    bg: Query<(Entity, &EntityInstance), Added<LevelBg>>,
    mut commnands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let (bg, entity_instance) = get_single!(bg);
    let mesh = meshes.add(Rectangle::new(
        entity_instance.width as f32,
        entity_instance.height as f32,
    ));
    let color = materials.add(entity_instance.smart_color);
    commnands.entity(bg).insert((
        Mesh2d(mesh.clone()),
        MeshMaterial2d(color.clone()),
        Transform::from_xyz(
            entity_instance.width as f32 / 2.,
            entity_instance.height as f32 / 2.,
            1.,
        ),
    ));
}

#[derive(Component, Default)]
struct LevelBg;

#[derive(Bundle, LdtkEntity, Default)]
struct LevelBgBundle {
    level_bg: LevelBg,
    #[from_entity_instance]
    entity_instance: EntityInstance,
}
