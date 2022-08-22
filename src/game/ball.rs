use crate::game::cup::Cup;
use crate::game::GameOnlyMarker;
use bevy::prelude::*;
use bevy::sprite::Anchor;
use bevy_rapier2d::dynamics::{GravityScale, RigidBody, Sleeping};
use bevy_rapier2d::geometry::Collider;

pub fn spawn_ball_system(
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

pub enum BallKind {
    Red,
    Blue,
}

#[derive(Component)]
pub struct Ball(pub usize);

fn spawn_ball(x_offset: f32, idx: usize, commands: &mut Commands, asset_server: &AssetServer) {
    let radius = 2.8;
    let sprite_tex = asset_server.load("ball.png");
    let transform = Transform::from_xyz(x_offset, 100.0, 1.0);
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
        //TODO: Ball CCD probably good but also likely a performance bottleneck. Revisit
        // .insert(Ccd::enabled())
        .insert(GravityScale(1.0))
        .insert(Ball(idx))
        .insert(GameOnlyMarker)
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
