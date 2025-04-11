use crate::prelude::*;
use animation::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins(AnimationStatePlugin::new(UpdateSets::Draw));
    app.add_systems(Update, add_configs.in_set(UpdateSets::Update));
}

fn add_configs(
    query: Query<(Entity, &EntityInstance), Added<EntityInstance>>,
    mut commands: Commands,
) {
    query.iter().for_each(|(entity, e_instance)| {
        let config = match e_instance.identifier.as_str() {
            "PalmTop" => Config::new(0, 4, 4, true),
            "Candle" => Config::new(32, 6, 10, true),
            "CandleLight" => Config::new(40, 4, 4, true),
            "ChainSmall" => Config::new(16, 8, 2, true),
            "ChainBig" => Config::new(24, 8, 2, true),
            "Window" => Config::new(48, 72, 10, true),
            "Player" => Config::new(0, 5, 10, true),
            _ => {
                return;
            }
        };
        commands.entity(entity).insert(config);
    });
}
