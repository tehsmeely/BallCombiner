use bevy::prelude::*;
use std::time::Duration;

#[derive(Component, Debug)]
pub struct TimedRemoval {
    timer: Timer,
}

impl TimedRemoval {
    pub fn new(lifetime: Duration) -> Self {
        Self {
            timer: Timer::new(lifetime, false),
        }
    }

    fn system(mut commands: Commands, mut query: Query<(Entity, &mut Self)>, time: Res<Time>) {
        for (entity, mut timed_removal) in query.iter_mut() {
            timed_removal.timer.tick(time.delta());
            if timed_removal.timer.just_finished() {
                commands.entity(entity).despawn_recursive();
            }
        }
    }
}

pub struct GeneralComponentsPlugin;
impl Plugin for GeneralComponentsPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(TimedRemoval::system);
    }
}
