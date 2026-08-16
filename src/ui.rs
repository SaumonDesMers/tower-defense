use bevy::color::palettes::tailwind;
use bevy::prelude::*;
use bevy::ui::prelude::*;
use bevy::ui_widgets::Activate;

pub struct UIPlugin;

impl Plugin for UIPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, button_system);
    }
}

#[derive(Component, Clone, Copy)]
pub struct ButtonColors {
    base: Srgba,
    hovered: Srgba,
    pressed: Srgba,
}

impl Default for ButtonColors {
    fn default() -> Self {
        Self {
            base: tailwind::SLATE_800,
            hovered: tailwind::SLATE_900,
            pressed: tailwind::SLATE_950,
        }
    }
}

fn button_system(
    mut commands: Commands,
    mut interaction_query: Query<
        (
            Entity,
            &Interaction,
            &mut Button,
            &mut BackgroundColor,
            &mut BorderColor,
            Option<&ButtonColors>,
        ),
        (Changed<Interaction>, With<Button>),
    >,
) {
    for (entity, interaction, mut button, mut background, mut border, colors) in
        &mut interaction_query
    {
        let colors = colors.copied().unwrap_or_default();
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
