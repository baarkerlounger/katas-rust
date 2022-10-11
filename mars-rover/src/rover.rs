use crate::direction::Direction;
use crate::map::Map;

pub struct Rover {
    x: usize,
    y: usize,
    direction: Direction,
    map: Map,
}

impl Rover {
    pub fn new(x: usize, y: usize, direction: Direction, map: Map) -> Self {
        Rover {
            x,
            y,
            direction,
            map,
        }
    }

    pub fn get_x(&self) -> usize {
        self.x
    }

    pub fn get_y(&self) -> usize {
        self.y
    }

    pub fn get_direction(&self) -> Direction {
        self.direction
    }

    pub fn run_commands(&mut self, commands: Vec<&str>) {
        for command in commands {
            match command {
                "f" => self.move_forward(),
                "b" => self.move_backward(),
                "l" => self.turn_left(),
                "r" => self.turn_right(),
                _ => {}
            }
        }
    }

    fn move_forward(&mut self) {
        match self.direction {
            Direction::N => {
                if self.y == self.map.get_y_limit() {
                    self.y = self.map.get_y_origin();
                } else {
                    self.y = self.y + 1;
                }
            }
            Direction::E => {
                if self.x == self.map.get_x_limit() {
                    self.x = self.map.get_x_origin();
                } else {
                    self.x = self.x + 1;
                }
            }
            Direction::S => {
                if self.y == self.map.get_y_origin() {
                    self.y = self.map.get_y_limit();
                } else {
                    self.y = self.y - 1;
                }
            }
            Direction::W => {
                if self.x == self.map.get_x_origin() {
                    self.x = self.map.get_x_limit();
                } else {
                    self.x = self.x - 1;
                }
            }
        }
    }

    fn move_backward(&mut self) {
        match self.direction {
            Direction::N => {
                if self.y == self.map.get_y_origin() {
                    self.y = self.map.get_y_limit();
                } else {
                    self.y = self.y - 1;
                }
            }
            Direction::E => {
                if self.x == self.map.get_x_origin() {
                    self.x = self.map.get_x_limit();
                } else {
                    self.x = self.x - 1;
                }
            }
            Direction::S => {
                if self.y == self.map.get_y_limit() {
                    self.y = self.map.get_y_origin();
                } else {
                    self.y = self.y + 1;
                }
            }
            Direction::W => {
                if self.x == self.map.get_x_limit() {
                    self.x = self.map.get_x_origin();
                } else {
                    self.x = self.x + 1;
                }
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
    use crate::rover::Rover;

    #[test]
    fn run_commands() {
        let map = Map::new(0, 0, 5, 5);
        let mut rover = Rover::new(3, 2, Direction::N, map);
        let commands = Vec::from(["f", "l", "b"]);
        rover.run_commands(commands);
        assert_eq!(rover.get_x(), 4);
        assert_eq!(rover.get_y(), 3);
        assert_eq!(rover.get_direction(), Direction::W);
    }

    #[test]
    fn wrapping_forwards() {
        let map = Map::new(0, 0, 5, 5);
        let mut rover = Rover::new(3, 2, Direction::N, map);
        let commands = Vec::from(["f", "f", "f", "f", "f", "r", "f", "f", "f"]);
        rover.run_commands(commands);
        assert_eq!(rover.get_x(), 0);
        assert_eq!(rover.get_y(), 1);
        assert_eq!(rover.get_direction(), Direction::E);
    }

    #[test]
    fn wrapping_backwards() {
        let map = Map::new(0, 0, 5, 5);
        let mut rover = Rover::new(3, 2, Direction::N, map);
        let commands = Vec::from(["b", "b", "b", "l", "b", "b", "b", "b"]);
        rover.run_commands(commands);
        assert_eq!(rover.get_x(), 1);
        assert_eq!(rover.get_y(), 5);
        assert_eq!(rover.get_direction(), Direction::W);
    }

    #[test]
    fn move_forwards_facing_north() {
        let map = Map::new(0, 0, 5, 5);
        let mut rover = Rover::new(3, 2, Direction::N, map);
        rover.move_forward();
        assert_eq!(rover.get_x(), 3);
        assert_eq!(rover.get_y(), 3);
    }

    #[test]
    fn move_backwards_facing_north() {
        let map = Map::new(0, 0, 5, 5);
        let mut rover = Rover::new(3, 2, Direction::N, map);
        rover.move_backward();
        assert_eq!(rover.get_x(), 3);
        assert_eq!(rover.get_y(), 1);
    }

    #[test]
    fn turn_left_facing_north() {
        let map = Map::new(0, 0, 5, 5);
        let mut rover = Rover::new(3, 2, Direction::N, map);
        rover.turn_left();
        assert_eq!(rover.get_direction(), Direction::W);
    }

    #[test]
    fn turn_right_facing_north() {
        let map = Map::new(0, 0, 5, 5);
        let mut rover = Rover::new(3, 2, Direction::N, map);
        rover.turn_right();
        assert_eq!(rover.get_direction(), Direction::E);
    }

    #[test]
    fn move_forwards_facing_south() {
        let map = Map::new(0, 0, 5, 5);
        let mut rover = Rover::new(3, 2, Direction::S, map);
        rover.move_forward();
        assert_eq!(rover.get_x(), 3);
        assert_eq!(rover.get_y(), 1);
    }

    #[test]
    fn move_backwards_facing_south() {
        let map = Map::new(0, 0, 5, 5);
        let mut rover = Rover::new(3, 2, Direction::S, map);
        rover.move_backward();
        assert_eq!(rover.get_x(), 3);
        assert_eq!(rover.get_y(), 3);
    }

    #[test]
    fn turn_left_facing_south() {
        let map = Map::new(0, 0, 5, 5);
        let mut rover = Rover::new(3, 2, Direction::S, map);
        rover.turn_left();
        assert_eq!(rover.get_direction(), Direction::E);
    }

    #[test]
    fn turn_right_facing_south() {
        let map = Map::new(0, 0, 5, 5);
        let mut rover = Rover::new(3, 2, Direction::S, map);
        rover.turn_right();
        assert_eq!(rover.get_direction(), Direction::W);
    }

    #[test]
    fn move_forwards_facing_east() {
        let map = Map::new(0, 0, 5, 5);
        let mut rover = Rover::new(3, 2, Direction::E, map);
        rover.move_forward();
        assert_eq!(rover.get_x(), 4);
        assert_eq!(rover.get_y(), 2);
    }

    #[test]
    fn move_backwards_facing_east() {
        let map = Map::new(0, 0, 5, 5);
        let mut rover = Rover::new(3, 2, Direction::E, map);
        rover.move_backward();
        assert_eq!(rover.get_x(), 2);
        assert_eq!(rover.get_y(), 2);
    }

    #[test]
    fn turn_left_facing_east() {
        let map = Map::new(0, 0, 5, 5);
        let mut rover = Rover::new(3, 2, Direction::E, map);
        rover.turn_left();
        assert_eq!(rover.get_direction(), Direction::N);
    }

    #[test]
    fn turn_right_facing_east() {
        let map = Map::new(0, 0, 5, 5);
        let mut rover = Rover::new(3, 2, Direction::E, map);
        rover.turn_right();
        assert_eq!(rover.get_direction(), Direction::S);
    }

    #[test]
    fn move_forwards_facing_west() {
        let map = Map::new(0, 0, 5, 5);
        let mut rover = Rover::new(3, 2, Direction::W, map);
        rover.move_forward();
        assert_eq!(rover.get_x(), 2);
        assert_eq!(rover.get_y(), 2);
    }

    #[test]
    fn move_backwards_facing_west() {
        let map = Map::new(0, 0, 5, 5);
        let mut rover = Rover::new(3, 2, Direction::W, map);
        rover.move_backward();
        assert_eq!(rover.get_x(), 4);
        assert_eq!(rover.get_y(), 2);
    }

    #[test]
    fn turn_left_facing_west() {
        let map = Map::new(0, 0, 5, 5);
        let mut rover = Rover::new(3, 2, Direction::W, map);
        rover.turn_left();
        assert_eq!(rover.get_direction(), Direction::S);
    }

    #[test]
    fn turn_right_facing_west() {
        let map = Map::new(0, 0, 5, 5);
        let mut rover = Rover::new(3, 2, Direction::W, map);
        rover.turn_right();
        assert_eq!(rover.get_direction(), Direction::N);
    }
}
