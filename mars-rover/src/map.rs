use crate::Obstacle;
use crate::Position;

#[derive(Copy, Clone, Debug, PartialEq)]
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

    pub fn x_origin(&self) -> usize {
        self.x_origin
    }

    pub fn y_origin(&self) -> usize {
        self.y_origin
    }

    pub fn x_limit(&self) -> usize {
        self.x_limit
    }

    pub fn y_limit(&self) -> usize {
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
                    if obstacle.position() == position {
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

#[cfg(test)]
mod position_obstructed {

    use crate::map::Map;
    use crate::obstacle::Obstacle;
    use crate::position::Position;

    #[test]
    fn position_is_obstructed() {
        let position = Position::new(1, 3);
        let obstacle_list = Some(Vec::from([
            Obstacle::new(Position::new(2, 4)),
            Obstacle::new(Position::new(1, 3)),
        ]));
        let result = Map::is_position_obstructed(position, &obstacle_list);
        assert_eq!(result, true);
    }

    #[test]
    fn position_is_not_obstructed() {
        let position = Position::new(2, 3);
        let obstacle_list = Some(Vec::from([
            Obstacle::new(Position::new(2, 4)),
            Obstacle::new(Position::new(1, 3)),
        ]));
        let result = Map::is_position_obstructed(position, &obstacle_list);
        assert_eq!(result, false);
    }

    #[test]
    fn there_are_no_obstructions() {
        let position = Position::new(2, 3);
        let obstacle_list = None;
        let result = Map::is_position_obstructed(position, &obstacle_list);
        assert_eq!(result, false);
    }
}
