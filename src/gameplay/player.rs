use crate::prelude::*;
use bevy_tnua_avian2d::*;

pub(super) fn plugin(app: &mut App) {
    app.register_ldtk_entity::<PlauerBundle>("Player");
    app.init_resource::<PlayerInputVector>();
    app.init_resource::<PlayerStateResource>();
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

#[derive(Resource, Default)]
struct PlayerStateResource {
    current: PlayerState,
}

#[derive(Default)]
pub enum PlayerState {
    #[default]
    Idle,
    Jump,
}

fn player_input(
    mut input_vector: ResMut<PlayerInputVector>,
    mut player_state: ResMut<PlayerStateResource>,
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

    if kb_input.pressed(KeyCode::Space) {
        player_state.current = PlayerState::Jump;
        return;
    }

    // reset play state at the end?
    player_state.current = PlayerState::Idle;
}

fn player_controls(
    input_vector: Res<PlayerInputVector>,
    player_state: Res<PlayerStateResource>,
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

    // TODO rewrite to a match statement later when more state are happening
    // TODO figuer out how the Tsuna state management works into this
    if let PlayerState::Jump = player_state.current {
        controller.action(TnuaBuiltinJump {
            height: GRAVITY * 0.75,
            takeoff_extra_gravity: GRAVITY / 2.,
            fall_extra_gravity: GRAVITY * 4.,
            ..default()
        })
    }
}
