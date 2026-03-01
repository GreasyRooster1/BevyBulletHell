use crate::{PlaceholderTex, Player, Velocity, get_random_vec3};
use bevy::prelude::*;

pub struct EnemyPlugin;

#[derive(Resource, Default)]
struct SpawnTimer(Timer);

#[derive(Component)]
pub struct Enemy;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (spawn_enemies))
            .insert_resource(SpawnTimer(Timer::from_seconds(0.1, TimerMode::Repeating)));
    }
}

fn spawn_enemies(
    mut commands: Commands,
    mut timer: ResMut<SpawnTimer>,
    time: Res<Time>,
    handle: Res<PlaceholderTex>,
    asset_server: Res<AssetServer>,
    mut windows: Query<&mut Window>,
    player_transform: Single<&Transform, With<Player>>,
) {
    timer.0.tick(time.delta());

    if timer.0.just_finished() {
        let player_t = player_transform.into_inner();
        let window = windows.single_inner().unwrap();
        let pos = get_random_vec3().normalize() * window.width();
        let vel = (player_t.translation - pos).normalize();
        spawn_rock(commands, asset_server, pos, vel)
    }
}

#[derive(Component)]
struct Rock;

fn spawn_rock(mut commands: Commands, asset_server: Res<AssetServer>, pos: Vec3, vel: Vec3) {
    commands.spawn((
        Enemy,
        Rock,
        Transform::from_translation(pos).with_scale(Vec3::splat(3.0)),
        Sprite::from_image(asset_server.load("rock.png")),
        Velocity(vel * 3.0),
    ));
}
