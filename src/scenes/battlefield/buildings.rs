use bevy::prelude::*;

use crate::scenes::battlefield::{pathfinding::PartialUpdatePathfindingMapEvent, wave::WavePhase};

pub struct BuildingsPlugin;

impl Plugin for BuildingsPlugin {
    fn build(&self, app: &mut App) {
        app.add_observer(move_building_on_drag.run_if(in_state(WavePhase::Finished)))
            .add_observer(on_drag_end.run_if(in_state(WavePhase::Finished)));
    }
}

#[derive(Component, Clone)]
pub struct Building;

fn move_building_on_drag(
    drag: On<Pointer<Drag>>,
    mut building_query: Query<&mut Transform, With<Building>>,
    proj_query: Query<&Projection>,
) {
    if let Ok(mut transform) = building_query.get_mut(drag.entity) {
        if drag.button == PointerButton::Primary {
            let proj = proj_query.single().expect("Expected a single projection");
            let proj = match proj {
                Projection::Orthographic(ortho) => ortho,
                _ => panic!("Expected an orthographic projection"),
            };
            transform.translation.x += drag.delta.x * proj.scale;
            transform.translation.y -= drag.delta.y * proj.scale;
        } else if drag.button == PointerButton::Secondary {
            transform.rotate(Quat::from_rotation_z(-drag.delta.x * 0.01));
        }
    }
}

fn on_drag_end(
    drag: On<Pointer<DragEnd>>,
    mut commands: Commands,
    building_query: Query<(), With<Building>>,
) {
    if building_query.contains(drag.entity) {
        commands.trigger(PartialUpdatePathfindingMapEvent);
    }
}
