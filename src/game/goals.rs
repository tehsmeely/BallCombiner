use crate::game::audio::AudioTriggerEvent;
use crate::game::balance::BalanceCounter;
use crate::game::ui::TimerDisplay;
use bevy::prelude::*;
use bevy::time::Stopwatch;
use std::fmt::{Display, Formatter};

pub enum Mix {
    FiftyFifty,
    AB { a_pct: usize },
}
impl Mix {
    pub fn to_string(&self) -> String {
        match self {
            Self::FiftyFifty => String::from("50/50"),
            Self::AB { a_pct } => format!("{}/{}", a_pct, 100 - a_pct),
        }
    }
}
impl Display for Mix {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_string())
    }
}

pub struct LevelCriteria {
    pub min_weight: f32,
    pub target_mix: Mix,
    pub countdown_time_secs: f32,
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
