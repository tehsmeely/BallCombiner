use crate::ui_core::buttons;
use crate::ui_core::nodes;
use crate::GameState;
use bevy::app::AppExit;
use bevy::prelude::*;
use bevy::utils::tracing::subscriber::with_default;
use nodes::Property;

pub struct MenuPlugin;

#[derive(Component, Clone)]
struct MenuOnlyMarker;

impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_enter(GameState::Menu).with_system(setup))
            .add_system_set(SystemSet::on_update(GameState::Menu).with_system(button_system))
            .add_system_set(SystemSet::on_exit(GameState::Menu).with_system(cleanup));
    }
}

const HALF_PANE: [Property; 4] = [
    Property::MarginAll(Val::Auto),
    Property::Height(Val::Percent(100.0)),
    Property::Width(Val::Percent(50.0)),
    Property::Direction(FlexDirection::ColumnReverse),
];
fn button_padding() -> UiRect<Val> {
    UiRect::new(Val::Px(0.0), Val::Px(0.0), Val::Px(100.0), Val::Px(500.0))
}
fn button_margin() -> UiRect<Val> {
    UiRect::new(Val::Px(0.0), Val::Px(0.0), Val::Px(10.0), Val::Px(50.0))
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let play_image = asset_server.load("buttons/play.png");
    let quit_image = asset_server.load("buttons/quit.png");

    println!("Menu Setup");

    let left_text_style = TextStyle {
        font: asset_server.load("Quicksand-Regular.ttf"),
        font_size: 28.0,
        color: Default::default(),
    };
    commands
        .spawn_bundle(nodes::new(nodes::defaults::full(
            FlexDirection::Row,
            Some(vec![/*Property::Colour(Color::RED)*/]),
        )))
        .insert(MenuOnlyMarker)
        .with_children(|parent| {
            // Left Panel
            parent
                .spawn_bundle(nodes::new(HALF_PANE.to_vec()))
                .with_children(|parent| {
                    parent
                        .spawn_bundle(nodes::new(vec![
                            Property::Height(Val::Percent(100.0)),
                            Property::Width(Val::Percent(100.0)),
                            Property::Direction(FlexDirection::Column),
                            Property::Justify(JustifyContent::Center),
                        ]))
                        .with_children(|parent| {
                            crate::ui_core::create_centred_texts(
                                parent,
                                left_text_style,
                                LEFT_TEXT.to_vec(),
                                MenuOnlyMarker,
                            )
                        });
                });

            // Right Panel
            parent
                .spawn_bundle(nodes::new(HALF_PANE.to_vec()))
                .with_children(|parent| {
                    parent
                        .spawn_bundle(nodes::new(vec![
                            Property::MarginAll(Val::Auto),
                            Property::Height(Val::Auto),
                            Property::Width(Val::Auto),
                            Property::Direction(FlexDirection::ColumnReverse),
                        ]))
                        .with_children(|parent| {
                            buttons::make_button_custom_image(
                                MenuButton::Play,
                                play_image,
                                parent,
                                Vec2::new(110f32, 68f32),
                                Some(button_padding()),
                                Some(button_margin()),
                            );
                            buttons::make_button_custom_image(
                                MenuButton::Quit,
                                quit_image,
                                parent,
                                Vec2::new(110f32, 68f32),
                                Some(button_padding()),
                                Some(button_margin()),
                            );
                        });
                });
        });
}

const LEFT_TEXT: [&str; 10] = [
    "Combine!",
    "",
    "",
    "",
    "",
    "",
    "Mix the two ingredients in the desired ratio",
    "You have all the time in the world ... until the minimum weight is reached",
    "When hit, the countdown will start ticking",
    "When the countdown hits zero, you'll get points for how close to the target mix it is",
];

#[derive(Component)]
pub enum MenuButton {
    Play,
    Quit,
}

pub fn button_system(
    buttons: Query<(&MenuButton, &Interaction), Changed<Interaction>>,
    mut state: ResMut<State<GameState>>,
    mut exit: EventWriter<AppExit>,
) {
    for (button, interaction) in buttons.iter() {
        match interaction {
            Interaction::Clicked => match button {
                MenuButton::Play => state.set(GameState::Game).unwrap(),
                MenuButton::Quit => exit.send(AppExit),
            },
            Interaction::Hovered | Interaction::None => (),
        }
    }
}

fn cleanup(mut commands: Commands, entities: Query<Entity, With<MenuOnlyMarker>>) {
    for entity in entities.iter() {
        commands.entity(entity).despawn_recursive();
    }
}
