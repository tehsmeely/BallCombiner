use crate::ui_core::buttons;
use crate::ui_core::nodes;
use crate::{GameState, TotalScore};
use bevy::app::AppExit;
use bevy::prelude::*;

use crate::game::not_a_cup::spawn_jar;
use crate::game::BallKind;
use crate::ui_core::buttons::CheckboxVariant;
use nodes::Property;

pub struct MenuPlugin;

#[derive(Component, Clone)]
struct MenuOnlyMarker;

impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(
            SystemSet::on_enter(GameState::Menu)
                .with_system(setup)
                .with_system(debug_tank_setup),
        )
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

fn debug_tank_setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    //spawn_jar(0.0, BallKind::Blue, &mut commands, &asset_server);
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    windows: Res<Windows>,
    total_score: Res<TotalScore>,
) {
    let play_image = asset_server.load("buttons/play.png");
    let quit_image = asset_server.load("buttons/quit.png");
    let reset_score_image = asset_server.load("buttons/reset_score.png");

    println!("Menu Setup");

    let mut window_width = windows.get_primary().unwrap().width();

    if cfg!(target_arch = "wasm32") {
        // Don't trust initial window width in wasm
        if window_width > crate::WINDOW_WIDTH {
            window_width = crate::WINDOW_WIDTH
        }
    }

    let font_size = if cfg!(target_arch = "wasm32") {
        22.0
    } else {
        28.0
    };

    let left_text_style = TextStyle {
        font: asset_server.load("Quicksand-Regular.ttf"),
        font_size,
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
                                left_text_style.clone(),
                                LEFT_TEXT.to_vec(),
                                MenuOnlyMarker,
                                Some((window_width / 2.0) - 20.0),
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
                                MenuButton::Reset,
                                reset_score_image,
                                parent,
                                Vec2::new(110f32, 68f32),
                                Some(button_padding()),
                                Some(button_margin()),
                            );
                            if !cfg!(target_arch = "wasm32") {
                                buttons::make_button_custom_image(
                                    MenuButton::Quit,
                                    quit_image,
                                    parent,
                                    Vec2::new(110f32, 68f32),
                                    Some(button_padding()),
                                    Some(button_margin()),
                                );
                            }

                            parent
                                .spawn_bundle(nodes::new(vec![
                                    Property::Height(Val::Auto),
                                    Property::Width(Val::Auto),
                                    Property::Direction(FlexDirection::Row),
                                    Property::Justify(JustifyContent::Center),
                                ]))
                                .with_children(|parent| {
                                    buttons::make_checkbox(
                                        parent,
                                        CheckboxVariant::Music,
                                        &asset_server,
                                    );
                                    buttons::make_checkbox(
                                        parent,
                                        CheckboxVariant::SFX,
                                        &asset_server,
                                    );
                                });
                        });
                });
        });

    // Bottom panel
    commands
        .spawn_bundle(nodes::new(vec![
            Property::Height(Val::Px(40.0)),
            Property::Width(Val::Percent(100.0)),
            Property::PositionType(PositionType::Absolute),
            Property::Justify(JustifyContent::Center),
        ]))
        .insert(MenuOnlyMarker)
        .with_children(|parent| {
            parent.spawn_bundle(TextBundle::from_section(
                format!("{}", *total_score),
                left_text_style,
            ));
        });
}

const LEFT_TEXT: [&str; 10] = [
    "Frantic Lab Tech",
    "",
    "",
    "",
    "",
    "",
    "The scientists need you to mix the two ingredients in the desired ratio.",
    "Their patience is unlimited... That is, until the minimum weight is reached at which point they're coming to get it, fast!",
    "When minimum weight hit, the countdown will start ticking.",
    "When the countdown hits zero, they award you points for how close to the target mix it is.",
];

#[derive(Component)]
pub enum MenuButton {
    Play,
    Quit,
    Reset,
}

pub fn button_system(
    buttons: Query<(&MenuButton, &Interaction), Changed<Interaction>>,
    mut state: ResMut<State<GameState>>,
    mut exit: EventWriter<AppExit>,
    mut total_score: ResMut<TotalScore>,
) {
    for (button, interaction) in buttons.iter() {
        match interaction {
            Interaction::Clicked => match button {
                MenuButton::Play => state.set(GameState::Game).unwrap(),
                MenuButton::Quit => exit.send(AppExit),
                MenuButton::Reset => {
                    total_score.reset();
                    state.restart().unwrap();
                }
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
