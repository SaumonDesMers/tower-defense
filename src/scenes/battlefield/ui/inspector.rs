use bevy::ui::prelude::*;
use bevy::{color::palettes::tailwind, prelude::*};

use crate::scenes::battlefield::attack_range::{AttackRange, AttackRangeType};
use crate::scenes::battlefield::attack_speed::{self, AttackSpeed};
use crate::scenes::battlefield::base::Base;
use crate::scenes::battlefield::health::{Damage, Health};
use crate::scenes::battlefield::selection::{Selectable, Selection};
use crate::scenes::battlefield::ui::inspector;

pub struct InspectorPlugin;

impl Plugin for InspectorPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            update_inspector.run_if(resource_exists_and_changed::<Selection>),
        );
    }
}

#[derive(Component)]
struct Inspector;

pub fn inspector_window() -> impl Bundle {
    (
        Node {
            width: px(400),
            height: px(600),
            border_radius: BorderRadius::all(px(20)),
            align_self: AlignSelf::Center,
            margin: UiRect {
                right: px(10),
                top: px(10),
                bottom: px(10),
                ..UiRect::default()
            },

            display: Display::Flex,
            flex_direction: FlexDirection::Column,
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            row_gap: px(10),
            ..default()
        },
        BackgroundColor(tailwind::INDIGO_900.into()),
        Inspector,
        Visibility::Inherited,
        children![(
            Text::new("Inspector"),
            TextColor(tailwind::SLATE_200.into()),
        )],
    )
}

fn update_inspector(
    mut inspector: Query<(&mut Visibility, &Children), With<Inspector>>,
    mut text: Query<&mut Text>,
    selectables: Query<
        (
            &Name,
            Option<&Health>,
            Option<&AttackSpeed>,
            Option<&Damage>,
            Option<&AttackRange>,
        ),
        With<Selectable>,
    >,
    selection: Res<Selection>,
) {
    let (mut visibility, children) = inspector.single_mut().expect("Inpector should exist");

    let mut text = text
        .get_mut(
            *children
                .get(0)
                .expect("Inspector should have at least one child"),
        )
        .expect("Inpector text should exist");

    let Some(selected) = selection.entity else {
        *visibility = Visibility::Hidden;
        return;
    };

    *visibility = Visibility::Inherited;

    let (name, health, attack_speed, damage, range) = selectables
        .get(selected)
        .expect("Selected entity should exist");

    **text = name.to_string() + "\n";
    if let Some(health) = health {
        text.push_str(&format!("\nHealth: {}/{}", health.current, health.max));
    }
    if let Some(attack_speed) = attack_speed {
        text.push_str(&format!("\nAttack speed: {}", attack_speed.per_second()));
    }
    if let Some(damage) = damage {
        text.push_str(&format!("\nDamage: {}", damage.amount));
    }
    if let Some(range) = range {
        text.push_str(&match range.range_type {
            AttackRangeType::Circle(radius) => format!("\nRange radius: {}", radius),
        });
    }
}
