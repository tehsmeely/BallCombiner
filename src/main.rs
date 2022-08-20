use bevy::prelude::*;
use bevy::render::texture::ImageSettings;
use bevy::sprite::{Anchor, Material2d, MaterialMesh2dBundle};
use bevy_rapier2d::prelude::*;

fn main() {
    let rapier: RapierPhysicsPlugin<NoUserData> = RapierPhysicsPlugin::pixels_per_meter(32f32);
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(rapier)
        .add_plugin(RapierDebugRenderPlugin::default())
        .insert_resource(ImageSettings::default_nearest())
        .add_startup_system(setup)
        .add_startup_system(spawn_cups)
        .add_startup_system(spawn_balance)
        .add_system(spawn_ball_system)
        .add_system(rotate_cup_system)
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

#[derive(Component)]
struct Cup(usize);

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
                .insert(Transform::from_xyz(0.0, 10.0, 0.0));
        });
}
