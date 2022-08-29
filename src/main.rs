use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};
use bevy::prelude::*;
use bevy::render::texture::ImageSettings;

use crate::ui_core::buttons::{CheckboxEvent, CheckboxState};
use bevy_kira_audio::{Audio, AudioApp, AudioChannel, AudioControl, AudioTween};
use bevy_rapier2d::prelude::*;
use std::fmt::Formatter;

mod game;
mod loading;
mod menu;
mod ui_core;

#[derive(Hash, Clone, PartialOrd, PartialEq, Debug, Eq)]
pub enum GameState {
    Loading,
    Menu,
    Game,
}

#[cfg(target_arch = "wasm32")]
const WINDOW_WIDTH: f32 = 960f32;
#[cfg(not(target_arch = "wasm32"))]
const WINDOW_WIDTH: f32 = 1280f32;

#[cfg(target_arch = "wasm32")]
const WINDOW_HEIGHT: f32 = 540f32;
#[cfg(not(target_arch = "wasm32"))]
const WINDOW_HEIGHT: f32 = 720f32;

fn main() {
    let rapier: RapierPhysicsPlugin<NoUserData> = RapierPhysicsPlugin::pixels_per_meter(32f32);
    App::new()
        .insert_resource(ImageSettings::default_nearest())
        .add_plugins(DefaultPlugins)
        .add_plugin(bevy_kira_audio::AudioPlugin)
        .add_plugin(rapier)
        .add_audio_channel::<MusicChannel>()
        .add_audio_channel::<SfxChannel>()
        .add_plugin(loading::LoadingPlugin)
        .add_plugin(game::GamePlugin)
        .add_plugin(menu::MenuPlugin)
        .add_plugin(FeatureEnabledPlugin)
        .add_state(GameState::Loading)
        .add_event::<CheckboxEvent>()
        .insert_resource(TotalScore::new())
        .add_system(ui_core::buttons::button_system)
        .add_system(ui_core::buttons::checkbox_button_system)
        .add_system(audio_setting_system)
        .add_startup_system(setup)
        .add_startup_system(setup_window)
        .add_startup_system(setup_background_music)
        .run();
}

fn setup(mut commands: Commands, _asset_server: Res<AssetServer>) {
    let camera_scale = if cfg!(target_arch = "wasm32") {
        0.6
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

pub struct TotalScore {
    scores: Vec<f32>,
}

impl TotalScore {
    pub fn new() -> Self {
        Self { scores: Vec::new() }
    }

    pub fn reset(&mut self) {
        self.scores.clear()
    }

    pub fn total(&self) -> f32 {
        self.scores.iter().sum()
    }

    pub fn mix_average(&self) -> f32 {
        if self.scores.is_empty() {
            0.0
        } else {
            self.total() / (self.scores.len() as f32)
        }
    }

    pub fn add_score(&mut self, score: f32) {
        self.scores.push(score);
    }
}

impl std::fmt::Display for TotalScore {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Total: {:.2} ({} Mixes. {:.2} Avg)",
            self.total(),
            self.scores.len(),
            self.mix_average()
        )
    }
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

fn setup_window(mut windows: ResMut<Windows>) {
    for window in windows.iter_mut() {
        println!("{:?}", window);
        window.set_resolution(WINDOW_WIDTH, WINDOW_HEIGHT);
        println!("{:?}", window);
    }
}

#[derive(Component, Default, Clone)]
pub struct MusicChannel;
type MusicAudio = AudioChannel<MusicChannel>;

#[derive(Component, Default, Clone)]
pub struct SfxChannel;
type SfxAudio = AudioChannel<SfxChannel>;

fn setup_background_music(asset_server: Res<AssetServer>, audio: Res<MusicAudio>) {
    let music = asset_server.load("audio/music/Getting it Done.mp3");
    audio.play(music).looped();
}

fn audio_setting_system(
    mut event_reader: EventReader<CheckboxEvent>,
    mut music_channel: ResMut<MusicAudio>,
    mut sfx_channel: ResMut<SfxAudio>,
) {
    for event in event_reader.iter() {
        let event: &CheckboxEvent = event;
        let enable = match event.new_state {
            CheckboxState::Checked => true,
            CheckboxState::Unchecked => false,
        };
        let audio = match event.variant {
            ui_core::buttons::CheckboxVariant::Music => match enable {
                true => music_channel.set_volume(1.0),
                false => music_channel.set_volume(0.0),
            },
            ui_core::buttons::CheckboxVariant::SFX => match enable {
                true => sfx_channel.set_volume(1.0),
                false => sfx_channel.set_volume(0.0),
            },
        };
    }
}
