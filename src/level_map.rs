use bevy::asset::Handle;
use bevy::prelude::{Commands, Image, Res, Resource, Transform, Vec2};
use bevy::sprite::SpriteBundle;
use bevy::utils::default;
use crate::images::Images;

#[path = "levels/level1.rs"] mod level1;

const MAP_WIDTH: usize = 23;
const MAP_HEIGHT: usize = 23;
const TILE_SIZE: f32 = 32.0;

#[derive(Clone)]
pub enum Tile {
    WALL,
    FLOOR,
}

#[derive(Resource)]
pub struct LevelMap {
    pub grid: Vec<Vec<Tile>>
}

impl Default for LevelMap {
    fn default() -> Self {
        Self {
            grid: vec![vec![Tile::WALL; MAP_HEIGHT]; MAP_WIDTH],
        }
    }
}

impl LevelMap {
    pub fn draw(&self, mut commands: Commands, images: &Res<Images>, window_center: Vec2) {
        let level_tiles = read_level_tiles(level1::MAP_DATA);
        for y in 0..MAP_HEIGHT {
            for x in 0..MAP_WIDTH {
                commands.spawn(SpriteBundle {
                    //texture: images.wall.clone(),
                    texture: get_image_for_tile(level_tiles.get(y).unwrap().get(x).unwrap(), images),
                    transform: Transform::from_xyz(
                        (x as f32 * TILE_SIZE) - window_center.x,
                        -(y as f32 * TILE_SIZE) + window_center.y,
                        0.0
                    ),
                    ..default()
                });
            }
        }
    }
}

fn get_image_for_tile(tile: &Tile, images: &Res<Images>) -> Handle<Image> {
    return match tile {
        Tile::WALL => images.wall.clone(),
        Tile::FLOOR => images.floor.clone(),
    }
}

fn read_level_tiles(map_data: &str) -> Vec<Vec<Tile>> {
    return map_data.lines().map(|line| {
        return line.chars().map(|character| {
            return match character {
                '#' => Tile::WALL,
                '.' => Tile::FLOOR,
                _ => Tile::WALL
            }
        }).collect()
    }).collect();
}
