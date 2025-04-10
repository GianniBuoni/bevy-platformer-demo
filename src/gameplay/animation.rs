use std::ops::Deref;

use crate::prelude::*;
use animation::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins(AnimationStatePlugin::new(UpdateSets::Draw));
    app.add_systems(Update, deref_configs.in_set(UpdateSets::Update));
}

#[derive(Component, Default, Deref)]
pub struct AnimationConfig(Config);

impl From<Config> for AnimationConfig {
    fn from(value: Config) -> Self {
        Self(value)
    }
}

impl From<&EntityInstance> for AnimationConfig {
    fn from(value: &EntityInstance) -> Self {
        match value.identifier.as_str() {
            "PalmTop" => Config::new(0, 4, 4, true).into(),
            "Candle" => Config::new(32, 6, 10, true).into(),
            "CandleLight" => Config::new(40, 4, 4, true).into(),
            "ChainSmall" => Config::new(16, 8, 2, true).into(),
            "ChainBig" => Config::new(24, 8, 2, true).into(),
            "Window" => Config::new(48, 72, 10, true).into(),
            _ => Config::new(0, 4, 10, true).into(),
        }
    }
}

// shenanigans to get animaton plugins working: has to be a better way?
fn deref_configs(
    query: Query<(Entity, &AnimationConfig), Added<AnimationConfig>>,
    mut commands: Commands,
) {
    query.iter().for_each(|(entity, config)| {
        let config = config.deref();
        commands
            .entity(entity)
            .insert(Config::from(config.clone()))
            .remove::<AnimationConfig>();
    });
}
