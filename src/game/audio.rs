use bevy::log::info;
use bevy::prelude::*;
use bevy_kira_audio::{Audio, AudioControl, AudioSource};
use enum_map::{enum_map, Enum, EnumMap};

pub enum AudioTriggerEvent {
    CountdownTick,
    CountdownStarted,
}

#[derive(Enum)]
pub enum AudioAsset {
    ShortBeep,
    Acquired,
}

#[derive(Deref)]
pub struct AudioAssetStore(EnumMap<AudioAsset, Handle<AudioSource>>);

impl AudioAssetStore {
    fn new(asset_server: &AssetServer) -> Self {
        Self(enum_map! {
            AudioAsset::ShortBeep => asset_server.load(AudioAsset::ShortBeep.to_filename()),
            AudioAsset::Acquired => asset_server.load(AudioAsset::Acquired.to_filename())
        })
    }

    fn get(&self, asset: AudioAsset) -> Handle<AudioSource> {
        self[asset].clone()
    }
}

impl AudioAsset {
    fn to_filename(&self) -> &str {
        match self {
            Self::ShortBeep => "audio/154953__keykrusher__microwave-beep.wav",
            Self::Acquired => "audio/608431__plasterbrain__shiny-coin-pickup.flac",
        }
    }
}

pub fn setup_audio(mut commands: Commands, asset_server: Res<AssetServer>, audio: Res<Audio>) {
    commands.insert_resource(AudioAssetStore::new(&asset_server));
}

pub fn triggered_audio_system(
    mut event_reader: EventReader<AudioTriggerEvent>,
    audio_asset_store: Res<AudioAssetStore>,
    audio: Res<Audio>,
) {
    for event in event_reader.iter() {
        match event {
            AudioTriggerEvent::CountdownTick => {
                let asset = audio_asset_store.get(AudioAsset::ShortBeep);
                info!("Playing Short Beep");
                audio.play(asset);
            }
            AudioTriggerEvent::CountdownStarted => {
                audio.play(audio_asset_store.get(AudioAsset::Acquired));
            }
        }
    }
}
