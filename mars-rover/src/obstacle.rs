use crate::position::Position;

pub struct Obstacle {
    position: Position,
}

impl Obstacle {
    pub fn new(position: Position) -> Self {
        Obstacle { position }
    }

    pub fn position(&self) -> Position {
        self.position
    }
}
