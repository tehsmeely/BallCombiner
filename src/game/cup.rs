use crate::game::ball::BallKind;
use crate::game::GameOnlyMarker;
use bevy::prelude::*;
use bevy::sprite::Anchor;
use bevy_rapier2d::dynamics::{Ccd, LockedAxes, RigidBody, Sleeping, Velocity};
use bevy_rapier2d::geometry::Collider;

fn angvel_of_input(input: &Input<KeyCode>, ac: KeyCode, c: KeyCode) -> f32 {
    if input.pressed(ac) {
        1.0
    } else if input.pressed(c) {
        -1.0
    } else {
        0.0
    }
}

pub fn rotate_cup_system(mut cup_query: Query<(&mut Velocity, &Cup)>, input: Res<Input<KeyCode>>) {
    let ang_vel_0 = angvel_of_input(&input, KeyCode::A, KeyCode::D);
    let ang_vel_1 = angvel_of_input(&input, KeyCode::H, KeyCode::K);
    for (mut velocity, cup) in cup_query.iter_mut() {
        match cup.0 {
            BallKind::Blue => velocity.angvel = ang_vel_0,
            BallKind::Red => velocity.angvel = ang_vel_1,
        }
    }
}

#[derive(Component)]
pub struct Cup(pub BallKind);

pub fn spawn_cups(mut commands: Commands, asset_server: Res<AssetServer>) {
    spawn_cup(-50.0, BallKind::Blue, &mut commands, &asset_server);
    spawn_cup(50.0, BallKind::Red, &mut commands, &asset_server);
}

fn spawn_cup(
    x_offset: f32,
    ball_kind: BallKind,
    commands: &mut Commands,
    asset_server: &AssetServer,
) {
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
            (Vec2::new(0.0, thickness / 2.0), 0.0, bottom),
            (Vec2::new(offset, height / 2.0), 0.0, side),
        ]
    };
    let transform = Transform::from_xyz(x_offset, 0.0, 0.0);
    let color = ball_kind.to_color();
    commands
        .spawn()
        .insert(RigidBody::Dynamic)
        .insert(LockedAxes::TRANSLATION_LOCKED)
        .insert(Collider::compound(shape))
        .insert(Sleeping::disabled())
        .insert(Velocity::default())
        .insert(Ccd::enabled())
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
}
