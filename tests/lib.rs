extern crate mazeir;

use std::path::Path;
use mazeir::{Maze, MazeError, DepthFirstBuild, Draw};

#[test]
fn depth_first_default() {
    let mut maze = Maze::default().seed(0);
    maze.depth_first();
    maze.draw(Path::new("tests/output/DepthFirst_default.png")).unwrap();
}

#[test]
fn depth_first_0x1() {
    let maze = Maze::new(0, 1);
    match maze {
        Ok(_) => panic!("Maze should not be created"),
        Err(MazeError::WidthIsZero) => (),
        Err(_) => panic!("Wrong error"),
    }
}

#[test]
fn depth_first_1x0() {
    let maze = Maze::new(1, 0);
    match maze {
        Ok(_) => panic!("Maze should not be created"),
        Err(MazeError::HeightIsZero) => (),
        Err(_) => panic!("Wrong error"),
    }
}

#[test]
fn depth_first_1x1() {
    let mut maze = Maze::new(1, 1).unwrap().seed(0);
    maze.depth_first();
    maze.draw(Path::new("tests/output/DepthFirst_1x1.png")).unwrap();
}

#[test]
fn depth_first_1x3() {
    let mut maze = Maze::new(1, 3).unwrap().seed(0);
    maze.depth_first();
    maze.draw(Path::new("tests/output/DepthFirst_1x3.png")).unwrap();
}

#[test]
fn depth_first_4x21() {
    let mut maze = Maze::new(4, 21).unwrap().seed(0);
    maze.depth_first();
    maze.draw(Path::new("tests/output/DepthFirst_4x21.png")).unwrap();
}

#[test]
fn depth_first_2047x1023() {
    let mut maze = Maze::new(2047, 1023).unwrap().seed(0);
    maze.depth_first();
    maze.draw(Path::new("tests/output/DepthFirst_2047x1023.png")).unwrap();
}