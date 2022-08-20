use crate::ui_core::nodes;
use bevy::prelude::*;
use nodes::Property;

pub fn setup_ui(mut commands: Commands, asset_server: Res<AssetServer>) {
    let text_style = TextStyle {
        font: asset_server.load("Quicksand-Regular.ttf"),
        font_size: 20.0,
        color: Default::default(),
    };
    commands
        .spawn_bundle(nodes::new(nodes::defaults::full(
            FlexDirection::ColumnReverse,
            Some(vec![Property::Colour(Color::rgba(0.0, 0.0, 0.0, 0.0))]),
        )))
        .with_children(|parent| {
            parent
                .spawn_bundle(nodes::new(vec![
                    Property::Height(Val::Px(80.0)),
                    Property::Width(Val::Percent(100.0)),
                    Property::Colour(Color::rgba(0.0, 1.0, 0.0, 0.2)),
                    Property::Justify(JustifyContent::Center),
                ]))
                .with_children(|parent| {
                    parent
                        .spawn_bundle(nodes::new(nodes::defaults::mini_centred()))
                        .with_children(|parent| {
                            parent.spawn_bundle(TextBundle::from_section("Game UI!", text_style));
                        });
                });
        });
}
