use std::path::Path;

use mazeir::Maze;
use mazeir::DepthFirstBuild;

fn main() {
    let mut maze = Maze::new(5, 10).unwrap();
    print!("{:?}", maze.center_point());
}