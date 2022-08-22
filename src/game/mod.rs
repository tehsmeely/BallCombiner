mod balance;
mod ball;
mod cup;
mod goals;
mod ui;

use crate::game::goals::{Countdown, LevelCriteria, LevelStopwatch, Mix};
use crate::GameState;
use balance::BalanceCounter;
use bevy::prelude::*;

pub struct GamePlugin;

#[derive(Component)]
struct GameOnlyMarker;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(BalanceCounter::new())
            .insert_resource(LevelCriteria {
                min_weight: 2.0,
                target_mix: Mix::FiftyFifty,
                countdown_time_secs: 10.0,
            })
            .insert_resource(goals::LevelStopwatch::new())
            .insert_resource(Countdown::Inactive)
            .add_system_set(
                SystemSet::on_enter(GameState::Game)
                    .with_system(cup::spawn_cups)
                    .with_system(balance::spawn_balance)
                    .with_system(ui::setup_ui)
                    .with_system(reset_game_resources),
            )
            .add_system_set(
                SystemSet::on_update(GameState::Game)
                    .with_system(ball::spawn_ball_system)
                    .with_system(cup::rotate_cup_system)
                    .with_system(balance::ball_sensor_system)
                    .with_system(ui::TimerDisplay::update_system)
                    .with_system(ui::button_click_system)
                    .with_system(goals::LevelStopwatch::update_system)
                    .with_system(goals::LevelCriteria::watch_system)
                    .with_system(goals::debug_countdown_trigger_system),
            )
            .add_system_set(SystemSet::on_exit(GameState::Game).with_system(cleanup));
    }
}

fn cleanup(mut commands: Commands, entities: Query<Entity, With<GameOnlyMarker>>) {
    for entity in entities.iter() {
        commands.entity(entity).despawn_recursive();
    }
}

fn reset_game_resources(mut stopwatch: ResMut<LevelStopwatch>, mut countdown: ResMut<Countdown>) {
    stopwatch.reset();
    countdown.reset();
}
