use bevy::prelude::{Commands, Res, Resource, Transform, Vec2};
use bevy::sprite::SpriteBundle;
use bevy::utils::default;
use bevy::window::WindowResolution;
use crate::images::Images;

const MAP_WIDTH: usize = 23;
const MAP_HEIGHT: usize = 23;
const TILE_SIZE: f32 = 32.0;

#[derive(Clone)]
pub enum Tile {
    WALL,
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
        for y in 0..MAP_HEIGHT {
            for x in 0..MAP_WIDTH {
                commands.spawn(SpriteBundle {
                    texture: images.wall.clone(),
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
