use avian2d::prelude::*;
use bevy::{prelude::*, sprite_render::AlphaMode2d};

use crate::scenes::{
    AppState,
    battlefield::{
        BattleFieldSet,
        enemy::Enemy,
        health::{Damage, Health},
        wave::WavePhase,
    },
};

pub struct ClickAttackPlugin;

impl Plugin for ClickAttackPlugin {
    fn build(&self, app: &mut App) {
        app.add_observer(on_click_attack.run_if(in_state(AppState::InGame)))
            .add_observer(
                attack_on_click
                    .run_if(in_state(AppState::InGame))
                    .run_if(in_state(WavePhase::Spawning).or_else(in_state(WavePhase::Killing))),
            )
            .add_systems(Update, (apply_damage, fade_out).in_set(BattleFieldSet));
    }
}

#[derive(Resource)]
pub struct ClickAttackGlobalData {
    pub damage: Damage,
    pub mesh: Handle<Mesh>,
}

#[derive(Event)]
pub struct ClickAttackEvent {
    position: Vec2,
    radius: f32,
}

impl ClickAttackEvent {
    pub fn new(position: Vec2, radius: f32) -> Self {
        Self { position, radius }
    }
}

#[derive(Component)]
struct ApplyDamage;

fn attack_on_click(
    mut event: On<Pointer<Release>>,
    mut commands: Commands,
    camera_query: Query<(&Camera, &GlobalTransform)>,
) {
    // info!("Trigger ClickAttackEvent ({})", event.entity);
    if event.button == PointerButton::Primary {
        let (camera, camera_transform) = camera_query.single().expect("Camera should exist.");

        if let Ok(pos) =
            camera.viewport_to_world_2d(camera_transform, event.pointer_location.position)
        {
            commands.trigger(ClickAttackEvent::new(pos, 100.0));
        } else {
            warn!("Missing position.")
        }
    }
    event.propagate(false);
}

fn on_click_attack(
    click_attack: On<ClickAttackEvent>,
    mut commands: Commands,
    mut materials: ResMut<Assets<ColorMaterial>>,
    click_attack_data: Res<ClickAttackGlobalData>,
) {
    // info!("Spawn clik attack entity at ({})", click_attack.position);
    commands.spawn((
        Transform::from_xyz(click_attack.position.x, click_attack.position.y, 0.0),
        Mesh2d(click_attack_data.mesh.clone()),
        MeshMaterial2d(materials.add(ColorMaterial {
            color: Color::srgba(1.0, 0.0, 0.0, 0.5),
            alpha_mode: AlphaMode2d::Blend,
            ..default()
        })),
        Collider::from(Circle::new(click_attack.radius)),
        Sensor,
        ApplyDamage,
        FadeOut(4.0),
    ));
}

fn apply_damage(
    mut commands: Commands,
    attack_query: Query<Entity, With<ApplyDamage>>,
    mut enemy_query: Query<(Entity, &mut Health), With<Enemy>>,
    collisions: Collisions,
    click_attack_data: Res<ClickAttackGlobalData>,
) {
    for attack in attack_query {
        for (enemy, mut health) in &mut enemy_query {
            if collisions.contains(attack, enemy) {
                health.take_damage(click_attack_data.damage);
            }
        }
        commands.entity(attack).remove::<ApplyDamage>();
    }
}

#[derive(Component)]
struct FadeOut(f32);

fn fade_out(
    mut commands: Commands,
    time: Res<Time>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut query: Query<(Entity, &MeshMaterial2d<ColorMaterial>, &FadeOut)>,
) {
    for (entity, mat_handle, fade) in &mut query {
        if let Some(mut material) = materials.get_mut(mat_handle) {
            let current_alpha = material.color.alpha();
            let new_alpha = (current_alpha - fade.0 * time.delta_secs()).max(0.0);

            if new_alpha <= 0.0 {
                commands.entity(entity).despawn();
            } else {
                material.color.set_alpha(new_alpha);
            }
        }
    }
}
