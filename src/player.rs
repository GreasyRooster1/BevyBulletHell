use crate::*;
use bevy::prelude::*;

#[derive(Component)]
pub struct Speed(f32);

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_player);
        app.add_systems(Update, move_player);
    }
}

fn spawn_player(mut commands: Commands, handle: Res<PlaceholderTex>) {
    commands.spawn((
        Player,
        Speed(3.0),
        Transform::from_xyz(0.0, 0.0, 0.0),
        Sprite::from_image(handle.0.clone()),
    ));
}

fn move_player(
    mut query: Query<(&mut Transform, &Speed), With<Player>>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
) {
    for (mut t, speed) in &mut query.iter_mut() {
        if keyboard_input.pressed(KeyCode::KeyW) {
            t.translation.y += speed.0;
        }
        if keyboard_input.pressed(KeyCode::KeyS) {
            t.translation.y -= speed.0;
        }
        if keyboard_input.pressed(KeyCode::KeyA) {
            t.translation.x -= speed.0;
        }
        if keyboard_input.pressed(KeyCode::KeyD) {
            t.translation.x += speed.0;
        }
    }
}
