use crate::game::cup::Cup;
use crate::game::GameOnlyMarker;
use bevy::prelude::*;
use bevy::sprite::Anchor;
use bevy_rapier2d::dynamics::{GravityScale, RigidBody, Sleeping};
use bevy_rapier2d::geometry::Collider;
use std::fmt::{Display, Formatter};

pub fn spawn_ball_system(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    cup_query: Query<(&Transform, &Cup)>,
    mut event_reader: EventReader<SpawnBallEvent>,
) {
    for _ in event_reader.iter() {
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

#[derive(Clone, Debug)]
pub struct SpawnBallEvent;

pub fn debug_spawn_ball_input_system(
    input: Res<Input<KeyCode>>,
    mut event_writer: EventWriter<SpawnBallEvent>,
) {
    if input.just_pressed(KeyCode::Return) {
        event_writer.send(SpawnBallEvent);
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

impl Display for BallKind {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Red => write!(f, "Red"),
            Self::Blue => write!(f, "Blue"),
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
