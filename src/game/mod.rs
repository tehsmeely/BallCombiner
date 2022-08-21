mod balance;
mod ball;
mod cup;
mod goals;
mod ui;

use crate::game::goals::{Countdown, LevelCriteria, Mix};
use crate::GameState;
use balance::BalanceCounter;
use bevy::prelude::*;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(BalanceCounter::new())
            .insert_resource(LevelCriteria {
                min_weight: 2.0,
                target_mix: Mix::FiftyFifty,
            })
            .insert_resource(goals::LevelStopwatch::new())
            .insert_resource(Countdown::Inactive)
            .add_system_set(
                SystemSet::on_enter(GameState::Game)
                    .with_system(cup::spawn_cups)
                    .with_system(balance::spawn_balance)
                    .with_system(ui::setup_ui),
            )
            .add_system_set(
                SystemSet::on_update(GameState::Game)
                    .with_system(ball::spawn_ball_system)
                    .with_system(cup::rotate_cup_system)
                    .with_system(balance::ball_sensor_system)
                    .with_system(ui::TimerDisplay::update_system)
                    .with_system(goals::LevelStopwatch::update_system)
                    .with_system(goals::LevelCriteria::watch_system)
                    .with_system(goals::debug_countdown_trigger_system),
            )
            .add_system_set(SystemSet::on_exit(GameState::Game));
    }
}
