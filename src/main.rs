use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};
use bevy::prelude::*;
use bevy::render::texture::ImageSettings;

use bevy_rapier2d::prelude::*;

mod game;
mod menu;
mod ui_core;

#[derive(Hash, Clone, PartialOrd, PartialEq, Debug, Eq)]
pub enum GameState {
    Menu,
    Game,
}

#[cfg(target_arch = "wasm32")]
const WINDOW_WIDTH: f32 = 640f32;
#[cfg(not(target_arch = "wasm32"))]
const WINDOW_WIDTH: f32 = 1280f32;

#[cfg(target_arch = "wasm32")]
const WINDOW_HEIGHT: f32 = 360f32;
#[cfg(not(target_arch = "wasm32"))]
const WINDOW_HEIGHT: f32 = 720f32;

fn main() {
    let rapier: RapierPhysicsPlugin<NoUserData> = RapierPhysicsPlugin::pixels_per_meter(32f32);
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(rapier)
        .add_plugin(bevy_kira_audio::AudioPlugin)
        .add_plugin(game::GamePlugin)
        .add_plugin(menu::MenuPlugin)
        .add_plugin(FeatureEnabledPlugin)
        .add_state(GameState::Menu)
        .insert_resource(ImageSettings::default_nearest())
        .insert_resource(TotalScore(0f32))
        .add_system(ui_core::buttons::button_system)
        .add_startup_system(setup)
        .add_startup_system(setup_window)
        .run();
}

fn setup(mut commands: Commands, _asset_server: Res<AssetServer>) {
    let camera_scale = if cfg!(target_arch = "wasm32") {
        0.8
    } else {
        0.4
    };
    commands.spawn_bundle(Camera2dBundle {
        projection: OrthographicProjection {
            scale: camera_scale,
            ..default()
        },
        ..default()
    });
}

pub struct TotalScore(f32);

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

fn setup_window(mut windows: ResMut<Windows>) {
    for window in windows.iter_mut() {
        println!("{:?}", window);
        window.set_resolution(WINDOW_WIDTH, WINDOW_HEIGHT);
        println!("{:?}", window);
    }
}
