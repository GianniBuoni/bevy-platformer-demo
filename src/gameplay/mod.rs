use crate::prelude::*;

mod animation;
mod level;
mod player;
mod walls;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins((
        animation::plugin,
        level::plugin,
        player::plugin,
        walls::plugin,
    ));
}
