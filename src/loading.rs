use crate::ui_core::nodes;
use crate::GameState;
use bevy::asset::LoadState;
use bevy::prelude::*;
use bevy_kira_audio::AudioSource;
use nodes::Property;

pub struct LoadingPlugin;

#[derive(Component)]
pub struct LoadingOnlyMarker;

impl Plugin for LoadingPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(
            SystemSet::on_enter(GameState::Loading)
                .with_system(loading_display_setup)
                .with_system(start_loading_things),
        )
        .add_system_set(SystemSet::on_update(GameState::Loading).with_system(loading_watcher))
        .add_system_set(SystemSet::on_exit(GameState::Loading).with_system(teardown));
    }
}

fn loading_display_setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let bevy_icon = asset_server.load("bevy.png");
    let font = asset_server.load("Quicksand-Bold.ttf");

    commands
        .spawn()
        .insert_bundle(nodes::new(nodes::defaults::full(
            FlexDirection::Column,
            Some(vec![Property::Justify(JustifyContent::Center)]),
        )))
        .with_children(|parent| {
            parent
                .spawn_bundle(nodes::new(vec![
                    Property::Direction(FlexDirection::ColumnReverse),
                    Property::MarginAll(Val::Auto),
                    Property::Width(Val::Auto),
                    Property::Height(Val::Auto),
                    Property::FlexGrow(0.0),
                    Property::FlexBasis(Val::Percent(0.0)),
                    Property::Justify(JustifyContent::Center),
                ]))
                .with_children(|parent| {
                    parent.spawn_bundle(nodes::new(vec![
                        Property::Width(Val::Px(60.0)),
                        Property::Height(Val::Px(60.0)),
                        Property::MarginAll(Val::Auto),
                        Property::Image(bevy_icon),
                        Property::Colour(Color::WHITE),
                    ]));

                    parent
                        .spawn_bundle(nodes::new(vec![
                            Property::Width(Val::Px(110.0)),
                            Property::Height(Val::Px(200.0)),
                            Property::MarginAll(Val::Auto),
                        ]))
                        .with_children(|parent| {
                            parent.spawn().insert_bundle(TextBundle {
                                style: Style {
                                    margin: UiRect::new(Val::Auto, Val::Auto, Val::Auto, Val::Auto),
                                    ..default()
                                },
                                text: Text::from_section(
                                    "Loading...",
                                    TextStyle {
                                        font,
                                        font_size: 30.0,
                                        ..default()
                                    },
                                )
                                .with_alignment(TextAlignment::CENTER),
                                ..default()
                            });
                        });
                });
        })
        .insert(LoadingOnlyMarker);
}

fn start_loading_things(mut commands: Commands, asset_server: Res<AssetServer>) {
    let audio = vec![asset_server.load("audio/music/Getting it Done.mp3")];
    commands.insert_resource(LoadedHandles { audio });
}

fn teardown(mut commands: Commands, q: Query<Entity, With<LoadingOnlyMarker>>) {
    for entity in q.iter() {
        commands.entity(entity).despawn_recursive();
    }
}

struct LoadedHandles {
    audio: Vec<Handle<AudioSource>>,
}

fn loading_watcher(
    loaded_handles: Res<LoadedHandles>,
    asset_server: Res<AssetServer>,
    mut state: ResMut<State<GameState>>,
) {
    let mut count = LoadStateCount::default();
    for handle in loaded_handles.audio.iter() {
        let load_state = asset_server.get_load_state(handle);
        count.incr(&load_state);
    }

    if count.all_finished() {
        info!("Finished Loading: {:?}", count);
        state.set(GameState::Menu).unwrap();
    }
}

#[derive(Default, Debug)]
struct LoadStateCount {
    not_loaded: usize,
    loading: usize,
    loaded: usize,
    failed: usize,
    unloaded: usize,
}

impl LoadStateCount {
    fn incr(&mut self, load_state: &LoadState) {
        match load_state {
            LoadState::NotLoaded => self.not_loaded += 1,
            LoadState::Loading => self.loading += 1,
            LoadState::Loaded => self.loaded += 1,
            LoadState::Failed => self.loaded += 1,
            LoadState::Unloaded => self.unloaded += 1,
        }
    }

    fn all_finished(&self) -> bool {
        self.not_loaded == 0 && self.loading == 0
    }
}
