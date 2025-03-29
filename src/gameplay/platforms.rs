use crate::prelude::*;

#[derive(Component, Default)]
pub struct Platform;

pub(super) fn plugin(app: &mut App) {
    app.add_systems(Update, spawn_platfrom_collider);
}

fn platform_collider() -> Collider {
    Collider::rectangle(GRID_SIZE, 5.)
}

fn platfrom_collider_transfrom() -> Transform {
    Transform::from_xyz(0., GRID_SIZE / 2. - 5., 0.)
}

fn spawn_platfrom_collider(
    platfrom: Query<Entity, Added<Platform>>,
    mut commnads: Commands,
) {
    for platform in platfrom.iter() {
        commnads
            .entity(platform)
            .insert(RigidBody::Static)
            .with_children(|children| {
                children
                    .spawn_empty()
                    .insert(platform_collider())
                    .insert(platfrom_collider_transfrom());
            });
    }
}
