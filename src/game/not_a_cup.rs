use crate::game::ball::BallKind;
use crate::game::cup::{Cup, CupUiHelper};
use crate::game::GameOnlyMarker;
use bevy::prelude::*;
use bevy::sprite::Anchor;
use bevy_rapier2d::prelude::*;
use std::time::Duration;

pub fn spawn_jar(
    x_offset: f32,
    ball_kind: BallKind,
    commands: &mut Commands,
    asset_server: &AssetServer,
) {
    let sprite_tex = asset_server.load("tank.png");
    let door_tex = asset_server.load("tank_door.png");
    let ui_helper_tex = asset_server.load("ui_helper_jar_l.png");

    let shape = {
        let thickness = 4.0;
        let height = 47.0;
        let width = 45.0;
        let side = Collider::cuboid(thickness / 2.0, height / 2.0);
        let small_side = Collider::cuboid(thickness / 2.0, (height - 10.0) / 2.0);
        let bottom = Collider::cuboid((width / 2.0) + 6.0, thickness / 2.0);
        let slide = Collider::cuboid(6.0, thickness / 2.0);
        let offset = width / 2.0 - thickness / 2.0;
        vec![
            // Left side
            (
                Vec2::new(-offset - 6.0, (height / 2.0) + 12.0),
                0.0,
                side.clone(),
            ),
            // Bottom
            (Vec2::new(0.0, (thickness / 2.0) + 7.0), 6.1, bottom),
            // Right side
            (
                Vec2::new(offset - 6.0, (height / 2.0) + 18.0),
                0.0,
                small_side,
            ),
            //Top slide
            (
                Vec2::new(offset + 4.0, (thickness / 2.0) + 20.0),
                6.1,
                slide,
            ),
        ]
    };
    let transform = Transform::from_xyz(x_offset, 0.0, 0.0);
    let mut door_trans = transform.clone();
    let mut ui_transform = transform.clone();
    let color = ball_kind.to_color();
    commands
        .spawn()
        .insert(RigidBody::Dynamic)
        .insert(LockedAxes::all())
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
        .insert(CollisionGroups::new(0b0010, 0b0011))
        .insert(Jar(ball_kind.clone()));

    // Door
    door_trans.translation.x += 16.0;
    door_trans.translation.y += 14.0;
    door_trans.translation.z = 1.0;
    let min_trans_y = door_trans.translation.y;
    let max_trans_y = door_trans.translation.y + 16.0;
    let locked_axes = LockedAxes::TRANSLATION_LOCKED_X.union(LockedAxes::ROTATION_LOCKED);
    commands
        .spawn()
        .insert(RigidBody::Dynamic)
        .insert(locked_axes)
        .insert(Collider::cuboid(2.0, 8.0))
        .insert(Sleeping::disabled())
        .insert(Velocity::default())
        .insert(Ccd::enabled())
        .insert(GameOnlyMarker)
        .insert_bundle(SpriteBundle {
            sprite: Sprite {
                color,
                anchor: Anchor::Center,
                ..default()
            },
            texture: door_tex,
            transform: door_trans,
            ..default()
        })
        .insert(CollisionGroups::new(0b0100, 0b0001))
        .insert(JarDoor {
            min_trans_y,
            max_trans_y,
        });

    ui_transform.translation.y += 60.0;
    ui_transform.translation.x -= 10.0;
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

#[derive(Component)]
pub struct JarDoor {
    max_trans_y: f32,
    min_trans_y: f32,
}

impl JarDoor {
    pub fn system(
        mut commands: Commands,
        mut door_query: Query<(&mut Transform, &mut Velocity, &mut JarDoor)>,
        input: Res<Input<KeyCode>>,
    ) {
        let vy = if input.pressed(KeyCode::W) {
            2.0
        } else if input.pressed(KeyCode::S) {
            -2.0
        } else {
            0.0
        };

        for (mut transform, mut velocity, mut door) in door_query.iter_mut() {
            let vy = if input.pressed(KeyCode::W) {
                20.0
            } else if input.pressed(KeyCode::S) {
                -20.0
            } else {
                0.0
            };
            velocity.linvel.y = vy;

            if transform.translation.y < door.min_trans_y {
                transform.translation.y = door.min_trans_y;
            }
            if transform.translation.y > door.max_trans_y {
                transform.translation.y = door.max_trans_y;
            }
        }
    }
}

#[derive(Component)]
pub struct Jar(pub BallKind);
