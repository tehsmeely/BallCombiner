mod audio;
mod balance;
mod ball;
mod cup;
mod goals;
mod ui;

use crate::game::audio::AudioTriggerEvent;
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
            .add_event::<AudioTriggerEvent>()
            .add_system_set(
                SystemSet::on_enter(GameState::Game)
                    .with_system(cup::spawn_cups)
                    .with_system(balance::spawn_balance)
                    .with_system(ui::setup_ui)
                    .with_system(reset_game_resources)
                    .with_system(audio::setup_audio)
                    .with_system(spawn_background),
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
                    .with_system(goals::debug_countdown_trigger_system)
                    .with_system(audio::triggered_audio_system),
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

fn spawn_background(mut commands: Commands, asset_server: Res<AssetServer>) {
    let sprite = asset_server.load("background.png");

    commands.spawn().insert_bundle(SpriteBundle {
        sprite: Sprite {
            custom_size: Some(Vec2::new(300.0, 300.0)),
            ..default()
        },
        transform: Default::default(),
        texture: sprite,
        ..default()
    });
}
