mod images;
mod level_map;

use bevy::prelude::*;
use crate::images::{Images, load_images};
use crate::level_map::LevelMap;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(Images::default())
        .insert_resource(LevelMap::default())
        .add_systems(Startup, (load_images, setup, draw).chain())
        .add_systems(Update, draw)
        .run();
}

fn setup(mut commands: Commands, mut clear_color: ResMut<ClearColor>) {
    commands.spawn(Camera2dBundle::default());
    clear_color.0 = Color::hsl(211., 1., 0.8);
}

fn draw(commands: Commands, level_map: Res<LevelMap>, images: Res<Images>) {
    level_map.draw(commands, &images);
}
