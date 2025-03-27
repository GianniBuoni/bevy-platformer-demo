use crate::prelude::*;

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
)]
pub struct Player;

#[derive(LdtkEntity, Default, Bundle)]
pub struct PlauerBundle {
    player: Player,
    #[sprite_sheet]
    sprite_sheet: Sprite,
}

fn player_controls(
    mut controller: Query<&mut TnuaController>,
    kb_input: Res<ButtonInput<KeyCode>>,
) {
    let mut controller = get_single_mut!(controller);
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
    controller.basis(TnuaBuiltinWalk {
        desired_velocity: input_vector * 250.,
        desired_forward: Dir3::new(input_vector).ok(),
        float_height: 16.,
        ..default()
    });
}
