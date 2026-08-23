use bevy::prelude::*;

#[derive(Component)]
pub struct Level {
    pub count: u32,
    pub price: f32,
    pub price_mul: f32,
}

#[derive(EntityEvent)]
pub struct UpgradeEvent {
    pub entity: Entity,
    pub _new_level: u32,
}
