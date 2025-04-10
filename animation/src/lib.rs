use crate::prelude::*;

mod config;
mod config_direction;
mod handling;
pub mod prelude;
#[cfg(test)]
mod tests;

pub struct AnimationStatePlugin<T: SystemSet + Clone> {
    schedule: T,
}

impl<T> AnimationStatePlugin<T>
where
    T: SystemSet + Clone,
{
    pub fn new(schedule: T) -> Self {
        Self { schedule }
    }
}

impl<T> Plugin for AnimationStatePlugin<T>
where
    T: SystemSet + Clone,
{
    fn build(&self, app: &mut App) {
        app.add_plugins(config::plugin);
        app.add_plugins(handling::plugin(self.schedule.clone()));
    }
}
