use crate::game::components::TimedRemoval;
use crate::game::goals::LevelStopwatch;
use crate::game::GameOnlyMarker;
use crate::ui_core::create_centred_texts;
use crate::ui_core::nodes;
use bevy::prelude::*;
use nodes::Property;
use std::time::Duration;

pub fn spawn(
    commands: &mut Commands,
    text_style: TextStyle,
    text_lines: Vec<impl Into<String>>,
    mut level_stopwatch: ResMut<LevelStopwatch>,
) {
    commands
        .spawn_bundle(nodes::new(nodes::defaults::full(
            FlexDirection::Column,
            Some(vec![
                Property::PositionType(PositionType::Absolute),
                Property::Justify(JustifyContent::Center),
                Property::Overflow(Overflow::Hidden),
            ]),
        )))
        .insert(Overlay)
        .insert(GameOnlyMarker)
        .with_children(|parent| {
            parent
                .spawn_bundle(nodes::new(centred_div(FlexDirection::ColumnReverse)))
                .with_children(|parent| {
                    create_centred_texts(parent, text_style.clone(), text_lines, Overlay, None);
                });
        })
        .insert(TimedRemoval::new(Duration::from_secs(10)));

    level_stopwatch.pause();
}

pub fn timer_resume_watcher(query: Query<&Overlay>, mut level_stopwatch: ResMut<LevelStopwatch>) {
    if query.is_empty() && level_stopwatch.paused() {
        level_stopwatch.resume();
    }
}

pub fn overlay_dismiss_system(
    mut commands: Commands,
    query: Query<Entity, With<Overlay>>,
    input: Res<Input<KeyCode>>,
) {
    if !query.is_empty() && input.just_pressed(KeyCode::Return) {
        despawn(&mut commands, &query);
    }
}

fn centred_div(fd: FlexDirection) -> Vec<Property> {
    vec![
        Property::Width(Val::Auto),
        Property::Height(Val::Auto),
        Property::Justify(JustifyContent::Center),
        //Property::Colour(Color::RED),
        Property::MarginAll(Val::Auto),
        Property::Direction(fd),
        Property::Colour(Color::rgba(0.7, 0.7, 0.7, 0.5)),
        Property::PaddingAll(Val::Px(6.0)),
    ]
}

pub fn despawn(commands: &mut Commands, entities: &Query<Entity, With<Overlay>>) {
    for entity in entities.iter() {
        commands.entity(entity).despawn_recursive();
    }
}

#[derive(Component, Clone)]
pub struct Overlay;
