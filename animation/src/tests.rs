use std::time::Duration;

use bevy::time::TimePlugin;

use super::*;

#[derive(SystemSet, PartialEq, Eq, Hash, Clone, Debug)]
enum TestSystemSets {
    FirstSet,
    SecondSet,
}

#[test]
fn test_init() {
    let mut app = App::new();
    app.add_plugins(TimePlugin);
    app.configure_sets(
        Update,
        (TestSystemSets::FirstSet, TestSystemSets::SecondSet),
    );
    app.add_plugins(AnimationStatePlugin::new(TestSystemSets::FirstSet));
    app.update();
}

#[test]
fn test_handle_animation() {
    let dummy_config = Config::new(0, 2, 1, true);
    let dummy_sprite = Sprite::from_color(Color::WHITE, Vec2::new(10., 10.));
    let mut app = App::new();

    app.add_plugins(TimePlugin);
    app.configure_sets(Update, TestSystemSets::SecondSet);
    app.add_plugins(AnimationStatePlugin::new(TestSystemSets::SecondSet));
    let config_id = app
        .world_mut()
        .commands()
        .spawn((dummy_sprite, dummy_config))
        .id();

    // loop through app update for a couple of secs to mutate config
    let mut timer = Timer::from_seconds(2., TimerMode::Once);
    while !timer.finished() {
        timer.tick(Duration::from_secs(1));
        app.update();
    }

    // panic if config is removed from entity
    let updated_config = app.world().get::<Config>(config_id).unwrap();
    let original_config = Config::new(0, 2, 1, true);
    assert_ne!(&original_config, updated_config);

    // TODO add sprite check
}
