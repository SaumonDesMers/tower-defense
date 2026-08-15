#![allow(unused_imports)]

use avian2d::prelude::*;
use bevy::{log::LogPlugin, prelude::*};
use bevy_egui::EguiPlugin;
use bevy_inspector_egui::quick::WorldInspectorPlugin;

mod camera;
mod scenes;
mod ui;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins.set(LogPlugin {
                level: bevy::log::Level::INFO,
                ..default()
            }),
            PhysicsPlugins::default(),
            PhysicsPickingPlugin,
            camera::CameraPlugin,
            ui::UIPlugin,
            scenes::ScenesPlugin,
        ))
        // .add_plugins(PhysicsDebugPlugin::default())
        // .add_plugins(EguiPlugin::default())
        // .add_plugins(WorldInspectorPlugin::new())
        .insert_resource(RessourcesHandler::default())
        .add_systems(Startup, setup)
        .run();
}

#[derive(Resource, Default)]
struct RessourcesHandler {
    pub tower_body_mesh: Handle<Mesh>,
    pub tower_body_material: Handle<ColorMaterial>,
    pub tower_range_mesh: Handle<Mesh>,
    pub tower_range_material: Handle<ColorMaterial>,
    pub enemy_mesh: Handle<Mesh>,
    pub enemy_material: Handle<ColorMaterial>,
    pub target_mesh: Handle<Mesh>,
    pub target_material: Handle<ColorMaterial>,
    pub obstacle_mesh: Handle<Mesh>,
    pub obstacle_material: Handle<ColorMaterial>,
    pub projectile_mesh: Handle<Mesh>,
    pub projectile_material: Handle<ColorMaterial>,
}

fn setup(
    mut _commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut ressources_handler: ResMut<RessourcesHandler>,
) {
    ressources_handler.tower_body_mesh = meshes.add(Circle::new(20.0));
    ressources_handler.tower_body_material = materials.add(Color::srgb(0.0, 0.0, 0.8));
    ressources_handler.tower_range_mesh = meshes.add(Circle::new(200.0));
    ressources_handler.tower_range_material = materials.add(Color::srgba(0.0, 0.0, 1.0, 0.1));
    ressources_handler.enemy_mesh = meshes.add(Circle::new(5.0));
    ressources_handler.enemy_material = materials.add(Color::srgb(1.0, 0.0, 0.0));
    ressources_handler.target_mesh = meshes.add(Circle::new(10.0));
    ressources_handler.target_material = materials.add(Color::srgb(0.0, 1.0, 0.0));
    ressources_handler.obstacle_mesh = meshes.add(Rectangle::new(50.0, 200.0));
    ressources_handler.obstacle_material = materials.add(Color::srgba(0.0, 0.0, 0.0, 1.0));
    ressources_handler.projectile_mesh = meshes.add(Circle::new(3.0));
    ressources_handler.projectile_material = materials.add(Color::srgb(1.0, 1.0, 0.0));
}
