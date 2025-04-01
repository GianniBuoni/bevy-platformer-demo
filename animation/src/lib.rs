use bevy::{
    app::{App, Plugin},
    prelude::Schedules,
};

mod config;
mod handling;
pub mod prelude;

pub struct AnimationPlugin {
    schedule: Schedules,
}

impl AnimationPlugin {
    fn new(schedule: Schedules) -> Self {
        Self { schedule }
    }
}

impl Plugin for AnimationPlugin {
    fn build(&self, app: &mut App) {}
}
