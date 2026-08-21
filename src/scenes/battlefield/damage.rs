#![allow(dead_code)]

use bevy::prelude::*;

use crate::scenes::battlefield::{damage, upgrade::Upgrade};

pub struct DamagePlugin;

impl Plugin for DamagePlugin {
    fn build(&self, app: &mut App) {
        app.add_observer(on_damage_upgrade);
    }
}

#[derive(Component, Debug, Clone, Copy)]
pub struct Damage {
    pub amount: f32,
}

impl Damage {
    pub fn new(amount: f32) -> Self {
        Self { amount }
    }
}

#[derive(EntityEvent, Clone)]
pub struct DamageUpgrade {
    entity: Entity,
}

impl DamageUpgrade {
    pub fn new() -> Self {
        Self {
            entity: Entity::PLACEHOLDER,
        }
    }
}

impl Upgrade for DamageUpgrade {
    fn trigger(&self, commands: &mut Commands, entity: Entity) {
        commands.trigger(Self { entity, ..*self });
    }

    fn text(&self) -> String {
        String::from("Damage: +1")
    }
}

fn on_damage_upgrade(event: On<DamageUpgrade>, mut damage_q: Query<&mut Damage>) {
    info!("on_damage_upgrade");
    if let Ok(mut damage) = damage_q.get_mut(event.entity) {
        damage.amount += 1.0;
    }
}
