use crate::Obstacle;
use crate::Position;

pub struct Map {
    x_origin: usize,
    y_origin: usize,
    x_limit: usize,
    y_limit: usize,
}

impl Map {
    pub fn new(x_origin: usize, y_origin: usize, x_limit: usize, y_limit: usize) -> Self {
        Map {
            x_origin,
            y_origin,
            x_limit,
            y_limit,
        }
    }

    pub fn get_x_origin(&self) -> usize {
        self.x_origin
    }

    pub fn get_y_origin(&self) -> usize {
        self.y_origin
    }

    pub fn get_x_limit(&self) -> usize {
        self.x_limit
    }

    pub fn get_y_limit(&self) -> usize {
        self.y_limit
    }

    pub fn is_position_obstructed(
        position: Position,
        obstacle_list: &Option<Vec<Obstacle>>,
    ) -> bool {
        let mut obstructed: bool = false;

        match obstacle_list {
            Some(obstacle_list) => {
                for obstacle in obstacle_list {
                    println!(
                        "Obstacle position: {:?}, rover position: {:?}",
                        obstacle.get_position(),
                        position
                    );
                    if obstacle.get_position() == position {
                        obstructed = true;
                        break;
                    }
                }
            }
            None => {}
        }
        obstructed
    }
}
