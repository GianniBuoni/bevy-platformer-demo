use crate::prelude::*;
use bevy_tnua_avian2d::*;

pub(super) fn plugin(app: &mut App) {
    app.register_ldtk_entity::<PlauerBundle>("Player");
    app.add_systems(Update, player_controls);
}

#[derive(Component, Default)]
#[require(
    Name(|| Name::new("Player")),
    RigidBody(|| RigidBody::Dynamic),
    Collider(|| Collider::capsule(8., 9.)),
    LinearVelocity,
    TnuaController,
    TnuaAvian2dSensorShape(|| TnuaAvian2dSensorShape(Collider::rectangle(16., 0.))),
)]
pub struct Player;

#[derive(LdtkEntity, Default, Bundle)]
pub struct PlauerBundle {
    player: Player,
    #[sprite_sheet]
    sprite_sheet: Sprite,
}

// TODO decouple control and input
// TODO create player and physics constants with documentation
fn player_controls(
    mut controller: Query<&mut TnuaController>,
    kb_input: Res<ButtonInput<KeyCode>>,
) {
    let left: f32 = if kb_input.any_pressed([
        KeyCode::ArrowLeft,
        KeyCode::KeyH,
        KeyCode::KeyW,
    ]) {
        1.
    } else {
        0.
    };
    let right: f32 = if kb_input.any_pressed([
        KeyCode::ArrowRight,
        KeyCode::KeyL,
        KeyCode::KeyS,
    ]) {
        1.
    } else {
        0.
    };
    let input_vector = Vec3::new(right - left, 0., 0.);

    let mut controller = get_single_mut!(controller);
    controller.basis(TnuaBuiltinWalk {
        desired_velocity: input_vector * 100.,
        desired_forward: Dir3::new(input_vector).ok(),
        float_height: 16.,
        acceleration: 100.,
        ..default()
    });

    if kb_input.just_pressed(KeyCode::Space) {
        controller.action(TnuaBuiltinJump {
            height: 72.,
            takeoff_extra_gravity: 50.,
            ..default()
        });
    }
}
