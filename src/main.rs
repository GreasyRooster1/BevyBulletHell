mod enemy;
mod player;

use crate::enemy::*;
use crate::player::*;
use bevy::app::App;
use bevy::asset::RenderAssetUsages;
use bevy::color::palettes::css;
use bevy::prelude::*;
use bevy::render::render_resource::{Extent3d, TextureDimension, TextureFormat};
use rand;
use rand::RngExt;

#[derive(Resource)]
pub struct PlaceholderTex(Handle<Image>);

#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct Velocity(Vec3);

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .add_systems(PreStartup, (setup, generate_placeholder_tex))
        .add_systems(Update, apply_velocity)
        .add_plugins(PlayerPlugin)
        .add_plugins(EnemyPlugin)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2d);
}

fn apply_velocity(mut query: Query<(&mut Transform, &Velocity)>) {
    for (mut t, vel) in &mut query.iter_mut() {
        t.translation += vel.0;
    }
}

fn generate_placeholder_tex(mut commands: Commands, mut images: ResMut<Assets<Image>>) {
    let image = Image::new_fill(
        Extent3d {
            width: 32,
            height: 32,
            depth_or_array_layers: 1,
        },
        TextureDimension::D2,
        &(css::BEIGE.to_u8_array()),
        TextureFormat::Rgba8UnormSrgb,
        RenderAssetUsages::MAIN_WORLD | RenderAssetUsages::RENDER_WORLD,
    );

    let handle = images.add(image);

    //commands.spawn(Sprite::from_image(handle.clone()));
    commands.insert_resource(PlaceholderTex(handle));
}

pub fn get_random_vec3() -> Vec3 {
    let mut rng = rand::rng();

    let x: f32 = rng.random_range(-1f32..=1f32);
    let y: f32 = rng.random_range(-1f32..=1f32);

    let vec = Vec3::new(x, y, 1.0);

    vec.normalize()
}
