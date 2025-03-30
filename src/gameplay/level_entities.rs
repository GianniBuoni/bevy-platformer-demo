use crate::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.register_ldtk_entity::<PalmBundle>("PalmTop");
    app.register_ldtk_entity::<DecorBundle>("Door");
    app.register_ldtk_entity::<DecorBundle>("Candle");
    app.register_ldtk_entity::<DecorBundle>("CandleLight");
    app.register_ldtk_entity::<DecorBundle>("Window");
    app.register_ldtk_entity::<DecorBundle>("ChainSmall");
    app.register_ldtk_entity::<DecorBundle>("ChainBig");
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

#[derive(LdtkEntity, Bundle, Default)]
struct DecorBundle {
    #[sprite_sheet]
    sprite: Sprite,
}
