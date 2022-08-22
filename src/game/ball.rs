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
            spawn_ball(
                transform.translation.x,
                cup.0.clone(),
                &mut commands,
                &asset_server,
            );
        }
    }
}

#[derive(Component, Hash, PartialEq, Eq, Debug, Clone)]
pub enum BallKind {
    Red,
    Blue,
}

impl BallKind {
    pub fn to_color(&self) -> Color {
        match self {
            Self::Red => Color::RED,
            Self::Blue => Color::BLUE,
        }
    }
}

#[derive(Component)]
pub struct Ball(pub BallKind);

fn spawn_ball(
    x_offset: f32,
    ball_kind: BallKind,
    commands: &mut Commands,
    asset_server: &AssetServer,
) {
    let radius = 2.8;
    let sprite_tex = asset_server.load("ball.png");
    let transform = Transform::from_xyz(x_offset, 100.0, 1.0);
    let color = ball_kind.to_color();
    commands
        .spawn()
        .insert(RigidBody::Dynamic)
        .insert(Collider::ball(radius))
        .insert(Sleeping::disabled())
        //TODO: Ball CCD probably good but also likely a performance bottleneck. Revisit
        // .insert(Ccd::enabled())
        .insert(GravityScale(1.0))
        .insert(Ball(ball_kind))
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
