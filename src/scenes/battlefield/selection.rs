use bevy::prelude::*;

use crate::scenes::AppState;

pub struct SelectionPlugin;

impl Plugin for SelectionPlugin {
    fn build(&self, app: &mut App) {
        app.add_observer(select_entity.run_if(in_state(AppState::InGame)));
        // .add_systems(
        //     Update,
        //     _log_on_selection_change.run_if(resource_exists_and_changed::<Selection>),
        // );
    }
}

#[derive(Component)]
pub struct Selectable;

#[derive(Resource)]
pub struct Selection {
    pub entity: Option<Entity>,
}

fn select_entity(
    mut click: On<Pointer<Click>>,
    query: Query<(), With<Selectable>>,
    mut selection: ResMut<Selection>,
) {
    if query.get(click.entity).is_ok() {
        if let Some(current) = selection.entity {
            if current != click.entity {
                selection.entity = Some(click.entity);
            }
        } else {
            selection.entity = Some(click.entity);
        }
    } else if selection.entity.is_some() {
        selection.entity = None;
    }
    click.propagate(false);
}

fn _log_on_selection_change(selection: Res<Selection>) {
    if let Some(entity) = selection.entity {
        info!("Selected entity: {:?}", entity);
    } else {
        info!("Deselect entity");
    }
}
