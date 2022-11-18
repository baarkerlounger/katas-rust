mod direction;
mod map;
mod obstacle;
mod position;
mod rover;

use direction::Direction;
use map::Map;
use obstacle::Obstacle;
use position::Position;
use rover::Rover;

fn main() {
    let mut rover = Rover::new(
        Position::new(3, 2),
        Direction::N,
        Map::new(0, 0, 5, 5),
        Some(obstacle_list()),
    );
    let commands = Vec::from(["f", "l", "b"]);
    rover.run_commands(commands);
    println!(
        "Command execution completed, rover is at x: {}, y: {} facing: {:?}",
        rover.x(),
        rover.y(),
        rover.direction()
    );
}

fn obstacle_list() -> Vec<Obstacle> {
    let obstacle_1 = Obstacle::new(Position::new(0, 3));
    let obstacle_2 = Obstacle::new(Position::new(2, 4));
    Vec::from([obstacle_1, obstacle_2])
}
