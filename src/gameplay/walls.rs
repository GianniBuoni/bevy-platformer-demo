use crate::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.register_ldtk_int_cell::<WallBundle>(1);
    app.add_systems(Update, spawn_wall_colliders);
}

#[derive(Component, Default)]
pub(super) struct Wall;

#[derive(Bundle, LdtkIntCell, Default)]
pub(super) struct WallBundle {
    wall: Wall,
}

fn spawn_wall_colliders(
    mut commands: Commands,
    walls: Query<Entity, Added<Wall>>,
) {
    walls.iter().for_each(|wall| {
        commands.entity(wall).insert((
            Collider::rectangle(32., 32.),
            RigidBody::Static,
            Friction::new(1.),
        ));
    });
}
