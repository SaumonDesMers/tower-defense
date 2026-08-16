use avian2d::parry::query;
use bevy::{ecs::observer, prelude::*};

use super::selection;

pub struct SelectionPlugin;

impl Plugin for SelectionPlugin {
    fn build(&self, app: &mut App) {
        app.add_observer(attach_selection_observers)
            // .add_systems(Update, _log_on_selection_change)
            .insert_resource(Selection {
                entity: None,
                observer: None,
            });
    }
}

#[derive(Component)]
pub struct Selectable;

#[derive(Resource)]
pub struct Selection {
    pub entity: Option<Entity>,
    observer: Option<Entity>,
}

fn attach_selection_observers(
    _: On<Add, Selectable>,
    mut commands: Commands,
    selectables: Query<Entity, With<Selectable>>,
    mut selection: ResMut<Selection>,
) {
    if let Some(observer) = selection.observer {
        commands.entity(observer).despawn();
    }
    selection.observer = Some(
        commands
            .spawn(Observer::new(select_entity).with_entities(selectables))
            .id(),
    );
}

fn select_entity(
    click: On<Pointer<Click>>,
    query: Query<(Entity, &Selectable)>,
    mut selection: ResMut<Selection>,
) {
    if query.get(click.entity).is_ok() {
        if let Some(current) = selection.entity
            && current == click.entity
        {
            selection.entity = None;
        } else {
            selection.entity = Some(click.entity);
        }
    } else {
        selection.entity = None;
    }
}

fn _log_on_selection_change(selection: Res<Selection>) {
    if selection.is_changed() {
        if let Some(entity) = selection.entity {
            info!("Selected entity: {:?}", entity);
        } else {
            info!("Deselected all entities");
        }
    }
}
