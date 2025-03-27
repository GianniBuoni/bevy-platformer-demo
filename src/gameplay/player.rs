use crate::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.register_ldtk_entity::<PlauerBundle>("Player");
}

#[derive(Component, Default)]
#[require(
    Name(|| Name::new("Player")),
    RigidBody(|| RigidBody::Static),
    Collider(|| Collider::rectangle(32., 32.)),
)]
pub struct Player;

#[derive(LdtkEntity, Default, Bundle)]
pub struct PlauerBundle {
    player: Player,
    #[sprite_sheet]
    sprite_sheet: Sprite,
}
