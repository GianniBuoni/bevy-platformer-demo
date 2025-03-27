use bevy::render::camera::ScalingMode::AutoMin;

use crate::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.add_systems(Startup, spawn_camera);
}

fn spawn_camera(mut commands: Commands) {
    commands.spawn((
        Camera2d,
        OrthographicProjection {
            scaling_mode: AutoMin {
                min_width: GAME_W,
                min_height: GAME_H,
            },
            ..OrthographicProjection::default_2d()
        },
        Transform::from_xyz(GAME_W / 2., GAME_H / 2. + 24., 0.),
    ));
}

// TODO logic for having camera follow player if resource exists
