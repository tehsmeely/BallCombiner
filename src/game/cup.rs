use crate::game::ball::BallKind;
use crate::game::not_a_cup::spawn_jar;
use crate::game::GameOnlyMarker;
use bevy::prelude::*;
use bevy::sprite::Anchor;
use bevy_rapier2d::dynamics::{Ccd, LockedAxes, RigidBody, Sleeping, Velocity};
use bevy_rapier2d::geometry::{Collider, CollisionGroups};
use rand::Rng;
use std::time::Duration;

fn angvel_of_input(input: &Input<KeyCode>, ac: KeyCode, c: KeyCode, fast: bool) -> f32 {
    let v = if input.pressed(ac) {
        1.0
    } else if input.pressed(c) {
        -1.0
    } else {
        0.0
    };
    if fast {
        v * 2.5
    } else {
        v
    }
}

pub fn rotate_cup_system(mut cup_query: Query<(&mut Velocity, &Cup)>, input: Res<Input<KeyCode>>) {
    let fast = input.pressed(KeyCode::LShift) || input.pressed(KeyCode::RShift);
    let ang_vel_0 = angvel_of_input(&input, KeyCode::A, KeyCode::D, fast);
    let ang_vel_1 = angvel_of_input(&input, KeyCode::H, KeyCode::K, fast);

    for (mut velocity, cup) in cup_query.iter_mut() {
        match cup.0 {
            BallKind::Blue => velocity.angvel = ang_vel_0,
            BallKind::Red => velocity.angvel = ang_vel_1,
        }
    }
}

pub fn ui_helper_show_system(
    mut query: Query<(&mut Visibility, &mut CupUiHelper)>,
    input: Res<Input<KeyCode>>,
    time: Res<Time>,
) {
    for (mut vis, mut helper) in query.iter_mut() {
        helper.0.tick(time.delta());
        if helper.0.just_finished() {
            vis.is_visible = false;
        }
    }
    if input.just_pressed(KeyCode::Slash) {
        for (mut vis, mut helper) in query.iter_mut() {
            vis.is_visible = !vis.is_visible;
            helper.0.pause();
        }
    }
}

#[derive(Component)]
pub struct Cup(pub BallKind);

#[derive(Component)]
pub struct CupUiHelper(pub Timer);

pub fn spawn_cups(mut commands: Commands, asset_server: Res<AssetServer>) {
    let mut rng = rand::thread_rng();
    let spawn_jar_instead = rng.gen_bool(0.25);
    if spawn_jar_instead {
        spawn_jar(-50.0, BallKind::Blue, &mut commands, &asset_server);
    } else {
        spawn_cup(-50.0, BallKind::Blue, &mut commands, &asset_server);
    }
    spawn_cup(50.0, BallKind::Red, &mut commands, &asset_server);
    spawn_centre_ui_helper(&mut commands, &asset_server);
}

fn spawn_centre_ui_helper(commands: &mut Commands, asset_server: &Res<AssetServer>) {
    let texture = asset_server.load("ui_helper_2.png");
    let transform = Transform::from_xyz(0.0, -20.0, 0.0);

    let mut timer = Timer::new(Duration::from_secs(7), false);
    timer.pause();
    commands
        .spawn()
        .insert_bundle(SpriteBundle {
            sprite: Sprite {
                color: Color::rgba(1.0, 1.0, 1.0, 0.7),
                ..default()
            },
            texture,
            transform,
            ..default()
        })
        .insert(CupUiHelper(timer));
}

fn spawn_cup(
    x_offset: f32,
    ball_kind: BallKind,
    commands: &mut Commands,
    asset_server: &AssetServer,
) {
    let sprite_tex = asset_server.load("cup.png");

    let ui_helper_tex = {
        let fname = if x_offset < 0.0 {
            "ui_helper_l.png"
        } else {
            "ui_helper_r.png"
        };
        asset_server.load(fname)
    };
    let shape = {
        let thickness = 4.0;
        let height = 30.0;
        let width = 50.0;
        let side = Collider::cuboid(thickness / 2.0, height / 2.0);
        let bottom = Collider::cuboid(width / 2.0, thickness / 2.0);
        let offset = width / 2.0 - thickness / 2.0;
        vec![
            (Vec2::new(-offset, height / 2.0), 0.0, side.clone()),
            (Vec2::new(0.0, thickness / 2.0), 0.0, bottom),
            (Vec2::new(offset, height / 2.0), 0.0, side),
        ]
    };
    let transform = Transform::from_xyz(x_offset, 0.0, 0.0);
    let mut ui_transform = transform.clone();
    let color = ball_kind.to_color();
    commands
        .spawn()
        .insert(RigidBody::Dynamic)
        .insert(LockedAxes::TRANSLATION_LOCKED)
        .insert(Collider::compound(shape))
        .insert(Sleeping::disabled())
        .insert(Velocity::default())
        .insert(Ccd::enabled())
        .insert(CollisionGroups::new(0b0001, 0b0111))
        .insert(GameOnlyMarker)
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
        .insert(Cup(ball_kind));

    ui_transform.translation.y += 60.0;
    let mut timer = Timer::new(Duration::from_secs(7), false);
    timer.pause();
    commands
        .spawn()
        .insert_bundle(SpriteBundle {
            sprite: Sprite {
                color: Color::rgba(1.0, 1.0, 1.0, 0.7),
                ..default()
            },
            texture: ui_helper_tex,
            transform: ui_transform,
            ..default()
        })
        .insert(GameOnlyMarker)
        .insert(CupUiHelper(timer));
}
