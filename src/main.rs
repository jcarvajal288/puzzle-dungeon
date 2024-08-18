use bevy::prelude::*;
use bevy::scene::ron::de::Position;
use crate::images::{Images, load_images};
use crate::level_map::LevelMap;
use crate::player::Player;

mod images;
mod level_map;
mod player;
#[path = "levels/level1.rs"] mod level1;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(Images::default())
        .insert_resource(level_map::load_level_1())
        .add_systems(Startup, (load_images, setup).chain())
        .add_systems(Update, player_movement_system)
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

fn player_movement_system(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut player_query: Query<(&mut Player, &mut Transform)>,
    level_map: Res<LevelMap>,
) {
    let (mut player, mut transform) = player_query.single_mut();

    if keyboard_input.just_pressed(KeyCode::ArrowLeft) && level_map.is_position_walkable(&player.left()) {
        transform.translation.x -= level_map::TILE_SIZE;
        player.move_left();
    } else if keyboard_input.just_pressed(KeyCode::ArrowRight) && level_map.is_position_walkable(&player.right()) {
        transform.translation.x += level_map::TILE_SIZE;
        player.move_right();
    } else if keyboard_input.just_pressed(KeyCode::ArrowUp) && level_map.is_position_walkable(&player.up()) {
        transform.translation.y += level_map::TILE_SIZE;
        player.move_up();
    } else if keyboard_input.just_pressed(KeyCode::ArrowDown) && level_map.is_position_walkable(&player.down()) {
        transform.translation.y -= level_map::TILE_SIZE;
        player.move_down();
    }
}

