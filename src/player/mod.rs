use crate::prelude::*;

pub(crate) mod prelude {
    pub(crate) use super::Player;
    pub(crate) use super::input::{HorizontalDirection, PlayerInput};
    pub(crate) use super::state_management::{
        PlayerState, PlayerStateTransition,
    };
}

use controller::{PlayerPhysics, PlayerPlatform};
use state_management::PlayerStateComponent;

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
    PlayerPhysics,
    PlayerStateComponent,
    PlayerInput,
    PlayerPlatform,
    OrthoMovement
)]
pub struct Player;

#[derive(LdtkEntity, Default, Bundle)]
struct PlayerBundle {
    player: Player,
    #[sprite_sheet]
    sprite_sheet: Sprite,
    #[from_entity_instance]
    animation_player: AnimationConfig,
}
