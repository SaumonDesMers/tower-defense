#![allow(dead_code)]

use bevy::prelude::*;

use crate::scenes::battlefield::{damage, upgrade::UpgradeEvent};

pub struct DamagePlugin;

impl Plugin for DamagePlugin {
    fn build(&self, app: &mut App) {
        app.add_observer(on_upgrade);
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

fn on_upgrade(event: On<UpgradeEvent>, mut damage_q: Query<&mut Damage>) {
    if let Ok(mut damage) = damage_q.get_mut(event.entity) {
        damage.amount += 1.0;
    }
}
