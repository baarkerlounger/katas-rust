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

    pub fn x(&self) -> usize {
        self.position.x
    }

    pub fn y(&self) -> usize {
        self.position.y
    }

    pub fn direction(&self) -> Direction {
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
                if position.y == self.map.y_limit() {
                    position.y = self.map.y_origin();
                } else {
                    position.y = position.y + 1;
                }
            }
            Direction::E => {
                if position.x == self.map.x_limit() {
                    position.x = self.map.x_origin();
                } else {
                    position.x = position.x + 1;
                }
            }
            Direction::S => {
                if position.y == self.map.y_origin() {
                    position.y = self.map.y_limit();
                } else {
                    position.y = position.y - 1;
                }
            }
            Direction::W => {
                if position.x == self.map.x_origin() {
                    position.x = self.map.x_limit();
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
                if position.y == self.map.y_origin() {
                    position.y = self.map.y_limit();
                } else {
                    position.y = position.y - 1;
                }
            }
            Direction::E => {
                if position.x == self.map.x_origin() {
                    position.x = self.map.x_limit();
                } else {
                    position.x = position.x - 1;
                }
            }
            Direction::S => {
                if position.y == self.map.y_limit() {
                    position.y = self.map.y_origin();
                } else {
                    position.y = position.y + 1;
                }
            }
            Direction::W => {
                if position.x == self.map.x_limit() {
                    position.x = self.map.x_origin();
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
mod run_commands {

    use crate::direction::Direction;
    use crate::map::Map;
    use crate::obstacle::Obstacle;
    use crate::position::Position;
    use crate::rover::Rover;
    use lazy_static::lazy_static;

    lazy_static! {
        static ref MAP: Map = Map::new(0, 0, 5, 5);
        static ref POSITION: Position = Position::new(3, 2);
    }

    #[test]
    fn move_forward_turn_move_backward() {
        let mut rover = Rover::new(*POSITION, Direction::N, *MAP, None);
        let commands = Vec::from(["f", "l", "b"]);
        rover.run_commands(commands);
        assert_eq!(rover.x(), 4);
        assert_eq!(rover.y(), 3);
        assert_eq!(rover.direction(), Direction::W);
    }

    #[test]
    fn wrapping_forwards() {
        let mut rover = Rover::new(*POSITION, Direction::N, *MAP, None);
        let commands = Vec::from(["f", "f", "f", "f", "f", "r", "f", "f", "f"]);
        rover.run_commands(commands);
        assert_eq!(rover.x(), 0);
        assert_eq!(rover.y(), 1);
        assert_eq!(rover.direction(), Direction::E);
    }

    #[test]
    fn wrapping_backwards() {
        let mut rover = Rover::new(*POSITION, Direction::N, *MAP, None);
        let commands = Vec::from(["b", "b", "b", "l", "b", "b", "b", "b"]);
        rover.run_commands(commands);
        assert_eq!(rover.x(), 1);
        assert_eq!(rover.y(), 5);
        assert_eq!(rover.direction(), Direction::W);
    }

    #[test]
    fn obstacle_in_the_way() {
        let obstacle_list = Vec::from([
            Obstacle::new(Position::new(0, 3)),
            Obstacle::new(Position::new(2, 4)),
        ]);
        let mut rover = Rover::new(*POSITION, Direction::N, *MAP, Some(obstacle_list));
        let commands = Vec::from(["r", "b", "b", "b", "l", "f", "f", "f"]);
        rover.run_commands(commands);
        assert_eq!(rover.x(), 0);
        assert_eq!(rover.y(), 2);
        assert_eq!(rover.direction(), Direction::N);
    }
}
