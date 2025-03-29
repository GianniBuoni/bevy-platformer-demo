use std::{
    collections::{HashMap, HashSet},
    hash::Hash,
};

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
    wall_query: Query<(&GridCoords, &Parent), Added<Wall>>,
    parent_query: Query<&Parent, Without<Wall>>,
    level_query: Query<(Entity, &LevelIid)>,
    ldtk_projects: Query<&LdtkProjectHandle>,
    ldtk_assets: Res<Assets<LdtkProject>>,
) {
    #[derive(Clone, PartialEq, Eq, Debug, Default, Hash)]
    struct Plate {
        left: i32,
        right: i32,
    }

    struct Rect {
        top: i32,
        bottom: i32,
        left: i32,
        right: i32,
    }

    let mut level_to_wall_locations: HashMap<Entity, HashSet<GridCoords>> =
        HashMap::new();

    wall_query.iter().for_each(|(&grid_coords, parent_layer)| {
        if let Ok(grandparent_level) = parent_query.get(parent_layer.get()) {
            level_to_wall_locations
                .entry(grandparent_level.get())
                .or_default()
                .insert(grid_coords);
        }
    });

    if !wall_query.is_empty() {
        level_query.iter().for_each(|(level_entity, level_iid)| {
            if let Some(level_walls) =
                level_to_wall_locations.get(&level_entity)
            {
                let ldtk_project_handle = get_single!(ldtk_projects);
                let ldtk_project = ldtk_assets
                    .get(ldtk_project_handle)
                    .expect("LDTK project should already be loaded");
                let loaded_level = ldtk_project
                    .as_standalone()
                    .get_loaded_level_by_iid(&level_iid.to_string())
                    .expect("LDTK level should already be loaded");
                let LayerInstance {
                    c_wid: width,
                    c_hei: height,
                    grid_size,
                    ..
                } = loaded_level.layer_instances()[0];
                info_once!("width: {}, height: {}", width, height);

                // combine wall tiles into flat "plates" in each individual row
                let mut plate_stack: Vec<Vec<Plate>> = Vec::new();

                for y in 0..height {
                    let mut row_plates: Vec<Plate> = Vec::new();
                    let mut plate_start = None;

                    // + 1 to the width so the algorithm "terminates" plates that touch the right edge
                    for x in 0..width + 1 {
                        match (
                            plate_start,
                            level_walls.contains(&GridCoords { x, y }),
                        ) {
                            (Some(s), false) => {
                                row_plates.push(Plate {
                                    left: s,
                                    right: x - 1,
                                });
                                plate_start = None;
                            }
                            (None, true) => plate_start = Some(x),
                            _ => (),
                        }
                    }

                    plate_stack.push(row_plates);
                }
                // an extra empty row so the algorithm "finishes" the rects that touch the top edge
                plate_stack.push(Vec::new());

                // combine "plates" into rectangles across multiple rows
                let mut rect_builder: HashMap<Plate, Rect> = HashMap::new();
                let mut prev_row: Vec<Plate> = Vec::new();
                let mut wall_rects: Vec<Rect> = Vec::new();

                for (y, current_row) in plate_stack.into_iter().enumerate() {
                    for prev_plate in &prev_row {
                        if !current_row.contains(prev_plate) {
                            // remove the finished rect so that the same plate in the future starts a new rect
                            if let Some(rect) = rect_builder.remove(prev_plate)
                            {
                                wall_rects.push(rect);
                            }
                        }
                    }
                    for plate in &current_row {
                        rect_builder
                            .entry(plate.clone())
                            .and_modify(|e| e.top += 1)
                            .or_insert(Rect {
                                bottom: y as i32,
                                top: y as i32,
                                left: plate.left,
                                right: plate.right,
                            });
                    }
                    prev_row = current_row;
                }

                commands.entity(level_entity).with_children(|level| {
                    // Spawn colliders for every rectangle..
                    // Making the collider a child of the level serves two purposes:
                    // 1. Adjusts the transforms to be relative to the level for free
                    // 2. the colliders will be despawned automatically when levels unload
                    for wall_rect in wall_rects {
                        level
                            .spawn_empty()
                            .insert(Name::new("WallCollider"))
                            .insert(Collider::rectangle(
                                (wall_rect.right as f32
                                    - wall_rect.left as f32
                                    + 1.)
                                    * grid_size as f32,
                                (wall_rect.top as f32
                                    - wall_rect.bottom as f32
                                    + 1.)
                                    * grid_size as f32,
                            ))
                            .insert(RigidBody::Static)
                            .insert(Friction::new(1.0))
                            .insert(Transform::from_xyz(
                                (wall_rect.left + wall_rect.right + 1) as f32
                                    * grid_size as f32
                                    / 2.,
                                (wall_rect.bottom + wall_rect.top + 1) as f32
                                    * grid_size as f32
                                    / 2.,
                                0.,
                            ))
                            .insert(GlobalTransform::default());
                    }
                });
            }
        });
    }
}
