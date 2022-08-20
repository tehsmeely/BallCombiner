use std::collections::HashMap;

use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};
use bevy::prelude::*;
use bevy::render::texture::ImageSettings;
use bevy::sprite::{Anchor, Material2d, MaterialMesh2dBundle};
use bevy_rapier2d::prelude::*;

mod game;
mod ui_core;

#[derive(Hash, Clone, PartialOrd, PartialEq, Debug, Eq)]
pub enum GameState {
    Menu,
    Game,
}

fn main() {
    let rapier: RapierPhysicsPlugin<NoUserData> = RapierPhysicsPlugin::pixels_per_meter(32f32);
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(rapier)
        .add_plugin(RapierDebugRenderPlugin::default())
        .add_plugin(LogDiagnosticsPlugin::default())
        .add_plugin(FrameTimeDiagnosticsPlugin::default())
        .add_plugin(game::GamePlugin)
        .add_state(GameState::Game)
        .insert_resource(ImageSettings::default_nearest())
        .add_startup_system(setup)
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn_bundle(Camera2dBundle {
        projection: OrthographicProjection {
            scale: 0.3,
            ..default()
        },
        ..default()
    });
}
