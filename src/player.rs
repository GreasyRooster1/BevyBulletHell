use crate::*;
use bevy::prelude::*;
use std::time::Duration;

#[derive(Component)]
pub struct Speed(f32);

#[derive(Component)]
pub struct Dash {
    speed: f32,
    timer: Timer,
}

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_player);
        app.add_systems(Update, (move_player, dash_player));
    }
}

fn spawn_player(mut commands: Commands, handle: Res<PlaceholderTex>) {
    commands.spawn((
        Player,
        Speed(4.0),
        Dash {
            speed: 7.0,
            timer: Timer::new(Duration::from_millis(500), TimerMode::Once),
        },
        Transform::from_xyz(0.0, 0.0, 0.0),
        Sprite::from_image(handle.0.clone()),
    ));
}

fn move_player(
    mut query: Query<(&mut Transform, &Speed), With<Player>>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
) {
    for (mut t, speed) in &mut query.iter_mut() {
        let mut vec = Vec3::ZERO;
        if keyboard_input.pressed(KeyCode::KeyW) {
            vec.y += 1.0;
        }
        if keyboard_input.pressed(KeyCode::KeyS) {
            vec.y -= 1.0;
        }
        if keyboard_input.pressed(KeyCode::KeyA) {
            vec.x -= 1.0;
        }
        if keyboard_input.pressed(KeyCode::KeyD) {
            vec.x += 1.0;
        }
        let move_vec = vec.normalize_or_zero() * speed.0;
        t.translation += move_vec;
    }
}

fn dash_player(
    mut query: Query<(&Dash, &mut Speed), With<Player>>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
) {
    for (dash, mut speed) in query.iter_mut() {
        if !keyboard_input.just_pressed(KeyCode::Space) {
            continue;
        }
        speed.0 = dash.speed;
    }
}

