use crate::prelude::*;
use bevy::window::WindowResolution;

#[cfg(feature = "debug")]
use bevy_inspector_egui::quick::WorldInspectorPlugin;

mod camera;
mod gameplay;
mod physics;
mod player;
mod prelude;

pub struct AppPlugin;

impl Plugin for AppPlugin {
    fn build(&self, app: &mut App) {
        app.configure_sets(
            Update,
            (
                UpdateSets::TimersTick,
                UpdateSets::RecordInput,
                UpdateSets::Update,
                UpdateSets::StateManagement,
                UpdateSets::Draw,
            )
                .chain(),
        );
        app.add_plugins(
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Window {
                        title: "Platformer Demo! 💀".to_string(),
                        resolution: WindowResolution::new(
                            GAME_W * 3.,
                            GAME_H * 3.,
                        ),
                        ..default()
                    }
                    .into(),
                    ..default()
                })
                .set(ImagePlugin::default_nearest()),
        );
        #[cfg(feature = "debug")]
        app.add_plugins(WorldInspectorPlugin::new());
        app.add_plugins((
            camera::plugin,
            gameplay::plugin,
            physics::plugin,
            player::plugin,
        ));
    }
}

#[derive(SystemSet, PartialEq, Eq, Clone, Debug, Hash)]
pub enum UpdateSets {
    TimersTick,
    RecordInput,
    StateManagement,
    Update,
    Draw,
}
