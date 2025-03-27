use crate::prelude::*;

mod level;
mod player;
mod walls;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins((level::plugin, player::plugin, walls::plugin));
}
