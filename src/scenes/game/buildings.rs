use bevy::prelude::*;

pub struct BuildingsPlugin;

impl Plugin for BuildingsPlugin {
    fn build(&self, app: &mut App) {
        app.add_observer(move_building_on_drag);
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
