use crate::game::audio::AudioTriggerEvent;
use crate::game::balance::BalanceCounter;
use crate::game::overlay::Overlay;

use bevy::prelude::*;
use bevy::time::Stopwatch;
use rand::distributions::Standard;
use rand::prelude::Distribution;
use rand::Rng;
use std::fmt::{Display, Formatter};

enum ProtoMix {
    FiftyFifty,
    FixedQuarter,
    FixedThird,
    RandomOther,
}

impl Distribution<ProtoMix> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> ProtoMix {
        match rng.gen_range(0..=6) {
            0 | 1 => ProtoMix::FiftyFifty,
            2 | 3 => ProtoMix::FixedQuarter,
            4 | 5 => ProtoMix::FixedThird,
            6 => ProtoMix::RandomOther,
            _ => panic!("BUG: Generated outside of estabished range"),
        }
    }
}

pub enum Mix {
    FiftyFifty,
    AB { a_pct: usize },
}
impl Mix {
    pub fn to_string_hum(&self) -> String {
        match self {
            Self::FiftyFifty => String::from("50/50"),
            Self::AB { a_pct } => format!("{}/{}", a_pct, 100 - a_pct),
        }
    }
}
impl Display for Mix {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_string_hum())
    }
}

pub struct LevelCriteria {
    pub min_weight: f32,
    pub target_mix: Mix,
    pub countdown_time_secs: f32,
}

pub fn initial_goal_display(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    //overlay_query: Query<Entity, With<Overlay>>,
    level_stopwatch: ResMut<LevelStopwatch>,
    criteria: Res<LevelCriteria>,
) {
    let display_texts = vec![
        "Goals".into(),
        format!("Make a mix of {}", criteria.target_mix),
        format!("Minimum weight of: {}", criteria.min_weight),
        format!(
            "You get {}s once you hit this weight to get it right",
            criteria.countdown_time_secs
        ),
    ];

    let text_style = TextStyle {
        font: asset_server.load("Quicksand-Regular.ttf"),
        font_size: 20.0,
        color: Default::default(),
    };
    super::overlay::spawn(&mut commands, text_style, display_texts, level_stopwatch);
}

pub fn _final_calculation_display() {}

impl LevelCriteria {
    pub fn to_strings(&self) -> Vec<String> {
        vec![
            format!("Min Weight: {:.2}", self.min_weight),
            format!("Target Mix: {}", self.target_mix),
        ]
    }

    pub fn watch_system(
        criteria: Res<Self>,
        level_stopwatch: Res<LevelStopwatch>,
        mut countdown: ResMut<Countdown>,
        balance_counter: Res<BalanceCounter>,
        mut audio_trigger_event_writer: EventWriter<AudioTriggerEvent>,
        // TODO: don't use a local, store the one-shot final somewhre else
        mut finished: Local<bool>,
    ) {
        let result: CriteriaResult = match *countdown {
            Countdown::Inactive => {
                if balance_counter.total_weight() > criteria.min_weight {
                    CriteriaResult::StartCountdown
                } else {
                    CriteriaResult::Nothing
                }
            }
            Countdown::Active { end } => {
                if level_stopwatch.0.elapsed_secs() > end && !*finished {
                    CriteriaResult::CalculateResult
                } else {
                    CriteriaResult::Nothing
                }
            }
        };

        match result {
            CriteriaResult::StartCountdown => {
                *countdown = Countdown::Active {
                    end: level_stopwatch.0.elapsed_secs() + criteria.countdown_time_secs,
                };
                audio_trigger_event_writer.send(AudioTriggerEvent::CountdownStarted);
            }
            CriteriaResult::CalculateResult => {
                let true_ratio = balance_counter.calculate_ratio();
                println!(
                    "True Ratio: {}. Target: {}",
                    true_ratio, criteria.target_mix
                );
                *finished = true;
            }
            CriteriaResult::Nothing => (),
        };
    }

    pub fn new_random() -> Self {
        let mut rng = rand::thread_rng();

        let weight_bounds = 1.2f32..6.0f32;
        let min_weight = rng.gen_range(weight_bounds);
        let target_mix = {
            let proto_mix: ProtoMix = rng.gen();
            let left = rng.gen_bool(0.5);
            match proto_mix {
                ProtoMix::FiftyFifty => Mix::FiftyFifty,
                ProtoMix::FixedQuarter => {
                    let a_pct = if left { 25usize } else { 75usize };
                    Mix::AB { a_pct }
                }
                ProtoMix::FixedThird => {
                    let a_pct = if left { 33usize } else { 66usize };
                    Mix::AB { a_pct }
                }
                ProtoMix::RandomOther => {
                    let a_pct = rng.gen_range(10..90);
                    Mix::AB { a_pct }
                }
            }
        };
        let countdown_time_secs = rng.gen_range(5f32..15f32);

        Self {
            min_weight,
            target_mix,
            countdown_time_secs,
        }
    }
}

enum CriteriaResult {
    StartCountdown,
    CalculateResult,
    Nothing,
}

pub struct LevelStopwatch(pub Stopwatch);

impl LevelStopwatch {
    pub fn new() -> Self {
        Self(Stopwatch::new())
    }
    pub fn update_system(mut stopwatch: ResMut<Self>, time: Res<Time>) {
        stopwatch.0.tick(time.delta());
    }
    pub fn reset(&mut self) {
        self.0.reset();
    }
}

pub enum Countdown {
    Inactive,
    Active { end: f32 },
}

impl Countdown {
    pub fn reset(&mut self) {
        *self = Self::Inactive
    }
}

pub fn debug_countdown_trigger_system(
    mut countdown: ResMut<Countdown>,
    stopwatch: Res<LevelStopwatch>,
    level_criteria: Res<LevelCriteria>,
    input: Res<Input<KeyCode>>,
) {
    if input.just_pressed(KeyCode::P) {
        *countdown = Countdown::Active {
            end: stopwatch.0.elapsed_secs() + level_criteria.countdown_time_secs,
        }
    }
}
pub fn debug_overlay_system(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    overlay_query: Query<Entity, With<Overlay>>,
    input: Res<Input<KeyCode>>,
    level_stopwatch: ResMut<LevelStopwatch>,
) {
    if input.just_pressed(KeyCode::I) {
        if overlay_query.is_empty() {
            let text_style = TextStyle {
                font: asset_server.load("Quicksand-Regular.ttf"),
                font_size: 20.0,
                color: Default::default(),
            };
            super::overlay::spawn(
                &mut commands,
                text_style,
                vec!["Overlay", "Bro", "You have been warned"],
                level_stopwatch,
            );
        } else {
            super::overlay::despawn(&mut commands, &overlay_query);
        }
    }
}
