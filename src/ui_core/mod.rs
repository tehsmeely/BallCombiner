use bevy::ecs::component::Component;
use bevy::hierarchy::{BuildChildren, ChildBuilder};
use bevy::prelude::{FlexDirection, JustifyContent, Size, Text, TextBundle, TextStyle, Val};
use bevy::utils::default;

use crate::ui_core::nodes::Property;
use bevy::ui::{Overflow, Style};

pub mod buttons;
pub mod nodes;

pub fn create_centred_texts<C>(
    parent: &mut ChildBuilder,
    text_style: TextStyle,
    texts: Vec<impl Into<String>>,
    marker: C,
    max_width: Option<f32>,
) where
    C: Component + Clone,
{
    let max_width = match max_width {
        Some(width) => width,
        None => crate::WINDOW_WIDTH / 2.0,
    };
    parent
        .spawn()
        .insert_bundle(nodes::new(vec![
            Property::Justify(JustifyContent::Center),
            //Property::Height(Val::Auto),
            //Property::Height(Val::Percent(100.0)),
            //Property::Height(Val::Percent(0.0)),
            Property::Height(Val::Px((text_style.font_size + 2.0) * (texts.len() as f32))),
            Property::Width(Val::Percent(100.0)),
            Property::Direction(FlexDirection::ColumnReverse),
            Property::Overflow(Overflow::Hidden),
        ]))
        .with_children(|parent| {
            for text in texts.into_iter() {
                parent
                    .spawn_bundle(nodes::new(nodes::defaults::mini_centred_mw()))
                    .with_children(|parent| {
                        parent
                            .spawn_bundle(TextBundle {
                                text: Text::from_section(text, text_style.clone()),
                                style: Style {
                                    max_size: Size::new(Val::Px(max_width), Val::Auto),
                                    ..default()
                                },
                                ..default()
                            })
                            .insert(marker.clone());
                    });
            }
        });
}
