use crate::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.register_ldtk_entity::<PlauerBundle>("Player");
}

#[derive(Component, Default)]
pub struct Player;

#[derive(LdtkEntity, Default, Bundle)]
pub struct PlauerBundle {
    player: Player,
    #[sprite_sheet]
    sprite_sheet: Sprite,
}
