use crate::direction::Direction;
use crate::map::Map;
use crate::obstacle::Obstacle;
use crate::position::Position;

pub struct Rover {
    position: Position,
    direction: Direction,
    map: Map,
    obstacle_list: Option<Vec<Obstacle>>,
}

impl Rover {
    pub fn new(
        position: Position,
        direction: Direction,
        map: Map,
        obstacle_list: Option<Vec<Obstacle>>,
    ) -> Self {
        Rover {
            position,
            direction,
            map,
            obstacle_list,
        }
    }

    pub fn get_x(&self) -> usize {
        self.position.x
    }

    pub fn get_y(&self) -> usize {
        self.position.y
    }

    pub fn get_direction(&self) -> Direction {
        self.direction
    }

    pub fn run_commands(&mut self, commands: Vec<&str>) {
        for command in commands {
            match command {
                "f" => {
                    let position = self.move_forward();
                    match self.move_to(position) {
                        Ok(()) => {}
                        Err(_) => {
                            break;
                        }
                    }
                }

                "b" => {
                    let position = self.move_backward();
                    match self.move_to(position) {
                        Ok(()) => {}
                        Err(_) => break,
                    }
                }
                "l" => self.turn_left(),
                "r" => self.turn_right(),
                _ => {}
            }
        }
    }

    fn move_forward(&mut self) -> Position {
        let mut position = self.position.clone();
        match self.direction {
            Direction::N => {
                if position.y == self.map.get_y_limit() {
                    position.y = self.map.get_y_origin();
                } else {
                    position.y = position.y + 1;
                }
            }
            Direction::E => {
                if position.x == self.map.get_x_limit() {
                    position.x = self.map.get_x_origin();
                } else {
                    position.x = position.x + 1;
                }
            }
            Direction::S => {
                if position.y == self.map.get_y_origin() {
                    position.y = self.map.get_y_limit();
                } else {
                    position.y = position.y - 1;
                }
            }
            Direction::W => {
                if position.x == self.map.get_x_origin() {
                    position.x = self.map.get_x_limit();
                } else {
                    position.x = position.x - 1;
                }
            }
        }
        position
    }

    fn move_backward(&mut self) -> Position {
        let mut position = self.position.clone();
        match self.direction {
            Direction::N => {
                if position.y == self.map.get_y_origin() {
                    position.y = self.map.get_y_limit();
                } else {
                    position.y = position.y - 1;
                }
            }
            Direction::E => {
                if position.x == self.map.get_x_origin() {
                    position.x = self.map.get_x_limit();
                } else {
                    position.x = position.x - 1;
                }
            }
            Direction::S => {
                if position.y == self.map.get_y_limit() {
                    position.y = self.map.get_y_origin();
                } else {
                    position.y = position.y + 1;
                }
            }
            Direction::W => {
                if position.x == self.map.get_x_limit() {
                    position.x = self.map.get_x_origin();
                } else {
                    position.x = position.x + 1;
                }
            }
        }
        position
    }

    fn move_to(&mut self, position: Position) -> Result<(), &'static str> {
        match Map::is_position_obstructed(position, &self.obstacle_list) {
            true => Err("Rover is obstructed"),
            false => {
                self.position = position;
                Ok(())
            }
        }
    }

    fn turn_left(&mut self) {
        self.direction = Direction::turn_left(self.direction)
    }

    fn turn_right(&mut self) {
        self.direction = Direction::turn_right(self.direction)
    }
}

#[cfg(test)]
mod tests {

    use crate::direction::Direction;
    use crate::map::Map;
    use crate::obstacle::Obstacle;
    use crate::position::Position;
    use crate::rover::Rover;

    #[test]
    fn run_commands() {
        let map = Map::new(0, 0, 5, 5);
        let position = Position::new(3, 2);
        let mut rover = Rover::new(position, Direction::N, map, None);
        let commands = Vec::from(["f", "l", "b"]);
        rover.run_commands(commands);
        assert_eq!(rover.get_x(), 4);
        assert_eq!(rover.get_y(), 3);
        assert_eq!(rover.get_direction(), Direction::W);
    }

    #[test]
    fn wrapping_forwards() {
        let map = Map::new(0, 0, 5, 5);
        let position = Position::new(3, 2);
        let mut rover = Rover::new(position, Direction::N, map, None);
        let commands = Vec::from(["f", "f", "f", "f", "f", "r", "f", "f", "f"]);
        rover.run_commands(commands);
        assert_eq!(rover.get_x(), 0);
        assert_eq!(rover.get_y(), 1);
        assert_eq!(rover.get_direction(), Direction::E);
    }

    #[test]
    fn wrapping_backwards() {
        let map = Map::new(0, 0, 5, 5);
        let position = Position::new(3, 2);
        let mut rover = Rover::new(position, Direction::N, map, None);
        let commands = Vec::from(["b", "b", "b", "l", "b", "b", "b", "b"]);
        rover.run_commands(commands);
        assert_eq!(rover.get_x(), 1);
        assert_eq!(rover.get_y(), 5);
        assert_eq!(rover.get_direction(), Direction::W);
    }

    #[test]
    fn obstacle_in_the_way() {
        let map = Map::new(0, 0, 5, 5);
        let position = Position::new(3, 2);
        let obstacle_1 = Obstacle::new(Position::new(0, 3));
        let obstacle_2 = Obstacle::new(Position::new(2, 4));
        let obstacle_list = Vec::from([obstacle_1, obstacle_2]);
        let mut rover = Rover::new(position, Direction::N, map, Some(obstacle_list));
        let commands = Vec::from(["r", "b", "b", "b", "l", "f", "f", "f"]);
        rover.run_commands(commands);
        assert_eq!(rover.get_x(), 0);
        assert_eq!(rover.get_y(), 2);
        assert_eq!(rover.get_direction(), Direction::N);
    }
}
