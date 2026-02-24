use bevy::prelude::*;

pub struct EnemyPlugin;

#[derive(Resource, Default)]
struct SpawnTimer(Timer);

#[derive(Component)]
pub struct Enemy;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (spawn_enemies))
            .init_resource::<SpawnTimer>();
    }
}

fn spawn_enemies(mut commands: Commands, mut timer: ResMut<SpawnTimer>, time: Res<Time>) {
    timer.0.tick(time.delta());

    if timer.0.just_finished() {
        commands.spawn((Enemy));
    }
}
