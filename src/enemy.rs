use bevy::prelude::*;

pub struct EnemyPlugin;

#[derive(Resource, Default)]
struct SpawnTimer(Timer);

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (spawn_enemies))
            .init_resource::<SpawnTimer>();
    }
}

fn spawn_enemies(timer: Res<SpawnTimer>) {}
