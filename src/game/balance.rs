use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::game::ball::{Ball, BallKind};
use crate::game::goals::Mix;
use crate::game::GameOnlyMarker;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct BalanceCounter {
    ball_count: HashMap<BallKind, usize>,
}

impl BalanceCounter {
    pub fn new() -> Self {
        BalanceCounter {
            ball_count: HashMap::new(),
        }
    }

    pub fn reset(&mut self) {
        self.ball_count.clear();
    }

    fn incr(&mut self, ball_id: BallKind) {
        *self.ball_count.entry(ball_id).or_insert(0) += 1;
    }

    fn decr(&mut self, ball_id: BallKind) {
        match self.ball_count.get_mut(&ball_id) {
            Some(0) => warn!("Tried to decr 0 in map (key: {:?}), not doing.", ball_id),
            Some(val) => *val -= 1,
            None => warn!("Tried to decr nonexistent count (key: {:?})", ball_id),
        }
    }

    fn total_count(&self) -> usize {
        let mut i = 0;
        for v in self.ball_count.values() {
            i += v;
        }
        i
    }

    pub fn total_weight(&self) -> f32 {
        //TODO, support dynamic weight of balls
        0.8 * (self.total_count() as f32)
    }

    pub fn calculate_ratio(&self) -> f32 {
        // TODO: Do this properly
        let a = *self.ball_count.get(&BallKind::Blue).unwrap_or(&0);
        let b = *self.ball_count.get(&BallKind::Red).unwrap_or(&0);
        a as f32 / b as f32
    }

    pub fn ratios_and_score(&self, target_mix: &Mix) -> (String, String, f32) {
        let (a_type, a_target, b_type, b_target) = match target_mix {
            Mix::FiftyFifty => (BallKind::Blue, 50f32, BallKind::Red, 50f32),
            Mix::AB {
                a_pct,
                a_kind,
                b_kind,
            } => (
                a_kind.clone(),
                *a_pct as f32,
                b_kind.clone(),
                (100 - *a_pct) as f32,
            ),
        };

        let total = self.total_count() as f32;

        let a_true_pct = {
            let v = *self.ball_count.get(&a_type).unwrap_or(&0usize);
            (v as f32 / total) * 100.0
        };
        let b_true_pct = {
            let v = *self.ball_count.get(&b_type).unwrap_or(&0usize);
            (v as f32 / total) * 100.0
        };

        let a_result_str = format!("{}. Goal {:.2}, Actual {:.2}", a_type, a_target, a_true_pct);
        let b_result_str = format!("{}. Goal {:.2}, Actual {:.2}", b_type, b_target, b_true_pct);

        let score = pct_to_score(a_target, a_true_pct) + pct_to_score(b_target, b_true_pct);
        (a_result_str, b_result_str, score)
    }
}

fn pct_to_score(target: f32, actual: f32) -> f32 {
    let abs_difference = (target - actual).abs();
    if abs_difference > 30.0 {
        0.0
    } else if abs_difference < 1.0 {
        50.0
    } else {
        ((30.0 - abs_difference) / 29.0) * 48.0
    }
}

#[test]
fn test_pct_to_score() {
    let target = 50.0;

    fn to_2dp(f: f32) -> f32 {
        (f * 100.0).round() / 100.0
    }

    assert_eq!(50.0, to_2dp(pct_to_score(target, 50.0)));
    assert_eq!(41.38, to_2dp(pct_to_score(target, 45.0)));
    assert_eq!(41.38, to_2dp(pct_to_score(target, 55.0)));
    assert_eq!(33.1, to_2dp(pct_to_score(target, 40.0)));
    assert_eq!(3.31, to_2dp(pct_to_score(target, 22.0)));
    assert_eq!(1.82, to_2dp(pct_to_score(target, 21.1)));
    assert_eq!(0.0, to_2dp(pct_to_score(target, 20.0)));
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

pub fn ball_sensor_system(
    mut active_events: EventReader<CollisionEvent>,
    balance_sensor_query: Query<Entity, With<BalanceSensor>>,
    ball_query: Query<&Ball>,
    mut balance_text_query: Query<&mut Text, With<BalanceText>>,
    mut balance_counter: ResMut<BalanceCounter>,
) {
    //TODO: Consider rework below:
    //
    // Right now this system works by counting balls that enter or leave the sensor zone
    // This is flawed because:
    // 1. It counts too soon, a ball's weight is counted before it even hits the floor
    // 2. Balls can pile up outside of the sense
    //
    // Fixing (2) will only make (1) worse/more apparent
    //
    // Poss Solution:
    // Keep track of entities in bigger sensor area, don't count them until their velocity is ~0
    // Probably revisit those with ~0 vel in case they are nudged

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
                            balance_counter.incr(ball.0.clone());
                            counter_changed = true;
                        }
                    }
                }
                CollisionEvent::Stopped(e1, e2, _flags) => {
                    if let Some(other_entity) =
                        other_entity_if_match(&balance_sensor_entity, *e1, *e2)
                    {
                        if let Ok(ball) = ball_query.get(other_entity) {
                            balance_counter.decr(ball.0.clone());
                            counter_changed = true;
                        }
                    }
                }
            }
        }
        if counter_changed {
            println!("Counter changed: {:?}", balance_counter);
            let total_weight = balance_counter.total_weight();
            if let Ok(mut text) = balance_text_query.get_single_mut() {
                text.sections[0].value = format!("{:.2}", total_weight);
            }
        }
    }
}

#[derive(Component)]
pub struct BalanceSensor;

#[derive(Component)]
pub struct BalanceText;

pub fn spawn_balance(mut commands: Commands, asset_server: Res<AssetServer>) {
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
            (Vec2::new(0.0, (thickness / 2.0) + y_offset), 0.0, bottom),
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
        .insert(GameOnlyMarker)
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
        .insert(GameOnlyMarker)
        .insert(BalanceText);
}
