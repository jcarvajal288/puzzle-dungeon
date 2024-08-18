use bevy::prelude::Component;
use bevy::scene::ron::de::Position;

#[derive(Component)]
pub struct Player {
    pub position: Position
}
