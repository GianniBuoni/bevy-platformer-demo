use crate::prelude::*;

pub(crate) mod prelude {
    pub(crate) use super::animation::AnimationConfig;
}

mod animation;
mod level;
mod walls;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins((animation::plugin, level::plugin, walls::plugin));
}
