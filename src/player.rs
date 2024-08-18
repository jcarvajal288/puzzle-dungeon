use bevy::prelude::Component;
use bevy::scene::ron::de::Position;

#[derive(Component)]
pub struct Player {
    pub position: Position
}

impl Player {
    pub fn left(&self) -> Position {
        return Position {
            col: self.position.col - 1,
            line: self.position.line
        }
    }

    pub fn right(&self) -> Position {
        return Position {
            col: self.position.col + 1,
            line: self.position.line
        }
    }

    pub fn up(&self) -> Position {
        return Position {
            col: self.position.col,
            line: self.position.line + 1
        }
    }

    pub fn down(&self) -> Position {
        return Position {
            col: self.position.col,
            line: self.position.line - 1
        }
    }

    pub fn move_left(&mut self) {
        self.position = self.left();
    }

    pub fn move_right(&mut self) {
        self.position = self.right();
    }

    pub fn move_up(&mut self) {
        self.position = self.up();
    }

    pub fn move_down(&mut self) {
        self.position = self.down();
    }
}
