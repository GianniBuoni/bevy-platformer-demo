mod state_management;
use crate::gameplay::player::state_management::{
    PlayerState, PlayerStateTransition,
};
use crate::prelude::*;
pub(crate) mod prelude {
    pub(crate) use super::state_management::{
        PlayerState, PlayerStateTransition,
    };
}
use super::animation::AnimationConfig;
use bevy_tnua_avian2d::*;

pub(super) fn plugin(app: &mut App) {
    app.register_ldtk_entity::<PlauerBundle>("Player");
    app.init_resource::<PlayerInput>();
    app.add_plugins(state_management::plugin);
    app.add_systems(Update, player_input.in_set(UpdateSets::RecordInput));
    app.add_systems(Update, player_controls.in_set(UpdateSets::Update));
}

#[derive(Component, Default)]
#[require(
    Name(|| Name::new("Player")),
    RigidBody(|| RigidBody::Dynamic),
    Collider(|| Collider::capsule(8., 9.)),
    ColliderDensity(|| ColliderDensity(100.)),
    LinearVelocity,
    TnuaController,
    TnuaAvian2dSensorShape(|| TnuaAvian2dSensorShape(Collider::rectangle(14., 0.))),
    PlayerState,
    PlayerStateTransition,
)]
pub(super) struct Player;

// TODO move input into it's own module
#[derive(LdtkEntity, Default, Bundle)]
pub struct PlauerBundle {
    player: Player,
    #[sprite_sheet]
    sprite_sheet: Sprite,
    #[from_entity_instance]
    animation_player: AnimationConfig,
}

#[derive(Resource, Default)]
pub(super) struct PlayerInput {
    pub movement_vector: Vec2,
    pub jump: bool,
}

fn player_input(
    mut input_resource: ResMut<PlayerInput>,
    kb_input: Res<ButtonInput<KeyCode>>,
    controller: Query<&TnuaController, With<Player>>,
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
    input_resource.movement_vector.x = right - left;

    let controller = get_single!(controller);
    let Ok(airborne) = &controller.is_airborne() else {
        return;
    };
    if kb_input.just_pressed(KeyCode::Space) && !airborne {
        input_resource.jump = true;
    }
}

fn player_controls(
    input: Res<PlayerInput>,
    mut player: Query<(&mut TnuaController, &PlayerState), With<Player>>,
) {
    let (mut controller, state) = get_single_mut!(player);
    controller.basis(TnuaBuiltinWalk {
        desired_velocity: input.movement_vector.extend(0.) * 100.,
        desired_forward: Dir3::new(input.movement_vector.extend(0.)).ok(),
        float_height: 16.,
        acceleration: 200.,
        ..default()
    });

    if let PlayerState::Jump = &state {
        controller.action(TnuaBuiltinJump {
            height: 75.,
            takeoff_extra_gravity: GRAVITY / 30.,
            fall_extra_gravity: GRAVITY,
            ..default()
        })
    }
}
