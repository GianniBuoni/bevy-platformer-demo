use crate::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.register_ldtk_entity::<PalmBundle>("PalmTop");
}

#[derive(Component, Default)]
struct PalmTop;

#[derive(LdtkEntity, Bundle, Default)]
struct PalmBundle {
    palm_top: PalmTop,
    #[sprite_sheet]
    sprite: Sprite,
}
