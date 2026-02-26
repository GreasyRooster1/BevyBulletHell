use crate::{PlaceholderTex, get_random_vec3};
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
    asset_server: Res<AssetServer>,
    mut windows: Query<&mut Window>,
) {
    timer.0.tick(time.delta());

    if timer.0.just_finished() {
        spawn_rock(commands, asset_server, windows)
    }
}

#[derive(Component)]
struct Rock;

fn spawn_rock(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut windows: Query<&mut Window>,
) {
    let window = windows.single_inner().unwrap();
    commands.spawn((
        Enemy,
        Rock,
        Transform::from_translation(get_random_vec3() * window.width()),
        Sprite::from_image(asset_server.load("rock.png")),
    ));
}
