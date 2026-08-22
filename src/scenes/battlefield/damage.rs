#![allow(dead_code)]

use bevy::prelude::*;

use crate::scenes::battlefield::damage;

pub struct DamagePlugin;

impl Plugin for DamagePlugin {
    fn build(&self, _app: &mut App) {}
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
