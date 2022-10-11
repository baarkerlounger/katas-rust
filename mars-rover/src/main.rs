mod direction;
mod map;
mod rover;

use direction::Direction;
use map::Map;
use rover::Rover;

fn main() {
    let map = Map::new(0, 0, 5, 5);
    let mut rover = Rover::new(3, 2, Direction::N, map);
    let commands = Vec::from(["f", "l", "b"]);
    rover.run_commands(commands);
    println!(
        "Command execution completed, rover is at x: {}, y: {} facing: {:?}",
        rover.get_x(),
        rover.get_y(),
        rover.get_direction()
    );
}
