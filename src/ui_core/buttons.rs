use bevy::asset::Handle;
use bevy::ecs::entity::Entity;
use bevy::ecs::prelude::{Changed, Query, With};
use bevy::prelude::{
    AlignItems, BuildChildren, Button, ButtonBundle, ChildBuilder, Color, Component, Font,
    Interaction, JustifyContent, Style, Text, TextBundle, TextStyle, UiColor, Val,
};
use bevy::ui::{Size, UiRect};

pub const NORMAL_BUTTON: Color = Color::rgb(0.15, 0.15, 0.15);
const HOVERED_BUTTON: Color = Color::rgb(0.25, 0.25, 0.25);
const PRESSED_BUTTON: Color = Color::rgb(0.35, 0.75, 0.35);
const TRANSPARENT: Color = Color::rgba(0.0, 0.0, 0.0, 0.0);

pub mod rect_consts {
    use bevy::ui::{UiRect, Val};
    pub const CENTRED: UiRect<Val> = UiRect {
        left: Val::Auto,
        right: Val::Auto,
        top: Val::Percent(0.0),
        bottom: Val::Px(10.0),
    };
}

pub fn button_system(
    mut interaction_query: Query<
        (&Interaction, &mut UiColor),
        (Changed<Interaction>, With<Button>),
    >,
) {
    for (interaction, mut color) in interaction_query.iter_mut() {
        match *interaction {
            Interaction::Clicked => {
                *color = PRESSED_BUTTON.into();
            }
            Interaction::Hovered => {
                *color = HOVERED_BUTTON.into();
            }
            Interaction::None => {
                *color = NORMAL_BUTTON.into();
            }
        }
    }
}

pub trait ButtonComponent: Component {
    fn to_text(&self) -> &'static str;
}

pub fn make_button<C>(
    button_component: C,
    parent: &mut ChildBuilder,
    font: Handle<Font>,
) -> (Entity, Entity)
where
    C: ButtonComponent,
{
    let button_size = Size::new(Val::Px(150.0), Val::Px(65.0));
    make_button_custom_size(button_component, button_size, parent, font)
}
pub fn make_button_custom_size<C>(
    button_component: C,
    button_size: Size<Val>,
    parent: &mut ChildBuilder,
    font: Handle<Font>,
) -> (Entity, Entity)
where
    C: ButtonComponent,
{
    let mut text_entity = None;
    let text = button_component.to_text();
    let button_entity = parent
        .spawn_bundle(ButtonBundle {
            style: Style {
                size: button_size,
                // center button
                margin: rect_consts::CENTRED,
                padding: (UiRect {
                    left: Val::Percent(0.0),
                    right: Val::Percent(0.0),
                    top: Val::Px(100.0),
                    bottom: Val::Px(100.0),
                }),
                // horizontally center child text
                justify_content: JustifyContent::Center,
                // vertically center child text
                align_items: AlignItems::Center,
                ..Default::default()
            },
            color: NORMAL_BUTTON.into(),
            ..Default::default()
        })
        .insert(button_component)
        .with_children(|parent| {
            let text_entity_ = parent
                .spawn_bundle(TextBundle {
                    text: Text::from_section(
                        text,
                        TextStyle {
                            font,
                            font_size: 40.0,
                            color: Color::rgb(0.9, 0.9, 0.9),
                        },
                    ),
                    ..Default::default()
                })
                .id();
            text_entity = Some(text_entity_);
        })
        .id();
    (button_entity, text_entity.unwrap())
}
