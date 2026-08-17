use std::collections::VecDeque;
use std::iter::successors;
use std::time::Instant;

use avian2d::{debug_render, parry::query, prelude::*};
use bevy::ecs::relationship::RelationshipSourceCollection;
use bevy::image::ImageSampler;
use bevy::image::TextureAccessError;
use bevy::platform::collections::HashSet;
use bevy::{
    asset::RenderAssetUsages,
    prelude::*,
    render::render_resource::{Extent3d, TextureDimension, TextureFormat},
};
use itertools::Itertools;

use super::{base::Base, physic::GameLayer};

pub struct PathfindingPlugin;

impl Plugin for PathfindingPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            create_debug_sprite.run_if(resource_added::<PathfindingMap>),
        )
        .add_systems(
            Update,
            remove_debug_sprite.run_if(resource_removed::<PathfindingMap>),
        )
        .add_observer(update_pathfinding_map)
        .add_systems(Update, show_pathfinding_map);
    }
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
                    to_target: Vec2::ZERO,
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
}

#[derive(Clone, Copy, Debug)]
struct Tile {
    pub to_target: Vec2,
    pub avoidance: Vec2,
}

#[derive(Event)]
pub struct UpdatePathfindingMapEvent;

#[derive(Component)]
struct PathfindingMapSprite;

fn create_debug_sprite(
    mut commands: Commands,
    map: ResMut<PathfindingMap>,
    mut images: ResMut<Assets<Image>>,
) {
    let mut image = Image::new(
        Extent3d {
            width: map.width as u32,
            height: map.height as u32,
            depth_or_array_layers: 1,
        },
        TextureDimension::D2,
        vec![0; map.width * map.height * 4],
        TextureFormat::Rgba8UnormSrgb,
        RenderAssetUsages::RENDER_WORLD | RenderAssetUsages::MAIN_WORLD,
    );
    image.sampler = ImageSampler::nearest();

    let mut sprite = Sprite::from_image(images.add(image));
    sprite.custom_size = Some(Vec2::new(
        map.width as f32 * map.tile_size,
        map.height as f32 * map.tile_size,
    ));
    sprite.color = Color::WHITE.with_alpha(0.3);

    commands.spawn((
        Transform::from_xyz(0.0, 0.0, 0.1),
        sprite,
        PathfindingMapSprite,
        Visibility::Hidden,
    ));
}

fn remove_debug_sprite(mut commands: Commands, query: Query<Entity, With<PathfindingMapSprite>>) {
    if let Ok(entity) = query.single() {
        if let Ok(mut cmd) = commands.get_entity(entity) {
            cmd.despawn();
        }
    }
}

fn show_pathfinding_map(
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

#[derive(Debug, Clone, Copy)]
struct TileForUpdate {
    path: Option<usize>,
    visited: bool,
}

fn update_pathfinding_map(
    _: On<UpdatePathfindingMapEvent>,
    mut map: ResMut<PathfindingMap>,
    spatial_query: SpatialQuery,
    mut sprite_query: Query<&mut Sprite, With<PathfindingMapSprite>>,
    mut images: ResMut<Assets<Image>>,
    target_query: Query<&Transform, With<Base>>,
) {
    info!("Updating pathfinding map v3...");
    let start_time = Instant::now();

    let shape = Collider::from(Circle::new(5.0));
    let mut config = ShapeCastConfig::default();
    let filter = SpatialQueryFilter::from_mask(GameLayer::Building);

    let base_target_pos = if let Ok(target_transform) = target_query.single() {
        Vec2::new(
            target_transform.translation.x,
            target_transform.translation.y,
        )
    } else {
        warn!("No target found for pathfinding map");
        return;
    };

    let Some(base_target_index) = map.index(base_target_pos) else {
        warn!("Target is out of bounds for pathfinding map");
        return;
    };
    map.tiles[base_target_index] = Tile {
        to_target: Vec2::ZERO,
        avoidance: Vec2::ZERO,
    };

    let mut tiles = vec![
        TileForUpdate {
            path: None,
            visited: false,
        };
        map.tiles.len()
    ];
    let mut to_visit = VecDeque::new();
    to_visit.push_back(base_target_index);
    tiles[base_target_index] = TileForUpdate {
        path: None,
        visited: true,
    };

    while let Some(current) = to_visit.pop_front() {
        for neighbor_index in map.neighbor_indices(current) {
            if tiles[neighbor_index].visited {
                continue;
            }
            tiles[neighbor_index].visited = true;

            let neighbor_pos = map.position(neighbor_index);

            // When two colliders overlap and a point is inside one but is closer to the edge of the other one (of which it's outside).
            // If solid = false, project_point would return the collider with the closest edge, hense missing the one where the point is inside.
            // So we first call project_point with solid = true to test if the point is inside a collider.
            // And then we call project_point with solid = false to get the closest edge.
            let walkable = spatial_query
                .project_point(neighbor_pos, true, &filter)
                .map(|point_projection| !point_projection.is_inside)
                .unwrap_or(true);

            if walkable {
                map.tiles[neighbor_index].avoidance = spatial_query
                    .project_point(neighbor_pos, false, &filter)
                    .map(|point_projection| neighbor_pos - point_projection.point)
                    .unwrap_or(Vec2::ZERO);
                tiles[neighbor_index].path = Some(current);
                to_visit.push_back(neighbor_index);
            }
        }
    }

    for start in 0..tiles.len() {
        // Trace back from start to target to get the full path.
        // Skip start as it is not needed (and it avoid a zero vector later).
        let full_path: Vec<usize> = successors(Some(start), |index| {
            tiles.get(*index).and_then(|tile| tile.path)
        })
        .skip(1)
        .collect();

        let current_pos = map.position(start);
        // Follow the path in reverse and find the first tile that have a direct line of sight with the start tile.
        let to_target = full_path.into_iter().rev().find_map(|i| {
            let target_pos = map.position(i);
            let to_target = target_pos - current_pos;

            config.max_distance = to_target.length();
            if spatial_query
                .cast_shape(
                    &shape,
                    current_pos,
                    0.0,
                    Dir2::new(to_target).expect("Origin should never be the same as target"),
                    &config,
                    &filter,
                )
                .is_none()
            {
                Some(to_target)
            } else {
                None
            }
        });

        if let Some(to_target) = to_target {
            map.tiles[start].to_target = to_target.normalize_or_zero();
        }
    }

    info!("Done in {} ms", (Instant::now() - start_time).as_millis());

    info!("Updating pathfinding map sprite...");

    let Ok(Some(mut image)) = sprite_query
        .single_mut()
        .map(|sprite| images.get_mut(&sprite.image))
    else {
        return;
    };

    for (i, tile) in map.tiles.iter().enumerate() {
        let color = if tile.to_target != Vec2::ZERO {
            // Map flow direction to color for visualization
            let flow = tile.to_target.normalize_or_zero();
            let flow = (flow + Vec2::ONE) / 2.0; // Map from [-1, 1] to [0, 1]
            let color_x_0 = Vec3::new(1.0, 0.0, 0.0);
            let color_x_1 = Vec3::new(0.0, 1.0, 0.0);
            let color_y_0 = Vec3::new(1.0, 0.0, 1.0);
            let color_y_1 = Vec3::new(0.0, 0.0, 1.0);
            let color = color_x_0 * (1.0 - flow.x) + color_x_1 * flow.x;
            let color =
                color * (1.0 - flow.y) + (color_y_0 * (1.0 - flow.y) + color_y_1 * flow.y) * flow.y;

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
