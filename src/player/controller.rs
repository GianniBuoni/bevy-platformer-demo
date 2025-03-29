use crate::prelude::*;
use bevy_tnua::{
    TnuaGhostSensor, TnuaProximitySensor,
    control_helpers::TnuaSimpleFallThroughPlatformsHelper,
};
use bevy_tnua_avian2d::*;

pub(super) fn plugin(app: &mut App) {
    app.add_systems(Update, player_controls.in_set(UpdateSets::Update));
}

#[derive(Component, Default)]
#[require(
    RigidBody(|| RigidBody::Dynamic),
    Collider(|| Collider::capsule(8., 9.)),
    ColliderDensity(|| ColliderDensity(100.)),
    CollisionLayers(player_collision_layers),
    TnuaController,
    TnuaAvian2dSensorShape(player_sensor_shape),
)]
pub struct PlayerPhysics;

#[derive(Component, Default)]
#[require(
    TnuaGhostSensor,
    TnuaProximitySensor,
    TnuaSimpleFallThroughPlatformsHelper
)]
pub struct PlayerPlatform;

fn player_controls(
    mut player: Query<
        (&mut TnuaController, &HorizontalDirection, &PlayerInput),
        With<Player>,
    >,
) {
    let (mut controller, dir, input) = get_single_mut!(player);
    let input_vector: Vec3 = Vec3::new(dir.0, 0., 0.);

    controller.basis(TnuaBuiltinWalk {
        desired_velocity: input_vector * 100.,
        desired_forward: Dir3::new(input_vector).ok(),
        float_height: 16.,
        acceleration: 200.,
        ..default()
    });

    if input.jump {
        controller.action(TnuaBuiltinJump {
            height: 64.,
            ..default()
        })
    }
}

fn player_collision_layers() -> CollisionLayers {
    CollisionLayers::new(GameLayer::Player, [GameLayer::Default])
}

fn player_sensor_shape() -> TnuaAvian2dSensorShape {
    TnuaAvian2dSensorShape(Collider::rectangle(14., 0.))
}
