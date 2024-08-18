mod images;
mod level_map;
mod player;
#[path = "levels/level1.rs"] mod level1;

use bevy::asset::ron::de::Position;
use bevy::prelude::*;
use crate::images::{Images, load_images};
use crate::level_map::LevelMap;
use crate::player::Player;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(Images::default())
        .insert_resource(level_map::load_level_1())
        .add_systems(Startup, (load_images, setup).chain())
        //.add_systems(Update, draw)
        .run();
}

fn setup(mut commands: Commands, level_map: Res<LevelMap>, images: Res<Images>, windows: Query<&Window>, mut clear_color: ResMut<ClearColor>) {
    commands.spawn(Camera2dBundle::default());
    clear_color.0 = Color::hsl(211., 1., 0.8);

    let window_center = Vec2::new(windows.single().resolution.width() / 2., windows.single().resolution.height() / 2.);

    commands.spawn((
        SpriteBundle {
            texture: images.player1.clone(),
            transform: level_map::transform_from_position(&level1::PLAYER1_STARTING_POSITION, window_center, 1.0),
            ..default()
        },
        Player {
            position: level1::PLAYER1_STARTING_POSITION,
        }
    ));

    level_map.draw(commands, &images, window_center);
}

