use crate::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.register_ldtk_entity::<PalmBundle>("PalmTop");
}

#[derive(Component, Default)]
#[require(Platform)]
struct PalmTop;

#[derive(LdtkEntity, Bundle, Default)]
struct PalmBundle {
    palm_top: PalmTop,
    #[sprite_sheet]
    sprite: Sprite,
    #[from_entity_instance]
    animation_config: AnimationConfig,
}
