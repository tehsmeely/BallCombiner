use crate::game::audio::AudioTriggerEvent;
use crate::game::balance::BalanceCounter;
use crate::game::overlay::Overlay;

use crate::game::ball::{BallKind, SpawnBallEvent};
use crate::TotalScore;
use bevy::prelude::*;
use bevy::time::Stopwatch;
use rand::distributions::Standard;
use rand::prelude::Distribution;
use rand::Rng;
use std::fmt::{Display, Formatter};
use std::time::Duration;

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
    AB {
        a_pct: usize,
        a_kind: BallKind,
        b_kind: BallKind,
    },
}
impl Mix {
    pub fn to_string_hum(&self) -> String {
        match self {
            Self::FiftyFifty => String::from("50/50"),
            Self::AB {
                a_pct,
                a_kind,
                b_kind,
            } => format!("{}pct {} to {}pct {}", a_pct, a_kind, 100 - a_pct, b_kind),
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
        "-".into(),
        format!("Make a mix of {:.2}", criteria.target_mix),
        format!("Minimum weight of: {:.2}", criteria.min_weight),
        format!(
            "You get {:.0}s once you hit this weight to get it right",
            criteria.countdown_time_secs
        ),
        "(Enter to dismiss)".into(),
    ];

    let text_style = TextStyle {
        font: asset_server.load("Quicksand-Regular.ttf"),
        font_size: 20.0,
        color: Default::default(),
    };
    super::overlay::spawn(&mut commands, text_style, display_texts, level_stopwatch);
}

fn final_calculation_display(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    level_stopwatch: ResMut<LevelStopwatch>,
    a_result: String,
    b_result: String,
    score: f32,
) {
    let display_texts = vec![
        "Result".into(),
        "".into(),
        a_result,
        b_result,
        format!("Score: {:.2}", score),
    ];

    let text_style = TextStyle {
        font: asset_server.load("Quicksand-Regular.ttf"),
        font_size: 20.0,
        color: Default::default(),
    };
    super::overlay::spawn(&mut commands, text_style, display_texts, level_stopwatch);
}

impl LevelCriteria {
    pub fn to_strings(&self) -> Vec<String> {
        vec![
            format!("Min Weight: {:.2}", self.min_weight),
            format!("Target Mix: {}", self.target_mix),
        ]
    }

    pub fn watch_system(
        criteria: Res<Self>,
        mut level_stopwatch: ResMut<LevelStopwatch>,
        mut countdown: ResMut<Countdown>,
        balance_counter: Res<BalanceCounter>,
        mut audio_trigger_event_writer: EventWriter<AudioTriggerEvent>,
        mut commands: Commands,
        asset_server: Res<AssetServer>,
        mut total_score: ResMut<TotalScore>,
    ) {
        let result: CriteriaResult = match *countdown {
            Countdown::Inactive => {
                if balance_counter.total_weight() > criteria.min_weight {
                    CriteriaResult::StartCountdown
                } else {
                    CriteriaResult::Nothing
                }
            }
            Countdown::Active {
                end,
                end_calculated,
            } => {
                if level_stopwatch.stopwatch.elapsed_secs() > end && !end_calculated {
                    CriteriaResult::CalculateResult
                } else {
                    CriteriaResult::Nothing
                }
            }
        };

        match result {
            CriteriaResult::StartCountdown => {
                *countdown = Countdown::Active {
                    end: level_stopwatch.stopwatch.elapsed_secs() + criteria.countdown_time_secs,
                    end_calculated: false,
                };
                audio_trigger_event_writer.send(AudioTriggerEvent::CountdownStarted);
            }
            CriteriaResult::CalculateResult => {
                level_stopwatch.stop();
                let (a_result, b_result, score) =
                    balance_counter.ratios_and_score(&criteria.target_mix);
                final_calculation_display(
                    commands,
                    asset_server,
                    level_stopwatch,
                    a_result,
                    b_result,
                    score,
                );
                total_score.0 += score;
                countdown.set_end_calculated();
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
                    Mix::AB {
                        a_pct,
                        a_kind: BallKind::Blue,
                        b_kind: BallKind::Red,
                    }
                }
                ProtoMix::FixedThird => {
                    let a_pct = if left { 33usize } else { 66usize };
                    Mix::AB {
                        a_pct,
                        a_kind: BallKind::Blue,
                        b_kind: BallKind::Red,
                    }
                }
                ProtoMix::RandomOther => {
                    let a_pct = rng.gen_range(10..90);
                    Mix::AB {
                        a_pct,
                        a_kind: BallKind::Blue,
                        b_kind: BallKind::Red,
                    }
                }
            }
        };
        let countdown_time_secs = rng.gen_range(5f32..15f32).round();

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

pub struct LevelStopwatch {
    pub stopwatch: Stopwatch,
    pub timer: Timer,
    stopped: bool,
}

impl LevelStopwatch {
    pub fn new() -> Self {
        let pre_finished_timer = {
            let mut timer = Timer::new(Duration::from_secs(2), true);
            timer.tick(Duration::from_millis(1500));
            timer
        };
        Self {
            stopwatch: Stopwatch::new(),
            timer: pre_finished_timer,
            stopped: false,
        }
    }
    pub fn update_system(
        mut stopwatch: ResMut<Self>,
        time: Res<Time>,
        mut ball_spawn_event_writer: EventWriter<SpawnBallEvent>,
    ) {
        stopwatch.stopwatch.tick(time.delta());
        stopwatch.timer.tick(time.delta());
        if stopwatch.timer.just_finished() {
            ball_spawn_event_writer.send(SpawnBallEvent);
        }
    }
    pub fn pause(&mut self) {
        self.stopwatch.pause();
        self.timer.pause();
    }

    pub fn paused(&mut self) -> bool {
        self.stopwatch.paused()
    }
    pub fn resume(&mut self) {
        if !self.stopped {
            self.stopwatch.unpause();
            self.timer.unpause();
        }
    }
    pub fn reset(&mut self) {
        self.stopwatch.reset();
        self.timer.reset();
        self.stopped = false;
    }

    pub fn stop(&mut self) {
        self.stopped = true;
        self.pause();
    }
}

pub enum Countdown {
    Inactive,
    Active { end: f32, end_calculated: bool },
}

impl Countdown {
    pub fn reset(&mut self) {
        *self = Self::Inactive
    }

    fn set_end_calculated(&mut self) {
        let mut end_calc = match self {
            Self::Active { end_calculated, .. } => Some(end_calculated),
            Self::Inactive => None,
        };
        if let Some(mut end_calculated) = end_calc {
            *end_calculated = true
        }
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
            end: stopwatch.stopwatch.elapsed_secs() + level_criteria.countdown_time_secs,
            end_calculated: false,
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
