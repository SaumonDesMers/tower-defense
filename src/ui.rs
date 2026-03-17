use bevy::prelude::*;
use bevy::ui::prelude::*;
use bevy::ui_widgets::Activate;

pub struct UIPlugin;

impl Plugin for UIPlugin {
	fn build(&self, app: &mut App) {
		app.add_systems(Update, button_system);
	}
}

fn button_system(
    mut commands: Commands,
    mut interaction_query: Query<(Entity, &Interaction), (Changed<Interaction>, With<Button>)>,
) {
    for (entity, interaction) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                commands.trigger(Activate { entity: entity });
            }
            _ => {}
        }
    }
}
