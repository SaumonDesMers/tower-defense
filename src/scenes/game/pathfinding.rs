use std::time::Instant;

use avian2d::{debug_render, parry::query, prelude::*};
use bevy::ecs::relationship::RelationshipSourceCollection;
use bevy::image::ImageSampler;
use bevy::image::TextureAccessError;
use bevy::reflect::List;
use bevy::{
    asset::RenderAssetUsages,
    prelude::*,
    render::render_resource::{Extent3d, TextureDimension, TextureFormat},
};

use super::{base::Base, physic::GameLayer};

pub struct PathfindingPlugin;

impl Plugin for PathfindingPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(PathfindingMap::new(
            Vec2::new(-1000.0, -500.0),
            200,
            100,
            10.0,
        ))
        .add_systems(Startup, setup)
        .add_observer(update_pathfinding_map)
        .add_systems(Update, show_pathfinding_map);
    }
}

fn setup(
    mut commands: Commands,
    images: ResMut<Assets<Image>>,
    pathfinding_map: ResMut<PathfindingMap>,
) {
    commands.trigger(UpdatePathfindingMapEvent);
    pathfinding_map.create_debug_sprite(commands, images);
}

#[derive(Resource)]
pub struct PathfindingMap {
    position: Vec2,
    width: usize,
    height: usize,
    tile_size: f32,
    tiles: Vec<Tile>,
}

impl PathfindingMap {
    pub fn new(position: Vec2, width: usize, height: usize, tile_size: f32) -> Self {
        Self {
            position,
            width,
            height,
            tile_size,
            tiles: vec![
                Tile {
                    walkable: true,
                    to_target: Vec2::ZERO,
                    distance_to_target: 0.0,
                    avoidance: Vec2::ZERO,
                };
                width * height
            ],
        }
    }

    fn index(&self, pos: Vec2) -> Option<usize> {
        let pos = (pos - self.position) / self.tile_size;
        if pos.x < self.width as f32 && pos.y < self.height as f32 && pos.x >= 0.0 && pos.y >= 0.0 {
            Some((pos.y as usize) * self.width + (pos.x as usize))
        } else {
            None
        }
    }

    fn position(&self, index: usize) -> Vec2 {
        Vec2::new(
            (index % self.width) as f32 + 0.5,
            (index / self.width) as f32 + 0.5,
        ) * self.tile_size
            + self.position
    }

    fn neighbor_indices(&self, index: usize) -> Vec<usize> {
        let pos = self.position(index);
        [
            Vec2::new(1.0, 0.0),
            Vec2::new(-1.0, 0.0),
            Vec2::new(0.0, 1.0),
            Vec2::new(0.0, -1.0),
        ]
        .into_iter()
        .filter_map(|offset| self.index(pos + offset * self.tile_size))
        .collect()
    }

    pub fn get_flow(&self, pos: Vec2, agent_radius: f32) -> Vec2 {
        if let Some(index) = self.index(pos) {
            let to_target = self.tiles[index].to_target;
            let avoidance = self.tiles[index].avoidance;
            let obstacle_distance = avoidance.length();
            let avoidance_strength = 1.0 / (obstacle_distance - agent_radius);
            (to_target + avoidance.normalize_or_zero() * avoidance_strength).normalize_or_zero()
        } else {
            Vec2::ZERO
        }
    }

    fn create_debug_sprite(&self, mut commands: Commands, mut images: ResMut<Assets<Image>>) {
        let mut image = Image::new(
            Extent3d {
                width: self.width as u32,
                height: self.height as u32,
                depth_or_array_layers: 1,
            },
            TextureDimension::D2,
            self.tiles
                .iter()
                .flat_map(|tile| {
                    if tile.walkable {
                        vec![0, 255, 0, 255]
                    } else {
                        vec![255, 0, 0, 255]
                    }
                })
                .collect::<Vec<u8>>(),
            TextureFormat::Rgba8UnormSrgb,
            RenderAssetUsages::RENDER_WORLD | RenderAssetUsages::MAIN_WORLD,
        );
        image.sampler = ImageSampler::nearest();

        let mut sprite = Sprite::from_image(images.add(image));
        sprite.custom_size = Some(Vec2::new(
            self.width as f32 * self.tile_size,
            self.height as f32 * self.tile_size,
        ));
        sprite.color = Color::WHITE.with_alpha(0.3);

        commands.spawn((
            Transform::from_xyz(0.0, 0.0, 0.1),
            sprite,
            PathfindingMapSprite,
            Visibility::Hidden,
        ));
    }
}

#[derive(Clone, Copy, Debug)]
struct Tile {
    pub walkable: bool,
    pub to_target: Vec2,
    pub distance_to_target: f32,
    pub avoidance: Vec2,
}

#[derive(Event)]
pub struct UpdatePathfindingMapEvent;

#[derive(Component)]
struct PathfindingMapSprite;

fn update_pathfinding_map(
    _: On<UpdatePathfindingMapEvent>,
    mut map: ResMut<PathfindingMap>,
    spatial_query: SpatialQuery,
    mut sprite_query: Query<&mut Sprite, With<PathfindingMapSprite>>,
    mut images: ResMut<Assets<Image>>,
    target_query: Query<&Transform, With<Base>>,
) {
    info!("Updating pathfinding map...");
    let start_time = Instant::now();

    let query_filter = SpatialQueryFilter::from_mask(GameLayer::Building);
    for i in 0..map.tiles.len() {
        let pos = map.position(i);

        // When two colliders overlap and a point is inside one but is closer to the edge of the other one (of which it's outside).
        // If solid = false, project_point would return the collider with the closest edge, hense missing the one where the point is inside.
        // So we first project_point with solid = true to test if the point is inside a  collider
        // Then we project_point with solid = false to get the closest edge.
        if let Some(point_projection) = spatial_query.project_point(pos, true, &query_filter) {
            map.tiles[i] = Tile {
                walkable: !point_projection.is_inside,
                to_target: Vec2::ZERO,
                distance_to_target: f32::INFINITY,
                avoidance: pos
                    - spatial_query
                        .project_point(pos, false, &query_filter)
                        .expect("Should return Some")
                        .point,
            };
        } else {
            map.tiles[i] = Tile {
                walkable: true,
                to_target: Vec2::ZERO,
                distance_to_target: f32::INFINITY,
                avoidance: Vec2::ZERO,
            };
        }
    }

    let base_target_pos = if let Ok(target_transform) = target_query.single() {
        Vec2::new(
            target_transform.translation.x,
            target_transform.translation.y,
        )
    } else {
        warn!("No target found for pathfinding map");
        return;
    };

    let mut visited = std::collections::HashSet::new();
    let mut border_cells = std::collections::HashSet::new();
    let Some(base_target_index) = map.index(base_target_pos) else {
        warn!("Target is out of bounds for pathfinding map");
        return;
    };
    map.tiles[base_target_index] = Tile {
        walkable: true,
        to_target: Vec2::ZERO,
        distance_to_target: 0.0,
        avoidance: Vec2::ZERO,
    };
    visited.insert(base_target_index);
    border_cells.insert(base_target_index);

    let mut last_target = None;

    while let Some(target_index) = border_cells
        .iter()
        .copied()
        .min_by_key(|index| map.tiles[*index].distance_to_target as u32)
    {
        if Some(target_index) == last_target {
            border_cells = border_cells
                .into_iter()
                .filter(|index| *index != target_index)
                .collect();
            warn!(
                "targeting {target_index} ({}) again, skipping",
                map.position(target_index)
            );
            continue;
        }
        last_target = Some(target_index);

        // info!(
        //     "target ({}) {:?}",
        //     map.position(target_index),
        //     map.tiles[target_index]
        // );
        let mut to_expand = std::collections::VecDeque::new();
        to_expand.push_back(target_index);
        let target_pos = map.position(target_index);

        while let Some(current_index) = to_expand.pop_front() {
            // info!(
            //     "expand ({}) {:?}",
            //     map.position(current_index),
            //     map.tiles[current_index]
            // );
            let mut has_obstructed_neighbor = false;
            for neighbor_index in map.neighbor_indices(current_index) {
                if map.tiles[neighbor_index].walkable && !visited.contains(&neighbor_index) {
                    // info!(
                    //     "neighbor ({}) {:?}",
                    //     map.position(neighbor_index),
                    //     map.tiles[neighbor_index]
                    // );
                    let neighbor_pos = map.position(neighbor_index);
                    let to_target = target_pos - neighbor_pos;
                    let neighbor = map.tiles[neighbor_index];

                    if spatial_query
                        .cast_ray(
                            neighbor_pos,
                            Dir2::new(to_target)
                                .expect("Direction from cell to target should be valid"),
                            to_target.length(),
                            false,
                            &query_filter,
                        )
                        .is_none()
                    {
                        // info!("not obstructed");
                        map.tiles[neighbor_index] = Tile {
                            to_target: to_target.normalize_or_zero(),
                            distance_to_target: map.tiles[target_index].distance_to_target
                                + to_target.length(),
                            ..neighbor
                        };

                        visited.insert(neighbor_index);
                        to_expand.push_back(neighbor_index);
                    } else {
                        // info!("obstructed");
                        has_obstructed_neighbor = true;
                    }
                }
            }

            if has_obstructed_neighbor {
                border_cells.insert(current_index);
            }
        }

        // Filter all the border cells that don't have any obstructed neighbor (meaning they are not border cell anymore).
        border_cells = border_cells
            .into_iter()
            .filter(|index| {
                map.neighbor_indices(*index)
                    .into_iter()
                    .any(|neighbor_index| {
                        map.tiles[neighbor_index].walkable && !visited.contains(&neighbor_index)
                    })
            })
            .collect();

        // break;
    }

    info!("Done in {} ms", (Instant::now() - start_time).as_millis());

    info!("Updating pathfinding map sprite...");
    if let Some(sprite) = sprite_query.iter_mut().next() {
        if let Some(image) = images.get_mut(&sprite.image) {
            for (i, tile) in map.tiles.iter().enumerate() {
                let color = if tile.walkable && tile.to_target != Vec2::ZERO {
                    // Map flow direction to color for visualization
                    let flow = tile.to_target.normalize_or_zero();
                    let flow = (flow + Vec2::ONE) / 2.0; // Map from [-1, 1] to [0, 1]
                    let color_x_0 = Vec3::new(1.0, 0.0, 0.0);
                    let color_x_1 = Vec3::new(0.0, 1.0, 0.0);
                    let color_y_0 = Vec3::new(1.0, 0.0, 1.0);
                    let color_y_1 = Vec3::new(0.0, 0.0, 1.0);
                    let color = color_x_0 * (1.0 - flow.x) + color_x_1 * flow.x;
                    let color = color * (1.0 - flow.y)
                        + (color_y_0 * (1.0 - flow.y) + color_y_1 * flow.y) * flow.y;

                    Color::srgb(color.x, color.y, color.z)
                } else {
                    Color::srgb(0.0, 0.0, 0.0)
                };
                match image.set_color_at(
                    (i % map.width) as u32,
                    (map.height - 1 - (i / map.width)) as u32,
                    color,
                ) {
                    Ok(_) => {}
                    Err(TextureAccessError::OutOfBounds { x, y, z: _ }) => warn!(
                        "Failed to set color: Out of bounds at ({}, {}) in image of size {:?}",
                        x,
                        y,
                        image.size()
                    ),
                    Err(e) => warn!("Failed to set color: {:?}", e),
                }
            }
        }
    }
}

fn show_pathfinding_map(
    // _: On<ShowPathfindingMapEvent>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut query: Query<&mut Visibility, With<PathfindingMapSprite>>,
) {
    if keyboard_input.just_pressed(KeyCode::KeyP) {
        for mut visibility in query.iter_mut() {
            info!("Toggling pathfinding map visibility...");
            *visibility = match *visibility {
                Visibility::Visible => Visibility::Hidden,
                Visibility::Hidden => Visibility::Visible,
                _ => *visibility,
            };
        }
    }
}
