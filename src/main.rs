use bevy::app::App;
use bevy::prelude::Component;

#[derive(Component)]
struct Player;

#[derive(Component)]
struct Position {
    x: f32,
    y: f32,
}

fn main() {
    App::new().run();
}
