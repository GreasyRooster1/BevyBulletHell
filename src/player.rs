use crate::*;
use bevy::prelude::*;
use std::time::Duration;

#[derive(Component)]
pub struct Speed(f32);

#[derive(Component)]
pub struct Dash {
    speed: f32,
    timer: Timer,
    duration: i32,
    def_speed: f32,
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
            speed: 14.0,
            def_speed: 4.0,
            duration: 500,
            timer: Timer::from_seconds(0., TimerMode::Once),
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
    mut query: Query<(&mut Dash, &mut Speed), With<Player>>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
) {
    for (mut dash, mut speed) in query.iter_mut() {
        dash.timer.tick(time.delta());
        if keyboard_input.just_pressed(KeyCode::Space) && dash.timer.is_finished() {
            dash.timer = Timer::new(Duration::from_millis(dash.duration), TimerMode::Once);
            dash.def_speed = speed.0;
            speed.0 = dash.speed;
        }
        if dash.timer.just_finished() {
            speed.0 = dash.def_speed
        }
    }
}
