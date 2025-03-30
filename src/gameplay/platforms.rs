use crate::prelude::*;

const MIN_Y: f32 = GRID_SIZE / 2. + 5.;
const MAX_X: f32 = GRID_SIZE / 2.;

#[derive(Component, Default)]
pub struct Platform;

/// Component that goes on the platform.
/// Marks the component as a ghost platform and the
/// plyaer controller can phase through it when jumping up.
#[derive(Component, Default)]
#[require(ColliderDisabled)]
pub struct GhostPlatform;

pub(super) fn plugin(app: &mut App) {
    app.add_systems(Update, spawn_platfrom_collider);
    app.add_systems(Update, get_platform.in_set(UpdateSets::Update));
}

fn spawn_platfrom_collider(
    platfrom: Query<Entity, Added<Platform>>,
    mut commands: Commands,
) {
    for platform in platfrom.iter() {
        commands
            .entity(platform)
            .insert(RigidBody::Static)
            .with_children(|children| {
                children
                    .spawn_empty()
                    .insert(Name::new("PlatformCollider"))
                    .insert(platform_collider())
                    .insert(platfrom_collider_transfrom())
                    .insert(GhostPlatform);
            });
    }
}

fn get_platform(
    player: Query<&Position, With<TnuaController>>,
    ghost_platform: Query<(Entity, &Position), With<GhostPlatform>>,
    mut commands: Commands,
) {
    for (id, platform) in ghost_platform.iter() {
        let player = get_single!(player);
        let proximity = player.0 - platform.0;
        if proximity.x.abs() <= MAX_X && proximity.y >= MIN_Y {
            commands.entity(id).remove::<ColliderDisabled>();
            break;
        }
        if proximity.x.abs() >= MAX_X || proximity.y <= 0. {
            commands.entity(id).insert(ColliderDisabled);
        }
    }
}

fn platform_collider() -> Collider {
    Collider::rectangle(GRID_SIZE, 5.)
}

fn platfrom_collider_transfrom() -> Transform {
    Transform::from_xyz(0., GRID_SIZE / 2. - 5., 0.)
}
