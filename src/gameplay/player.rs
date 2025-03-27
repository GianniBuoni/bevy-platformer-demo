use crate::prelude::*;
use bevy_tnua::TnuaAnimatingState;
use bevy_tnua_avian2d::*;

use super::animation::AnimationConfig;

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
    TnuaAvian2dSensorShape(|| TnuaAvian2dSensorShape(Collider::rectangle(14., 0.))),
    TnuaAnimatingState<PlayerState>,
)]
pub struct Player;

#[derive(LdtkEntity, Default, Bundle)]
pub struct PlauerBundle {
    player: Player,
    #[sprite_sheet]
    sprite_sheet: Sprite,
    #[from_entity_instance]
    animation_player: AnimationConfig,
}

#[derive(Resource, Default)]
struct PlayerInputVector(pub Vec2);

#[derive(Default)]
pub enum PlayerState {
    #[default]
    Idle,
    #[allow(dead_code)]
    Jump,
}

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

// TODO write state control for player

fn player_controls(
    input_vector: Res<PlayerInputVector>,
    mut controller: Query<(
        &mut TnuaController,
        &TnuaAnimatingState<PlayerState>,
    )>,
) {
    let (mut controller, player_state) = get_single_mut!(controller);
    controller.basis(TnuaBuiltinWalk {
        desired_velocity: input_vector.0.extend(0.) * 100.,
        desired_forward: Dir3::new(input_vector.0.extend(0.)).ok(),
        float_height: 16.,
        acceleration: 100.,
        ..default()
    });

    // TODO rewrite to a match statement later when more state are happening
    // TODO extract struct configs into constants or resoures
    if let Some(PlayerState::Jump) = player_state.get() {
        controller.action(TnuaBuiltinJump {
            height: GRAVITY * 0.75,
            takeoff_extra_gravity: GRAVITY / 2.,
            fall_extra_gravity: GRAVITY * 4.,
            ..default()
        })
    }
}
