use bevy::log::tracing_subscriber::field::debug;
use bevy::prelude::*;
use bevy::input_focus::InputFocus;
use bevy::color::palettes::tailwind;
use bevy::ui::prelude::*;
use bevy::ui_widgets::{observe, Activate};

use crate::scenes::SceneState;

pub struct MainMenuPlugin;

impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(OnEnter(SceneState::Menu), setup_menu)
            .add_systems(OnExit(SceneState::Menu), cleanup_menu)
            .add_systems(Update, button_system);
    }
}

#[derive(Component)]
struct MainMenu;

#[derive(Component)]
struct MainMenuButton;

fn setup_menu(
    mut commands: Commands,
) {
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
        BackgroundColor(tailwind::ORANGE_200.into()),
        children![
            (
                button("Start Game"),
                observe(|_activate: On<Activate>, mut scene_next_state: ResMut<NextState<SceneState>>| {
                    info!("Start Game button clicked!");
                    scene_next_state.set(SceneState::Game);
                })
            ),
            (
                button("Options"),
                observe(|_activate: On<Activate>| {
                    info!("Options button clicked!");
                })
            ),
            (
                button("Exit"),
                observe(|_activate: On<Activate>, mut exit: MessageWriter<AppExit>| {
                    exit.write(AppExit::Success);
                })
            )
        ],
    ));
}

fn cleanup_menu(
    mut commands: Commands,
    query: Query<Entity, With<MainMenu>>,
) {
    info!("Cleaning up main menu...");
    for entity in &query {
        commands.entity(entity).despawn();
    }
}

const NORMAL_BUTTON: Srgba = tailwind::ORANGE_300;
const HOVERED_BUTTON: Srgba = tailwind::ORANGE_400;
const PRESSED_BUTTON: Srgba = tailwind::ORANGE_500;

fn button_system(
    mut interaction_query: Query<
        (
            &Interaction,
            &mut BackgroundColor,
            &mut BorderColor,
            &mut Button,
        ),
        (
            Changed<Interaction>,
            With<MainMenuButton>,
        ),
    >,
) {
    for (interaction, mut color, mut border_color, mut button) in
        &mut interaction_query
    {
        match *interaction {
            Interaction::Pressed => {
                *color = PRESSED_BUTTON.into();
                *border_color = BorderColor::all(tailwind::RED_400);
                button.set_changed();
            }
            Interaction::Hovered => {
                *color = HOVERED_BUTTON.into();
                *border_color = BorderColor::all(tailwind::ORANGE_500);
                button.set_changed();
            }
            Interaction::None => {
                *color = NORMAL_BUTTON.into();
                *border_color = BorderColor::all(tailwind::ORANGE_400);
            }
        }
    }
}

fn button(
    text: &str
) -> impl Bundle {
    (
        Button,
        MainMenuButton,
        Node {
            width: px(300),
            height: px(50),
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            display: Display::Flex,
            flex_direction: FlexDirection::Column,
            row_gap: px(10),
            border: UiRect::all(px(5)),
            border_radius: BorderRadius::all(percent(20)),
            ..default()
        },
        BackgroundColor::from(tailwind::ORANGE_300),
        BorderColor::all(tailwind::ORANGE_400),
        children![
            (
                Text::new(text),
                TextColor(tailwind::ORANGE_100.into()),
            )
        ]
    )
}