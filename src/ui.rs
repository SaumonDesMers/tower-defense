use std::time::Duration;

use bevy::color::palettes::tailwind;
use bevy::ecs::system::command;
use bevy::prelude::*;
use bevy::ui::prelude::*;
use bevy::ui_widgets::Activate;
use bevy::window::WindowResized;

pub struct UIPlugin;

impl Plugin for UIPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (button_system, change_scaling))
            .add_observer(on_button_enable);
    }
}

#[derive(Component)]
struct ButtonDisabled(i32);

#[derive(EntityEvent)]
pub struct EnableButtonEvent {
    pub entity: Entity,
    pub enable: bool,
}

#[derive(Component, Clone, Copy)]
pub struct ButtonColors {
    base: Srgba,
    hovered: Srgba,
    pressed: Srgba,
    disabled: Srgba,
}

impl Default for ButtonColors {
    fn default() -> Self {
        Self {
            base: tailwind::SLATE_800,
            hovered: tailwind::SLATE_900,
            pressed: tailwind::SLATE_950,
            disabled: tailwind::GRAY_950,
        }
    }
}

fn button_system(
    mut commands: Commands,
    mut button_query: Query<
        (
            Entity,
            &Interaction,
            Has<ButtonDisabled>,
            &mut Button,
            &mut BackgroundColor,
            &mut BorderColor,
            Option<&ButtonColors>,
        ),
        Changed<Interaction>,
    >,
) {
    for (entity, interaction, disabled, mut button, mut background, mut border, colors) in
        &mut button_query
    {
        let colors = colors.copied().unwrap_or_default();

        if disabled {
            *background = colors.disabled.into();
            *border = colors.disabled.into();
            continue;
        }

        match *interaction {
            Interaction::Pressed => {
                commands.trigger(Activate { entity: entity });
                *background = colors.pressed.into();
                *border = colors.pressed.into();
                button.set_changed();
            }
            Interaction::Hovered => {
                *background = colors.hovered.into();
                *border = colors.hovered.into();
                button.set_changed();
            }
            Interaction::None => {
                *background = colors.base.into();
                *border = colors.base.into();
            }
        }
    }
}

fn on_button_enable(
    event: On<EnableButtonEvent>,
    mut commands: Commands,
    mut button_query: Query<(
        Entity,
        Option<&mut ButtonDisabled>,
        &mut BackgroundColor,
        &mut BorderColor,
        &Children,
        Option<&ButtonColors>,
    )>,
    mut text_query: Query<&mut TextColor>,
) {
    let (entity, disabled, mut background, mut border, children, colors) = button_query
        .get_mut(event.entity)
        .expect("Entity stored by event should be valid.");
    let colors = colors.copied().unwrap_or_default();

    if let Some(mut disabled) = disabled {
        disabled.0 += if event.enable { -1 } else { 1 };

        if disabled.0 == 0 {
            commands.entity(entity).remove::<ButtonDisabled>();

            *background = colors.base.into();
            *border = colors.base.into();
            if let Ok(mut text_color) = text_query.get_mut(children[0]) {
                *text_color = TextColor(tailwind::SLATE_200.into());
            }
        }
    } else if !event.enable {
        commands.entity(entity).insert(ButtonDisabled(1));

        *background = colors.disabled.into();
        *border = colors.disabled.into();
        if let Ok(mut text_color) = text_query.get_mut(children[0]) {
            *text_color = TextColor(tailwind::SLATE_500.into());
        }
    } else {
        warn!("Trying to enable an already enabled button.");
    }
}

fn change_scaling(mut resize_reader: MessageReader<WindowResized>, mut ui_scale: ResMut<UiScale>) {
    let target_width = 1920.0;
    for event in resize_reader.read() {
        ui_scale.0 = event.width / target_width;
    }
}
