mod balance;
mod ball;
mod cup;
mod ui;

use crate::GameState;
use balance::BalanceCounter;
use bevy::prelude::*;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(BalanceCounter::new())
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
                    .with_system(balance::ball_sensor_system),
            )
            .add_system_set(SystemSet::on_exit(GameState::Game));
    }
}
