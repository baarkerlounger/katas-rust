#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Direction {
    N,
    E,
    S,
    W,
}

impl Direction {
    pub fn turn_left(direction: Direction) -> Direction {
        match direction {
            Direction::N => Direction::W,
            Direction::E => Direction::N,
            Direction::S => Direction::E,
            Direction::W => Direction::S,
        }
    }

    pub fn turn_right(direction: Direction) -> Direction {
        match direction {
            Direction::N => Direction::E,
            Direction::E => Direction::S,
            Direction::S => Direction::W,
            Direction::W => Direction::N,
        }
    }
}
