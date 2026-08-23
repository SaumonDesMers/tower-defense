use avian2d::prelude::*;
use bevy::prelude::*;

use crate::scenes::battlefield::{
    base::Base,
    buildings::Building,
    pathfinding::PathfindingMap,
    wave::{WavePhase, WaveSpawnerZone},
};

pub struct MapValidityPlugin;

impl Plugin for MapValidityPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            test_map_validity.run_if(in_state(WavePhase::Finished)),
        );
    }
}

#[derive(Resource)]
pub struct MapValidity {
    pub error: Option<String>,
}

fn test_map_validity(
    buildings_q: Query<Entity, With<Building>>,
    base_q: Query<Entity, With<Base>>,
    wave_spawner_q: Query<(Entity, &GlobalTransform), With<WaveSpawnerZone>>,
    collisions: Collisions,
    pathfinding_map: Res<PathfindingMap>,
    mut map_validity: ResMut<MapValidity>,
) {
    // for buildings in buildings_q.iter().array_combinations::<2>() {
    //     if collisions.contains(buildings[0], buildings[1]) {
    //         map_validity.error = Some("Buildings overlap with each other.".to_string());
    //         return;
    //     }
    // }

    let Ok(base) = base_q.single() else {
        warn!("Missing base.");
        return;
    };
    for building in buildings_q {
        if collisions.contains(building, base) {
            map_validity.error = Some("Building overlap with base.".to_string());
            return;
        }
    }

    let Ok((wave_spawner, transform)) = wave_spawner_q.single() else {
        warn!("Missing wave spawner.");
        return;
    };
    for building in buildings_q {
        if collisions.contains(building, wave_spawner) {
            map_validity.error = Some("Building overlap with spawner zone.".to_string());
            return;
        }
    }

    let wave_spawner_center = transform.translation();
    if !pathfinding_map
        .is_accessible_from_base(Vec2::new(wave_spawner_center.x, wave_spawner_center.y))
    {
        map_validity.error = Some("The path has been blocked.".to_string());
        return;
    }

    map_validity.error = None;
}
