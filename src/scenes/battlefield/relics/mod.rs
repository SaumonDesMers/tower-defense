use std::sync::Arc;

use bevy::prelude::*;

use ricochet::RicochetPlugin;

use crate::scenes::battlefield::relics::ricochet::RicochetRelic;

pub mod ricochet;

pub struct RelicsPlugin;

impl Plugin for RelicsPlugin {
    fn build(&self, app: &mut App) {
        app.add_observer(on_add)
            .add_observer(on_remove)
            .add_plugins(RicochetPlugin);
    }
}

pub trait RelicTrait: Send + Sync + 'static {
    fn name(&self) -> &'static str;
    fn add(&self, commands: &mut EntityCommands);
    fn remove(&self, commands: &mut EntityCommands);
}

#[derive(Component, Clone)]
pub struct Relic(pub Arc<dyn RelicTrait>);

impl PartialEq for Relic {
    fn eq(&self, other: &Self) -> bool {
        Arc::ptr_eq(&self.0, &other.0)
    }
}

#[derive(Component)]
pub struct Relics {
    pub list: Vec<Relic>,
    pub slot_count: usize,
}

impl Relics {
    pub fn new(slot_count: usize) -> Self {
        Self {
            list: vec![],
            slot_count,
        }
    }
}

#[derive(Event)]
pub struct AddRelic {
    pub relic: Relic,
    pub target: Entity,
}

#[derive(Event)]
pub struct RemoveRelic {
    pub relic: Relic,
    pub target: Entity,
}

fn on_add(event: On<AddRelic>, mut commands: Commands, mut relics: Query<&mut Relics>) {
    let Ok(mut relics) = relics.get_mut(event.target) else {
        // warn!("Trying to add relic from entity that can't store ones");
        return;
    };
    if relics.list.len() >= relics.slot_count {
        // warn!("Trying to add relic but no slot left");
        return;
    }
    if relics.list.contains(&event.relic) {
        // warn!("Trying to add relic that is already there");
        return;
    }

    relics.list.push(event.relic.clone());
    event.relic.0.add(&mut commands.entity(event.target));
    info!("Add relic \"{}\"", event.relic.0.name());
}

fn on_remove(event: On<RemoveRelic>, mut commands: Commands, mut relics: Query<&mut Relics>) {
    let Ok(mut relics) = relics.get_mut(event.target) else {
        // warn!("Trying to remove relic from entity that can't store ones");
        return;
    };
    let Some(index) = relics.list.iter().position(|relic| *relic == event.relic) else {
        // warn!("Trying to remove relic that is not there");
        return;
    };

    event.relic.0.remove(&mut commands.entity(event.target));
    relics.list.remove(index);
    info!("Remove relic \"{}\"", event.relic.0.name());
}
