use std::time::Duration;

use bevy::prelude::*;

use rand::prelude::*;

pub struct SpawnerPlugin;

impl Plugin for SpawnerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, spawn_things);
    }
}

trait SpawnStrategy: Send + Sync + 'static {
    fn spawn(&self, commands: &mut Commands, position: Vec3);
}

pub struct Spawnable<T: Bundle + Clone>(pub T);

impl<T: Bundle + Clone> SpawnStrategy for Spawnable<T> {
    fn spawn(&self, commands: &mut Commands, position: Vec3) {
        commands
            .spawn(self.0.clone())
            .insert(Transform::from_translation(position));
    }
}

#[derive(Component)]
pub struct Spawner {
    timer: Timer,
    zone: Vec2,
    spawnable: Box<dyn SpawnStrategy>,
}

impl Spawner {
    pub fn new(delay: Duration, zone: Vec2, spawnable: Spawnable<impl Bundle + Clone>) -> Self {
        Self {
            timer: Timer::from_seconds(delay.as_secs_f32(), TimerMode::Repeating),
            zone,
            spawnable: Box::new(spawnable),
        }
    }
}

fn spawn_things(
    mut commands: Commands,
    mut query: Query<(&mut Spawner, &GlobalTransform)>,
    time: Res<Time>,
) {
    for (mut spawner, transform) in query.iter_mut() {
        spawner.timer.tick(time.delta());
        if !spawner.timer.is_finished() {
            continue;
        }
        let mut rng = rand::rng();
        let rand_translation = Vec3::new(
            if spawner.zone.x as u32 != 0 {
                rng.random_range((-spawner.zone.x / 2.0)..(spawner.zone.x / 2.0))
            } else {
                0.0
            },
            if spawner.zone.y as u32 != 0 {
                rng.random_range((-spawner.zone.y / 2.0)..(spawner.zone.y / 2.0))
            } else {
                0.0
            },
            0.0,
        );
        spawner
            .spawnable
            .spawn(&mut commands, transform.translation() + rand_translation);
    }
}
