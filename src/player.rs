use crate::*;
use bevy::prelude::*;

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
        Transform::from_xyz(0.0, 0.0, 0.0),
        Sprite::from_image(handle.0.clone()),
    ));
}

fn move_player(
    mut query: Query<&mut Transform, With<Player>>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
) {
    for mut t in query.iter_mut() {
        if keyboard_input.pressed(KeyCode::KeyW) {
            t.translation.y += 3.0;
        }
        if keyboard_input.pressed(KeyCode::KeyS) {
            t.translation.y -= 3.0;
        }
        if keyboard_input.pressed(KeyCode::KeyA) {
            t.translation.x -= 3.0;
        }
        if keyboard_input.pressed(KeyCode::KeyD) {
            t.translation.x += 3.0;
        }
    }
}

