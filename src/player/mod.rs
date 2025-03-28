use crate::prelude::*;
use bevy_tnua_avian2d::*;

pub(crate) mod prelude {
    pub(crate) use super::Player;
    pub(crate) use super::input::PlayerInput;
    pub(crate) use super::state_management::{
        PlayerState, PlayerStateTransition,
    };
}

mod controller;
mod input;
mod state_management;

pub(super) fn plugin(app: &mut App) {
    app.register_ldtk_entity::<PlayerBundle>("Player");
    app.add_plugins((
        controller::plugin,
        input::plugin,
        state_management::plugin,
    ));
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
    PlayerInput,
)]
pub struct Player;

#[derive(LdtkEntity, Default, Bundle)]
pub struct PlayerBundle {
    player: Player,
    #[sprite_sheet]
    sprite_sheet: Sprite,
    #[from_entity_instance]
    animation_player: AnimationConfig,
}
