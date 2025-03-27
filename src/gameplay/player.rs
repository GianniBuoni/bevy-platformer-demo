use crate::prelude::*;
use bevy_tnua_avian2d::*;

pub(super) fn plugin(app: &mut App) {
    app.register_ldtk_entity::<PlauerBundle>("Player");
    app.init_resource::<PlayerInputVector>();
    app.add_systems(Update, player_input.in_set(UpdateSets::RecordInput));
    app.add_systems(Update, player_controls.in_set(UpdateSets::Update));
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

#[derive(Resource, Default)]
struct PlayerInputVector(pub Vec2);

fn player_input(
    mut input_vector: ResMut<PlayerInputVector>,
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

    input_vector.0.x = right - left;
}

fn player_controls(
    input_vector: Res<PlayerInputVector>,
    mut controller: Query<&mut TnuaController>,
) {
    let mut controller = get_single_mut!(controller);
    controller.basis(TnuaBuiltinWalk {
        desired_velocity: input_vector.0.extend(0.) * 100.,
        desired_forward: Dir3::new(input_vector.0.extend(0.)).ok(),
        float_height: 16.,
        acceleration: 100.,
        ..default()
    });
}
