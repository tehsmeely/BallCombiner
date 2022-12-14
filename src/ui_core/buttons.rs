use bevy::asset::{AssetServer, Handle};
use bevy::ecs::entity::Entity;
use bevy::ecs::prelude::{Changed, Query, With};
use bevy::math::Vec2;
use bevy::prelude::*;
use bevy::render::prelude::Image;
use bevy::ui::{Node, Size, UiImage, UiRect};
use std::ops::Not;
use std::time::Duration;

pub const NORMAL_BUTTON: Color = Color::rgb(0.15, 0.15, 0.15);
const HOVERED_BUTTON: Color = Color::rgb(0.25, 0.25, 0.25);
const PRESSED_BUTTON: Color = Color::rgb(0.35, 0.75, 0.35);

pub const NORMAL_IMAGE_BUTTON: Color = Color::rgb(1.0, 1.0, 1.0);
const HOVERED_IMAGE_BUTTON: Color = Color::rgb(0.8, 1.0, 1.0);
const PRESSED_IMAGE_BUTTON: Color = Color::rgb(0.5, 0.5, 0.5);

#[derive(Component)]
pub struct ImageButton;

pub mod rect_consts {
    use bevy::ui::{UiRect, Val};
    pub const CENTRED: UiRect<Val> = UiRect {
        left: Val::Auto,
        right: Val::Auto,
        top: Val::Percent(0.0),
        bottom: Val::Px(10.0),
    };
}

pub type InteractionColorButton = (
    &'static Interaction,
    &'static mut UiColor,
    Option<&'static ImageButton>,
);
pub type ChangedInteractionButton = (Changed<Interaction>, With<Button>);

pub fn button_system(
    mut interaction_query: Query<InteractionColorButton, ChangedInteractionButton>,
) {
    for (interaction, mut color, maybe_image_button) in interaction_query.iter_mut() {
        let is_image_button = maybe_image_button.is_some();
        let colour = match *interaction {
            Interaction::Clicked => {
                if is_image_button {
                    PRESSED_IMAGE_BUTTON
                } else {
                    PRESSED_BUTTON
                }
            }
            Interaction::Hovered => {
                if is_image_button {
                    HOVERED_IMAGE_BUTTON
                } else {
                    HOVERED_BUTTON
                }
            }
            Interaction::None => {
                if is_image_button {
                    NORMAL_IMAGE_BUTTON
                } else {
                    NORMAL_BUTTON
                }
            }
        };
        *color = UiColor(colour);
    }
}

pub fn checkbox_button_system(
    mut query: Query<(&Interaction, &mut Checkbox, &mut UiImage), Changed<Interaction>>,
    time: Res<Time>,
    mut event_writer: EventWriter<CheckboxEvent>,
) {
    for (interaction, mut checkbox, mut ui_image) in query.iter_mut() {
        checkbox.debounce_timer.tick(time.delta());
        if let Interaction::Clicked = interaction {
            if checkbox.debounce_timer.finished() {
                checkbox.toggle();
                *ui_image = UiImage(checkbox.to_current_image());
                checkbox.debounce_timer.reset();
                println!("Clicking checkbox");
                event_writer.send(CheckboxEvent {
                    variant: checkbox.variant,
                    new_state: checkbox.state,
                })
            } else {
                println!("Not toggling checkbox, debounce");
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
pub fn make_button_custom_image(
    button_component: impl Component,
    button_image: Handle<Image>,
    parent: &mut ChildBuilder,
    button_size: Vec2,
    padding: Option<UiRect<Val>>,
    margin: Option<UiRect<Val>>,
) -> Entity {
    let padding = match padding {
        Some(padding) => padding,
        None => UiRect {
            left: Val::Percent(0.0),
            right: Val::Percent(0.0),
            top: Val::Px(100.0),
            bottom: Val::Px(100.0),
        },
    };
    let margin = match margin {
        Some(margin) => margin,
        None => rect_consts::CENTRED,
    };
    parent
        .spawn_bundle(ButtonBundle {
            node: Node { size: button_size },
            style: Style {
                size: Size::new(Val::Px(button_size.x), Val::Px(button_size.y)),
                // center button
                margin,
                padding,
                // horizontally center child text
                justify_content: JustifyContent::Center,
                // vertically center child text
                align_items: AlignItems::Center,
                ..Default::default()
            },
            image: UiImage(button_image),
            color: NORMAL_IMAGE_BUTTON.into(),
            ..Default::default()
        })
        .insert(ImageButton)
        .insert(button_component)
        .id()
}

#[derive(Debug, Clone, Copy)]
pub enum CheckboxVariant {
    SFX,
    Music,
}
impl CheckboxVariant {
    pub fn to_checked_unchecked_filename(&self) -> (&str, &str) {
        match self {
            Self::SFX => ("checkbox/sfx_checked.png", "checkbox/sfx_unchecked.png"),
            Self::Music => ("checkbox/music_checked.png", "checkbox/music_unchecked.png"),
        }
    }
}

pub struct CheckboxEvent {
    pub variant: CheckboxVariant,
    pub new_state: CheckboxState,
}

#[derive(Debug, Clone, Copy)]
pub enum CheckboxState {
    Checked,
    Unchecked,
}

impl Not for CheckboxState {
    type Output = Self;

    fn not(self) -> Self::Output {
        match self {
            Self::Checked => Self::Unchecked,
            Self::Unchecked => Self::Checked,
        }
    }
}

#[derive(Debug, Component)]
pub struct Checkbox {
    variant: CheckboxVariant,
    checked_image: Handle<Image>,
    unchecked_image: Handle<Image>,
    state: CheckboxState,
    debounce_timer: Timer,
}

impl Checkbox {
    fn new(
        variant: CheckboxVariant,
        checked_image: Handle<Image>,
        unchecked_image: Handle<Image>,
    ) -> Self {
        Self {
            variant,
            checked_image,
            unchecked_image,
            state: CheckboxState::Checked,
            debounce_timer: Timer::new(Duration::from_micros(190), false),
        }
    }

    fn toggle(&mut self) {
        self.state = !self.state;
    }
    fn to_current_image(&self) -> Handle<Image> {
        match self.state {
            CheckboxState::Checked => self.checked_image.clone(),
            CheckboxState::Unchecked => self.unchecked_image.clone(),
        }
    }
}

pub fn make_checkbox(
    parent: &mut ChildBuilder,
    variant: CheckboxVariant,
    asset_server: &AssetServer,
) -> Entity {
    let (checked, unchecked) = variant.to_checked_unchecked_filename();
    let checkbox = Checkbox::new(
        variant,
        asset_server.load(checked),
        asset_server.load(unchecked),
    );
    parent
        .spawn_bundle(ButtonBundle {
            style: Style {
                size: Size::new(Val::Px(40.0), Val::Px(20.0)),
                margin: rect_consts::CENTRED,
                // horizontally center child text
                justify_content: JustifyContent::Center,
                // vertically center child text
                align_items: AlignItems::Center,
                ..Default::default()
            },
            image: UiImage(checkbox.to_current_image()),
            color: NORMAL_IMAGE_BUTTON.into(),
            ..Default::default()
        })
        .insert(checkbox)
        .insert(ImageButton)
        //.insert(button_component)
        .id()
}
