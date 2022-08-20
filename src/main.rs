use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};
use bevy::prelude::*;
use bevy::render::texture::ImageSettings;
use bevy::sprite::{Anchor, Material2d, MaterialMesh2dBundle};
use bevy_rapier2d::prelude::*;
use std::collections::HashMap;

fn main() {
    let rapier: RapierPhysicsPlugin<NoUserData> = RapierPhysicsPlugin::pixels_per_meter(32f32);
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(rapier)
        .add_plugin(RapierDebugRenderPlugin::default())
        .add_plugin(LogDiagnosticsPlugin::default())
        .add_plugin(FrameTimeDiagnosticsPlugin::default())
        .insert_resource(ImageSettings::default_nearest())
        .add_startup_system(setup)
        .add_startup_system(spawn_cups)
        .add_startup_system(spawn_balance)
        .insert_resource(BalanceCounter::new())
        .add_system(spawn_ball_system)
        .add_system(rotate_cup_system)
        .add_system(ball_sensor_system)
        .run();
}

#[derive(Debug, Clone)]
pub struct BalanceCounter {
    ball_count: HashMap<usize, usize>,
}

impl BalanceCounter {
    fn new() -> Self {
        BalanceCounter {
            ball_count: HashMap::new(),
        }
    }

    fn incr(&mut self, ball_id: usize) {
        *self.ball_count.entry(ball_id).or_insert(0) += 1;
    }

    fn decr(&mut self, ball_id: usize) {
        match self.ball_count.get_mut(&ball_id) {
            Some(0) => warn!("Tried to decr 0 in map (key: {}), not doing.", ball_id),
            Some(val) => *val -= 1,
            None => warn!("Tried to decr nonexistent count (key: {})", ball_id),
        }
    }

    fn total_count(&self) -> usize {
        let mut i = 0;
        for v in self.ball_count.values() {
            i += v;
        }
        i
    }
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

fn spawn_ball_system(
    mut commands: Commands,
    input: Res<Input<KeyCode>>,
    asset_server: Res<AssetServer>,
    cup_query: Query<(&Transform, &Cup)>,
) {
    if input.just_pressed(KeyCode::Return) {
        for (transform, cup) in cup_query.iter() {
            spawn_ball(transform.translation.x, cup.0, &mut commands, &asset_server);
        }
    }
}

fn angvel_of_input(input: &Input<KeyCode>, ac: KeyCode, c: KeyCode) -> f32 {
    if input.pressed(ac) {
        1.0
    } else if input.pressed(c) {
        -1.0
    } else {
        0.0
    }
}

fn rotate_cup_system(mut cup_query: Query<(&mut Velocity, &Cup)>, input: Res<Input<KeyCode>>) {
    let ang_vel_0 = angvel_of_input(&input, KeyCode::A, KeyCode::D);
    let ang_vel_1 = angvel_of_input(&input, KeyCode::H, KeyCode::K);
    for (mut velocity, cup) in cup_query.iter_mut() {
        if cup.0 == 0 {
            velocity.angvel = ang_vel_0;
        } else if cup.0 == 1 {
            velocity.angvel = ang_vel_1;
        }
    }
}

fn other_entity_if_match(match_entity: &Entity, e1: Entity, e2: Entity) -> Option<Entity> {
    if e1 == *match_entity {
        Some(e2)
    } else if e2 == *match_entity {
        Some(e1)
    } else {
        None
    }
}

fn ball_sensor_system(
    mut active_events: EventReader<CollisionEvent>,
    balance_sensor_query: Query<Entity, With<BalanceSensor>>,
    ball_query: Query<&Ball>,
    mut balance_text_query: Query<&mut Text, With<BalanceText>>,
    mut balance_counter: ResMut<BalanceCounter>,
) {
    if !active_events.is_empty() {
        let balance_sensor_entity = balance_sensor_query.single();
        let mut counter_changed = false;
        for event in active_events.iter() {
            match event {
                CollisionEvent::Started(e1, e2, _flags) => {
                    if let Some(other_entity) =
                        other_entity_if_match(&balance_sensor_entity, *e1, *e2)
                    {
                        if let Ok(ball) = ball_query.get(other_entity) {
                            balance_counter.incr(ball.0);
                            counter_changed = true;
                        }
                    }
                }
                CollisionEvent::Stopped(e1, e2, _flags) => {
                    if let Some(other_entity) =
                        other_entity_if_match(&balance_sensor_entity, *e1, *e2)
                    {
                        if let Ok(ball) = ball_query.get(other_entity) {
                            balance_counter.decr(ball.0);
                            counter_changed = true;
                        }
                    }
                }
            }
        }
        if counter_changed {
            println!("Counter changed: {:?}", balance_counter);
            let total_mass = 0.8 * (balance_counter.total_count() as f32);
            if let Ok(mut text) = balance_text_query.get_single_mut() {
                text.sections[0].value = format!("{:.2}", total_mass);
            }
        }
    }
}

#[derive(Component)]
struct Cup(usize);
#[derive(Component)]
struct Ball(usize);
#[derive(Component)]
struct BalanceSensor;
#[derive(Component)]
struct BalanceText;

fn spawn_cups(mut commands: Commands, asset_server: Res<AssetServer>) {
    spawn_cup(-50.0, 0, &mut commands, &asset_server);
    spawn_cup(50.0, 1, &mut commands, &asset_server);
}
fn spawn_cup(x_offset: f32, idx: usize, commands: &mut Commands, asset_server: &AssetServer) {
    let sprite_tex = asset_server.load("cup.png");
    let shape = {
        let thickness = 4.0;
        let height = 30.0;
        let width = 50.0;
        let side = Collider::cuboid(thickness / 2.0, height / 2.0);
        let bottom = Collider::cuboid(width / 2.0, thickness / 2.0);
        let offset = width / 2.0 - thickness / 2.0;
        vec![
            (Vec2::new(-offset, height / 2.0), 0.0, side.clone()),
            (Vec2::new(0.0, thickness / 2.0), 0.0, bottom.clone()),
            (Vec2::new(offset, height / 2.0), 0.0, side),
        ]
    };
    let transform = Transform::from_xyz(x_offset, 0.0, 0.0);
    let color = if idx == 1 {
        Color::RED
    } else {
        Color::default()
    };
    commands
        .spawn()
        .insert(RigidBody::Dynamic)
        .insert(LockedAxes::TRANSLATION_LOCKED)
        .insert(Collider::compound(shape))
        .insert(Sleeping::disabled())
        .insert(Velocity::default())
        .insert(Ccd::enabled())
        .insert_bundle(SpriteBundle {
            //transform: Transform::from_xyz(0.0, 0.0, 0.0),
            sprite: Sprite {
                color,
                anchor: Anchor::BottomCenter,
                ..default()
            },
            texture: sprite_tex,
            transform,
            ..default()
        })
        .insert(Cup(idx));
}

fn spawn_ball(x_offset: f32, idx: usize, commands: &mut Commands, asset_server: &AssetServer) {
    let radius = 2.8;
    let sprite_tex = asset_server.load("ball.png");
    let transform = Transform::from_xyz(x_offset, 100.0, 0.0);
    let color = if idx == 1 {
        Color::RED
    } else {
        Color::default()
    };
    commands
        .spawn()
        .insert(RigidBody::Dynamic)
        .insert(Collider::ball(radius))
        .insert(Sleeping::disabled())
        .insert(Ccd::enabled())
        .insert(GravityScale(1.0))
        .insert(Ball(idx))
        .insert_bundle(SpriteBundle {
            //transform: Transform::from_xyz(0.0, 0.0, 0.0),
            sprite: Sprite {
                color,
                anchor: Anchor::Center,
                custom_size: Some(Vec2::new(radius * 2.0, radius * 2.0)),
                ..default()
            },
            texture: sprite_tex,
            transform,
            ..default()
        });
}

fn spawn_balance(mut commands: Commands, asset_server: Res<AssetServer>) {
    let font = asset_server.load("Quicksand-Regular.ttf");
    let (collider_shape, sensor_shape) = {
        let y_offset = -6.0;
        let thickness = 4.0;
        let height = 24.0;
        let width = 50.0;
        let side = Collider::cuboid(thickness / 2.0, height / 2.0);
        let bottom = Collider::cuboid(width / 2.0, thickness / 2.0);
        let offset = width / 2.0 - thickness / 2.0;
        let collider_shape = vec![
            (
                Vec2::new(-offset, (height / 2.0) + y_offset),
                0.0,
                side.clone(),
            ),
            (
                Vec2::new(0.0, (thickness / 2.0) + y_offset),
                0.0,
                bottom.clone(),
            ),
            (Vec2::new(offset, (height / 2.0) + y_offset), 0.0, side),
        ];
        let sensor_shape = Collider::cuboid(width / 2.0, height / 2.0);
        (collider_shape, sensor_shape)
    };

    let transform = Transform::from_xyz(0.0, -70.0, 0.0);
    let text_transform = Transform::from_xyz(-8.0, -96.0, 1.0);
    let texture = asset_server.load("balance.png");
    commands
        .spawn()
        .insert_bundle(SpriteBundle {
            texture,
            transform,
            ..default()
        })
        .insert(RigidBody::Fixed)
        .insert(Collider::compound(collider_shape))
        .with_children(|parent| {
            parent
                .spawn()
                .insert(sensor_shape)
                .insert(Sensor)
                .insert(ActiveEvents::COLLISION_EVENTS)
                .insert(BalanceSensor)
                .insert(Transform::from_xyz(0.0, 10.0, 0.0));
        });
    commands
        .spawn()
        .insert_bundle(Text2dBundle {
            text: Text::from_section(
                "0.00",
                TextStyle {
                    font,
                    font_size: 10.0,
                    color: Default::default(),
                },
            )
            .with_alignment(TextAlignment {
                vertical: VerticalAlign::Bottom,
                horizontal: HorizontalAlign::Center,
            }),
            transform: text_transform,
            text_2d_size: Default::default(),
            text_2d_bounds: Default::default(),
            ..default()
        })
        .insert(BalanceText);
}
