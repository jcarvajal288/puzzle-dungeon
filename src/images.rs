use bevy::asset::{AssetServer, Handle};
use bevy::prelude::{Image, Res, ResMut, Resource};

#[derive(Resource)]
pub struct Images {
    pub wall: Handle<Image>,
}

impl Default for Images {
    fn default() -> Self {
        Self {
            wall: Handle::default()
        }
    }
}

pub fn load_images(mut images: ResMut<Images>, asset_server: Res<AssetServer>) {
    images.wall = asset_server.load("images/brick_brown_0.png")
}