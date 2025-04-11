use crate::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.register_ldtk_entity::<PalmBundle>("PalmTop");
    app.register_ldtk_entity::<DecorBundle>("Door");
    app.register_ldtk_entity::<AnimatedDecorBundle>("Candle");
    app.register_ldtk_entity::<AnimatedDecorBundle>("CandleLight");
    app.register_ldtk_entity::<AnimatedDecorBundle>("Window");
    app.register_ldtk_entity::<AnimatedDecorBundle>("ChainSmall");
    app.register_ldtk_entity::<AnimatedDecorBundle>("ChainBig");
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
    entity_instance: EntityInstance,
}

#[derive(LdtkEntity, Bundle, Default)]
struct DecorBundle {
    #[sprite_sheet]
    sprite: Sprite,
}

#[derive(LdtkEntity, Bundle, Default)]
struct AnimatedDecorBundle {
    #[sprite_sheet]
    sprite: Sprite,
    #[from_entity_instance]
    entity_instance: EntityInstance,
}
