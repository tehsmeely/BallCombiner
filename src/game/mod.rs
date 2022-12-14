mod audio;
mod balance;
mod ball;
mod components;
mod cup;
mod goals;
pub mod not_a_cup;
mod overlay;
mod ui;

pub use ball::BallKind;

use crate::game::audio::AudioTriggerEvent;
use crate::game::components::GeneralComponentsPlugin;
use crate::game::goals::{Countdown, LevelCriteria, LevelStopwatch};
use crate::GameState;
use balance::BalanceCounter;
use bevy::prelude::*;

use crate::game::ball::SpawnBallEvent;
use bevy_rapier2d::geometry::Collider;
use bevy_rapier2d::prelude::RigidBody;

pub struct GamePlugin;

#[derive(Component)]
struct GameOnlyMarker;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(BalanceCounter::new())
            .insert_resource(LevelCriteria::new_random())
            .insert_resource(goals::LevelStopwatch::new())
            .insert_resource(Countdown::Inactive)
            .add_event::<AudioTriggerEvent>()
            .add_event::<SpawnBallEvent>()
            .add_plugin(GeneralComponentsPlugin)
            .add_system_set(
                SystemSet::on_enter(GameState::Game)
                    .with_system(cup::spawn_cups)
                    .with_system(balance::spawn_balance)
                    .with_system(ui::setup_ui)
                    .with_system(reset_game_resources)
                    .with_system(audio::setup_audio)
                    .with_system(spawn_background)
                    .with_system(goals::initial_goal_display),
            )
            .add_system_set(
                SystemSet::on_update(GameState::Game)
                    .with_system(ball::spawn_ball_system)
                    .with_system(cup::rotate_cup_system)
                    .with_system(cup::ui_helper_show_system)
                    .with_system(balance::ball_sensor_system)
                    .with_system(ui::TimerDisplay::update_system)
                    .with_system(ui::button_click_system)
                    .with_system(goals::LevelStopwatch::update_system)
                    .with_system(goals::LevelCriteria::watch_system)
                    .with_system(goals::debug_countdown_trigger_system)
                    .with_system(goals::debug_overlay_system)
                    .with_system(audio::triggered_audio_system)
                    .with_system(overlay::timer_resume_watcher)
                    .with_system(overlay::overlay_dismiss_system)
                    .with_system(not_a_cup::JarDoor::system),
            )
            .add_system_set(SystemSet::on_exit(GameState::Game).with_system(cleanup));
    }
}

fn cleanup(mut commands: Commands, entities: Query<Entity, With<GameOnlyMarker>>) {
    for entity in entities.iter() {
        commands.entity(entity).despawn_recursive();
    }
}

/*
app.insert_resource(BalanceCounter::new())
.insert_resource(LevelCriteria::new_random())
.insert_resource(goals::LevelStopwatch::new())
.insert_resource(Countdown::Inactive)
 */

fn reset_game_resources(
    mut stopwatch: ResMut<LevelStopwatch>,
    mut countdown: ResMut<Countdown>,
    mut balance_counter: ResMut<BalanceCounter>,
) {
    stopwatch.reset();
    countdown.reset();
    balance_counter.reset();
}

fn spawn_background(mut commands: Commands, asset_server: Res<AssetServer>) {
    let background_image = asset_server.load("background.png");
    let table_image = asset_server.load("table.png");

    commands
        .spawn()
        .insert_bundle(SpriteBundle {
            sprite: Sprite {
                custom_size: None,
                ..default()
            },
            transform: Transform::from_xyz(0.0, 0.0, -1.0),
            texture: background_image,
            ..default()
        })
        .insert(GameOnlyMarker)
        .insert(Background);

    let (table_transform, sub_transform) = {
        let x = 0.0;
        let y = -160.0;
        let z = 1.0;
        let sub_offset = 54.0;
        (
            Transform::from_xyz(x, y, z),
            Transform::from_xyz(0.0, sub_offset, 0.0),
        )
    };
    commands
        .spawn()
        .insert_bundle(SpriteBundle {
            transform: table_transform,
            texture: table_image,
            ..default()
        })
        .insert(RigidBody::Fixed)
        .with_children(|parent| {
            parent
                .spawn()
                .insert(sub_transform)
                .insert(Collider::cuboid(198.0 / 2.0, 9.0 / 2.0));
        })
        .insert(GameOnlyMarker)
        .insert(Background);
}

#[derive(Component)]
struct Background;
