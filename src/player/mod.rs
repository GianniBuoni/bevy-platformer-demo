use crate::prelude::*;

pub(crate) mod prelude {
    pub(crate) use super::Player;
    pub(crate) use super::input::PlayerInput;
}

use controller::{PlayerPhysics, PlayerPlatform};

mod controller;
mod input;

pub(super) fn plugin(app: &mut App) {
    app.register_ldtk_entity::<PlayerBundle>("Player");
    app.add_plugins((controller::plugin, input::plugin));
}

#[derive(Component, Default)]
#[require(
    Name(|| Name::new("Player")),
    PlayerPhysics,
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
    entity_instance: EntityInstance,
}
