use bevy::prelude::*;
use std::sync::atomic::{AtomicUsize, Ordering};

pub static DEBUG_COLOUR_I: AtomicUsize = AtomicUsize::new(0);
pub const DEBUG_COLOURS: [Color; 5] = [
    Color::RED,
    Color::BLUE,
    Color::GREEN,
    Color::PINK,
    Color::ORANGE,
];

pub fn debug_get_colour() -> Color {
    let mut i = DEBUG_COLOUR_I.load(Ordering::Relaxed) + 1;
    if i >= DEBUG_COLOURS.len() {
        i = 0;
    }
    DEBUG_COLOUR_I.store(i, Ordering::Relaxed);

    let mut c = DEBUG_COLOURS[i];
    c.set_a(0.1);
    c
}

#[derive(Debug, Clone)]
pub enum Property {
    Colour(Color),
    Height(Val),
    Width(Val),
    MarginAll(Val),
    Margin(UiRect<Val>),
    MaxWidth(Val),
    PaddingAll(Val),
    Padding(UiRect<Val>),
    PositionType(PositionType),
    Image(Handle<Image>),
    Justify(JustifyContent),
    Direction(FlexDirection),
    AspectRatio(f32),
    FlexGrow(f32),
    FlexBasis(Val),
    Overflow(Overflow),
}

pub mod defaults {
    use super::*;

    pub fn full(direction: FlexDirection, extra: Option<Vec<Property>>) -> Vec<Property> {
        let mut props = vec![
            Property::Height(Val::Percent(100.0)),
            Property::Width(Val::Percent(100.0)),
            Property::MarginAll(Val::Auto),
            Property::Direction(direction),
        ];
        if let Some(mut extra_props) = extra {
            props.append(&mut extra_props);
        }
        props
    }

    pub fn mini_centred() -> Vec<Property> {
        vec![
            Property::MarginAll(Val::Auto),
            Property::Width(Val::Auto),
            Property::Height(Val::Auto),
            Property::FlexGrow(0.0),
            Property::FlexBasis(Val::Percent(0.0)),
        ]
    }
    pub fn mini_centred_mw() -> Vec<Property> {
        vec![
            Property::MarginAll(Val::Auto),
            Property::Width(Val::Auto),
            Property::Height(Val::Auto),
            Property::FlexGrow(0.0),
            Property::MaxWidth(Val::Percent(98.0)),
            Property::FlexBasis(Val::Percent(0.0)),
        ]
    }
}

#[derive(Debug)]
struct Properties {
    colour: Color,
    height: Val,
    width: Val,
    margin: UiRect<Val>,
    max_size: Size<Val>,
    padding: UiRect<Val>,
    position_type: PositionType,
    image: UiImage,
    justify: JustifyContent,
    direction: FlexDirection,
    aspect_ratio: Option<f32>,
    flex_grow: f32,
    flex_basis: Val,
    overflow: Overflow,
}

impl Default for Properties {
    fn default() -> Self {
        Self {
            colour: Self::default_colour(),
            height: Val::default(),
            width: Val::default(),
            margin: UiRect::all(Val::default()),
            max_size: Default::default(),
            padding: UiRect::all(Val::default()),
            position_type: PositionType::default(),
            image: Default::default(),
            justify: JustifyContent::default(),
            direction: FlexDirection::default(),
            aspect_ratio: None,
            flex_grow: f32::default(),
            flex_basis: Val::default(),
            overflow: Overflow::default(),
        }
    }
}

impl Properties {
    #[cfg(feature = "debug_ui_node_colours")]
    fn default_colour() -> Color {
        debug_get_colour()
    }
    #[cfg(not(feature = "debug_ui_node_colours"))]
    fn default_colour() -> Color {
        Color::hsla(0f32, 0f32, 0f32, 0f32)
    }
    fn set(&mut self, property: Property) {
        match property {
            Property::Colour(color) => self.colour = color,
            Property::Height(val) => self.height = val,
            Property::Width(val) => self.width = val,
            Property::MarginAll(val) => self.margin = UiRect::all(val),
            Property::Margin(rect) => self.margin = rect,
            Property::MaxWidth(val) => self.max_size = Size::new(val, Val::Auto),
            Property::PaddingAll(val) => self.padding = UiRect::all(val),
            Property::Padding(rect) => self.padding = rect,
            Property::PositionType(pos_type) => self.position_type = pos_type,
            Property::Image(image) => self.image = UiImage(image),
            Property::Justify(justify_content) => self.justify = justify_content,
            Property::Direction(flex_direction) => self.direction = flex_direction,
            Property::AspectRatio(aspect_ratio) => self.aspect_ratio = Some(aspect_ratio),
            Property::FlexGrow(flex_grow) => self.flex_grow = flex_grow,
            Property::FlexBasis(flex_basis) => self.flex_basis = flex_basis,
            Property::Overflow(overflow) => self.overflow = overflow,
        }
    }
}

/// Create default node bundle with values overridden by passed properties.
/// A given [Property] enum value can exist multiple times in the vec, the latest one will
/// be applied.
pub fn new(properties: Vec<Property>) -> NodeBundle {
    let mut prop = Properties::default();
    for property in properties.into_iter() {
        prop.set(property);
    }

    NodeBundle {
        style: Style {
            size: Size::new(prop.width, prop.height),
            margin: prop.margin,
            padding: prop.padding,
            justify_content: prop.justify,
            flex_direction: prop.direction,
            aspect_ratio: prop.aspect_ratio,
            flex_grow: prop.flex_grow,
            flex_basis: prop.flex_basis,
            position_type: prop.position_type,
            overflow: prop.overflow,
            ..default()
        },
        color: UiColor(prop.colour),
        image: prop.image,
        ..Default::default()
    }
}
