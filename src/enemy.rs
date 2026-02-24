use crate::PlaceholderTex;
use bevy::prelude::*;

pub struct EnemyPlugin;

#[derive(Resource, Default)]
struct SpawnTimer(Timer);

#[derive(Component)]
pub struct Enemy;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (spawn_enemies))
            .insert_resource(SpawnTimer(Timer::from_seconds(2.0, TimerMode::Repeating)));
    }
}

fn spawn_enemies(
    mut commands: Commands,
    mut timer: ResMut<SpawnTimer>,
    time: Res<Time>,
    handle: Res<PlaceholderTex>,
) {
    timer.0.tick(time.delta());

    if timer.0.just_finished() {
        commands.spawn((
            Enemy,
            Transform::from_xyz(0.0, 100.0, 0.0),
            Sprite::from_image(handle.0.clone()),
        ));
    }
}
