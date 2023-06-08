use std::path::Path;

use maze_rs::Maze;
use maze_rs::DepthFirstBuild;

fn main() {
    let mut maze = Maze::new(5, 10).unwrap();
    print!("{:?}", maze.center_point());
}