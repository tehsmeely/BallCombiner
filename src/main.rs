use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};
use bevy::prelude::*;
use bevy::render::texture::ImageSettings;
use bevy::sprite::{Anchor, Material2d, MaterialMesh2dBundle};
use bevy_rapier2d::prelude::*;
use std::collections::HashMap;

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
        .add_plugin(bevy_kira_audio::AudioPlugin)
        .add_plugin(game::GamePlugin)
        .add_plugin(FeatureEnabledPlugin)
        .add_state(GameState::Game)
        .insert_resource(ImageSettings::default_nearest())
        .add_system(ui_core::buttons::button_system)
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

struct FeatureEnabledPlugin;
impl Plugin for FeatureEnabledPlugin {
    fn build(&self, app: &mut App) {
        if cfg!(feature = "debug_fps") {
            Self::diagnostics(app);
        }

        if cfg!(feature = "debug_render_colliders") {
            Self::rapier_collider_render(app);
        }
    }
}
impl FeatureEnabledPlugin {
    fn rapier_collider_render(app: &mut App) {
        app.add_plugin(RapierDebugRenderPlugin::default());
    }

    fn diagnostics(app: &mut App) {
        app.add_plugin(LogDiagnosticsPlugin::default())
            .add_plugin(FrameTimeDiagnosticsPlugin::default());
    }
}
