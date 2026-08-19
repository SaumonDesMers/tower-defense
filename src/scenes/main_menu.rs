use bevy::color::palettes::tailwind;
use bevy::prelude::*;
use bevy::ui_widgets::{Activate, observe};

use crate::scenes::AppState;

pub struct MainMenuPlugin;

impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::MainMenu), setup_menu)
            .add_systems(OnExit(AppState::MainMenu), cleanup_menu);
    }
}

#[derive(Component)]
struct MainMenu;

fn setup_menu(mut commands: Commands) {
    info!("Setting up main menu...");
    commands.spawn((
        MainMenu,
        Node {
            width: percent(100),
            height: percent(100),
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            display: Display::Flex,
            flex_direction: FlexDirection::Column,
            row_gap: px(10),
            ..default()
        },
        BackgroundColor(tailwind::INDIGO_900.into()),
        children![
            (
                button("Start Game"),
                observe(
                    |_activate: On<Activate>, mut scene_next_state: ResMut<NextState<AppState>>| {
                        info!("Start Game button clicked!");
                        scene_next_state.set(AppState::InGame);
                    }
                )
            ),
            (
                button("Exit"),
                observe(
                    |_activate: On<Activate>, mut exit: MessageWriter<AppExit>| {
                        exit.write(AppExit::Success);
                    }
                )
            )
        ],
    ));
}

fn cleanup_menu(mut commands: Commands, query: Query<Entity, With<MainMenu>>) {
    info!("Cleaning up main menu...");
    for entity in &query {
        commands.entity(entity).despawn();
    }
}

fn button(text: &str) -> impl Bundle {
    (
        Button,
        Node {
            width: px(300),
            height: px(50),
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            display: Display::Flex,
            flex_direction: FlexDirection::Column,
            row_gap: px(10),
            border: UiRect::all(px(5)),
            border_radius: BorderRadius::all(percent(10)),
            ..default()
        },
        children![(Text::new(text), TextColor(tailwind::SLATE_200.into()),)],
    )
}
