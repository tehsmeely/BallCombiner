use bevy::prelude::*;

use nodes::Property;

use crate::game::audio::AudioTriggerEvent;
use crate::game::goals::{Countdown, LevelCriteria, LevelStopwatch};
use crate::game::GameOnlyMarker;
use crate::ui_core::buttons::ButtonComponent;
use crate::ui_core::nodes;
use crate::{ui_core, GameState, TotalScore};

pub fn setup_ui(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    criteria: Res<LevelCriteria>,
    total_score: Res<TotalScore>,
) {
    let text_style = TextStyle {
        font: asset_server.load("Quicksand-Regular.ttf"),
        font_size: 20.0,
        color: Default::default(),
    };
    let score_text_style = TextStyle {
        font: text_style.font.clone(),
        font_size: 16.0,
        color: Default::default(),
    };

    let exit_button = asset_server.load("buttons/exit.png");
    let reset_button = asset_server.load("buttons/restart.png");

    commands
        .spawn_bundle(nodes::new(nodes::defaults::full(
            FlexDirection::ColumnReverse,
            Some(vec![
                Property::Colour(Color::rgba(0.0, 0.0, 0.0, 0.0)),
                Property::Padding(UiRect::new(Val::Auto, Val::Auto, Val::Px(5.0), Val::Auto)),
            ]),
        )))
        .insert(GameOnlyMarker)
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
                        .spawn_bundle(nodes::new(full_height_half_width()))
                        .with_children(|parent| {
                            GoalDisplay::create(
                                parent,
                                text_style.font.clone(),
                                criteria.to_strings(),
                            );
                        });
                    parent
                        .spawn_bundle(nodes::new(full_height_half_width()))
                        .with_children(|parent| {
                            TimerDisplay::create(parent, &asset_server);
                        });
                });
            parent
                .spawn_bundle(nodes::new(vec![
                    Property::Height(Val::Percent(100.0)),
                    Property::Width(Val::Percent(100.0)),
                    Property::Direction(FlexDirection::Row),
                    Property::Justify(JustifyContent::FlexStart),
                ]))
                .with_children(|parent| {
                    parent
                        .spawn_bundle(nodes::new(vec![
                            Property::Height(Val::Percent(100.0)),
                            Property::Width(Val::Auto),
                            Property::Direction(FlexDirection::Column),
                        ]))
                        .with_children(|parent| {
                            crate::ui_core::buttons::make_button_custom_image(
                                GameActionButton::Exit,
                                exit_button,
                                parent,
                                Vec2::new(110f32, 68f32),
                                None,
                                None,
                            );
                            crate::ui_core::buttons::make_button_custom_image(
                                GameActionButton::Reset,
                                reset_button,
                                parent,
                                Vec2::new(110f32, 68f32),
                                None,
                                None,
                            );

                            parent.spawn_bundle(TextBundle {
                                style: Style {
                                    max_size: Size::new(Val::Px(100.0), Val::Auto),
                                    margin: UiRect::new(
                                        Val::Px(10.0),
                                        Val::Auto,
                                        Val::Auto,
                                        Val::Px(15.0),
                                    ),
                                    ..default()
                                },
                                text: Text::from_section(
                                    format!("Total Score: {:.2}", total_score.0),
                                    score_text_style,
                                ),
                                ..default()
                            });
                        });
                });
        });
}

pub type InteractionAndButton = (&'static Interaction, &'static GameActionButton);
pub type ButtonInteractionChanged = (Changed<Interaction>, With<Button>);

pub fn button_click_system(
    interaction_query: Query<InteractionAndButton, ButtonInteractionChanged>,
    mut state: ResMut<State<GameState>>,
    mut commands: Commands,
) {
    for (interaction, game_action_button) in &interaction_query {
        match *interaction {
            Interaction::Clicked => match *game_action_button {
                GameActionButton::Reset => {
                    commands.insert_resource(LevelCriteria::new_random());
                    state.restart().unwrap();
                }
                GameActionButton::Exit => {
                    state.set(GameState::Menu).unwrap();
                }
            },
            Interaction::Hovered | Interaction::None => (),
        }
    }
}

#[derive(Component)]
pub enum GameActionButton {
    Reset,
    Exit,
}

impl ButtonComponent for GameActionButton {
    fn to_text(&self) -> &'static str {
        match self {
            GameActionButton::Reset => "Reset",
            GameActionButton::Exit => "Exit",
        }
    }
}

#[derive(Component)]
pub struct TimerDisplay {
    last_secs: f32,
    normal_style: TextStyle,
    countdown_style: TextStyle,
}

impl TimerDisplay {
    fn create(parent: &mut ChildBuilder, asset_server: &AssetServer) {
        let text_style = TextStyle {
            font: asset_server.load("Quicksand-Regular.ttf"),
            font_size: 30.0,
            color: Default::default(),
        };
        let countdown_style = TextStyle {
            font: asset_server.load("Quicksand-Bold.ttf"),
            font_size: 30.0,
            color: Color::RED,
        };
        parent
            .spawn()
            .insert_bundle(nodes::new(vec![
                Property::Justify(JustifyContent::Center),
                Property::Height(Val::Auto),
                Property::Width(Val::Auto),
            ]))
            .with_children(|parent| {
                parent
                    .spawn_bundle(nodes::new(nodes::defaults::mini_centred()))
                    .with_children(|parent| {
                        parent
                            .spawn_bundle(TextBundle {
                                text: Text::from_section(
                                    Self::display_text(0.0, 0.0, false),
                                    text_style.clone(),
                                ),
                                ..default()
                            })
                            .insert(Self {
                                last_secs: 0f32,
                                normal_style: text_style,
                                countdown_style,
                            });
                    });
            });
    }

    fn display_text(mins: f32, secs: f32, is_countdown: bool) -> String {
        let pre = match is_countdown {
            true => "Remaining",
            false => "Elapsed",
        };
        format!("{}: {:02}:{:02}", pre, mins, secs)
    }

    pub fn update_system(
        mut self_query: Query<&mut Self>,
        mut text_query: Query<&mut Text, With<Self>>,
        countdown: Res<Countdown>,
        level_stopwatch: Res<LevelStopwatch>,
        mut audio_trigger_event_writer: EventWriter<AudioTriggerEvent>,
    ) {
        for mut timer_display in self_query.iter_mut() {
            let round_seconds = level_stopwatch.stopwatch.elapsed_secs().floor();
            if round_seconds > timer_display.last_secs {
                //update text
                let (text_style, mins, secs, is_countdown) = {
                    let (text_style, secs_total, is_countdown) = match *countdown {
                        Countdown::Inactive => (&timer_display.normal_style, round_seconds, false),
                        Countdown::Active {
                            end,
                            end_calculated: _,
                        } => {
                            let mut remaining_time = end - round_seconds;
                            if remaining_time < 0.0 {
                                remaining_time = 0.0;
                            }
                            (&timer_display.countdown_style, remaining_time, true)
                        }
                    };
                    let mins = secs_total.div_euclid(60.0);
                    let secs = secs_total.rem_euclid(60.0).floor();
                    (text_style, mins, secs, is_countdown)
                };
                for mut text in text_query.iter_mut() {
                    text.sections[0].value = Self::display_text(mins, secs, is_countdown);
                    text.sections[0].style = text_style.clone();
                }
                timer_display.last_secs = round_seconds;

                if is_countdown && secs > 0.0 && secs <= 5.0 {
                    audio_trigger_event_writer.send(AudioTriggerEvent::CountdownTick);
                }
            }
        }
    }
}

#[derive(Component, Clone)]
pub struct GoalDisplay {
    _text_style: TextStyle,
}

impl GoalDisplay {
    fn create(parent: &mut ChildBuilder, font: Handle<Font>, texts: Vec<String>) {
        let text_style = TextStyle {
            font,
            font_size: 30.0,
            color: Default::default(),
        };
        ui_core::create_centred_texts(
            parent,
            text_style.clone(),
            texts,
            Self {
                _text_style: text_style,
            },
            None,
        );
    }
}

pub fn full_height_half_width() -> Vec<Property> {
    vec![
        Property::Width(Val::Percent(50.0)),
        Property::Height(Val::Percent(100.0)),
        Property::Justify(JustifyContent::Center),
    ]
}
