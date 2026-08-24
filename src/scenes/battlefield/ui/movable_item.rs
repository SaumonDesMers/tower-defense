use bevy::ui::prelude::*;
use bevy::{color::palettes::tailwind, prelude::*};

pub struct MovableItemPlugin;

impl Plugin for MovableItemPlugin {
    fn build(&self, app: &mut App) {
        app.add_observer(on_pick)
            .add_observer(on_drag)
            .add_observer(on_drop)
            .add_observer(on_enter)
            .add_observer(on_leave);
    }
}

#[derive(Component)]
pub struct MovableItemSlot;

#[derive(Component)]
pub struct MovableItem;

#[derive(Component)]
struct MovableItemLastParent(Entity);

fn on_pick(
    pick: On<Pointer<DragStart>>,
    mut commands: Commands,
    mut query: Query<Option<&ChildOf>, With<MovableItem>>,
) {
    let Ok(child_of) = query.get_mut(pick.entity) else {
        return;
    };
    if let Some(parent) = child_of {
        commands.entity(parent.0).detach_child(pick.entity);
        commands
            .entity(pick.entity)
            .insert((MovableItemLastParent(parent.0), Pickable::IGNORE));
    }
}

fn on_drag(
    drag: On<Pointer<Drag>>,
    mut node: Query<(&mut Node, &ComputedNode), With<MovableItem>>,
    ui_scale: Res<UiScale>,
) {
    let Ok((mut node, computed_node)) = node.get_mut(drag.entity) else {
        return;
    };

    node.left = px((drag.pointer_location.position.x - computed_node.size.x / 2.0) / ui_scale.0);
    node.top = px((drag.pointer_location.position.y - computed_node.size.y / 2.0) / ui_scale.0);
}

fn on_drop(
    mut drop: On<Pointer<DragDrop>>,
    mut commands: Commands,
    slot: Query<(), With<MovableItemSlot>>,
    mut movable: Query<(&mut Node, &MovableItemLastParent), With<MovableItem>>,
) {
    let Ok((mut node, last_parent)) = movable.get_mut(drop.dropped) else {
        return;
    };
    node.left = px(0);
    node.top = px(0);

    if slot.contains(drop.entity) {
        commands.entity(drop.entity).add_child(drop.dropped);
    } else {
        commands.entity(last_parent.0).add_child(drop.dropped);
    }
    commands
        .entity(drop.dropped)
        .remove::<MovableItemLastParent>();
    commands.entity(drop.dropped).insert(Pickable::default());
    drop.propagate(false);
}

fn on_enter(
    mut enter: On<Pointer<DragEnter>>,
    mut slot: Query<&mut BackgroundColor, With<MovableItemSlot>>,
    movable: Query<(), With<MovableItem>>,
) {
    if !movable.contains(enter.dragged) {
        return;
    }

    let Ok(mut color) = slot.get_mut(enter.entity) else {
        return;
    };

    *color = BackgroundColor(tailwind::RED_950.into());
    enter.propagate(false);
}

fn on_leave(
    mut leave: On<Pointer<DragLeave>>,
    mut slot: Query<&mut BackgroundColor, With<MovableItemSlot>>,
    movable: Query<(), With<MovableItem>>,
) {
    if !movable.contains(leave.dragged) {
        return;
    }

    let Ok(mut color) = slot.get_mut(leave.entity) else {
        return;
    };

    *color = BackgroundColor(tailwind::INDIGO_950.into());
    leave.propagate(false);
}
