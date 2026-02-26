mod enemy;
mod player;

use crate::enemy::*;
use crate::player::*;
use bevy::app::App;
use bevy::asset::RenderAssetUsages;
use bevy::color::palettes::css;
use bevy::prelude::*;
use bevy::render::render_resource::{Extent3d, TextureDimension, TextureFormat};

#[derive(Resource)]
pub struct PlaceholderTex(Handle<Image>);

#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct Velocity(Vec2);

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(PreStartup, (setup, generate_placeholder_tex))
        .add_plugins(PlayerPlugin)
        .add_plugins(EnemyPlugin)
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2d);
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
