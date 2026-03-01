use crate::{PlaceholderTex, Player, Velocity, get_random_vec3};
use bevy::prelude::*;

pub struct EnemyPlugin;

#[derive(Resource, Default)]
struct SpawnTimer(Timer);

#[derive(Component)]
pub struct Enemy;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (spawn_enemies, rotate_rock))
            .insert_resource(SpawnTimer(Timer::from_seconds(1.0, TimerMode::Repeating)));
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

        spawn_rock(&mut commands, &asset_server, pos, vel);
        spawn_missile(&mut commands, &asset_server, pos, vel);
    }
}

fn rotate_rock(mut query: Query<&mut Transform, With<Rock>>) {
    for mut transform in query.iter_mut() {
        transform.rotate_z(0.1);
    }
}

fn guide_missile(
    mut query: Query<(&mut Velocity, &Transform, &mut Missile)>,
    player_transform: Single<&Transform, With<Player>>,
) {
    let p_pos = player_transform.into_inner().translation;
    for (mut vel, t, mut missile) in query.iter_mut() {
        if (missile.fuel > 0.0) {
            missile.fuel -= 1.0;
            vel.into_inner().0 += ((p_pos - t.translation) as Vec3).normalize() * 0.25;
        }
    }
}

#[derive(Component)]
struct Rock;

fn spawn_rock(commands: &mut Commands, asset_server: &Res<AssetServer>, pos: Vec3, vel: Vec3) {
    commands.spawn((
        Enemy,
        Rock,
        Transform::from_translation(pos).with_scale(Vec3::splat(3.0)),
        Sprite::from_image(asset_server.load("rock.png")),
        Velocity(vel * 3.0),
    ));
}

#[derive(Component)]
struct Missile {
    fuel: f32,
}

fn spawn_missile(commands: &mut Commands, asset_server: &Res<AssetServer>, pos: Vec3, vel: Vec3) {
    commands.spawn((
        Enemy,
        Missile { fuel: 100.0 },
        Transform::from_translation(pos).with_scale(Vec3::splat(3.0)),
        Sprite::from_image(asset_server.load("missile.png")),
        Velocity(vel * 2.0),
    ));
}
