extern crate mazeir;

use std::fs;
use std::io::Read;
use std::path::{Path, PathBuf};
use mazeir::{base::*, arithmetic::*, output::*};


fn compare_snapshot<P: AsRef<Path>>(file_name: P) {
    let output_base: PathBuf = PathBuf::from("tests/output");
    let snapshot_base: PathBuf = PathBuf::from("tests/snapshot");
    let output_path = output_base.join(&file_name);
    let snapshot_path = snapshot_base.join(&file_name);
    if !output_path.exists() {
        panic!("Output file not found");
    }
    if !snapshot_path.exists() {
        fs::create_dir_all(snapshot_base).unwrap();
        fs::copy(&output_path, &snapshot_path).unwrap();
        return;
    }
    if output_path.metadata().unwrap().len() != snapshot_path.metadata().unwrap().len() {
        panic!("File size not match")
    }
    const BLOCK_SIZE: usize = 1024;
    let mut output_file = fs::File::open(&output_path).unwrap();
    let mut snapshot_file = fs::File::open(&snapshot_path).unwrap();
    let mut output_buf = [0u8; BLOCK_SIZE];
    let mut snapshot_buf = [0u8; BLOCK_SIZE];
    loop {
        let output_len = output_file.read(&mut output_buf).unwrap();
        snapshot_file.read(&mut snapshot_buf).unwrap();
        assert_eq!(output_buf, snapshot_buf);
        if output_len < BLOCK_SIZE { break; }
    }
}

#[test]
fn depth_first_default() {
    let mut maze = Maze::default();
    maze.seed(0).depth_first();
    maze.draw(Path::new("tests/output/DepthFirst_default.png")).unwrap();
}


#[test]
fn zero_size() {
    let maze = Maze::new(0, 1);
    match maze {
        Ok(_) => panic!("Maze should not be created"),
        Err(MazeError::SizeIsZero) => (),
        Err(_) => panic!("Wrong error"),
    };
    let maze = Maze::new(1, 0);
    match maze {
        Ok(_) => panic!("Maze should not be created"),
        Err(MazeError::SizeIsZero) => (),
        Err(_) => panic!("Wrong error"),
    };
}


#[test]
fn depth_first_3x1() {
    let file_name = Path::new("DepthFirst_3x1.png");
    let mut maze = Maze::new(3, 1).unwrap();
    maze.seed(0).depth_first();
    maze.draw(Path::new("tests/output/").join(file_name)).unwrap();
    compare_snapshot(file_name);
}


#[test]
fn depth_first_4x21() {
    let file_name = Path::new("DepthFirst_4x21.png");
    let mut maze = Maze::new(4, 21).unwrap();
    maze.seed(0).depth_first();
    maze.draw(Path::new("tests/output/").join(file_name)).unwrap();
    compare_snapshot(file_name);
}

#[test]
fn depth_first_1024x16() {
    let file_name = Path::new("DepthFirst_1024x16.png");
    let mut maze = Maze::new(1024, 16).unwrap();
    maze.seed(0).depth_first();
    maze.draw(Path::new("tests/output/").join(file_name)).unwrap();
    compare_snapshot(file_name);
}


#[test]
fn depth_first_2047x1023() {
    let file_name = Path::new("DepthFirst_2047x1023.png");
    let mut maze = Maze::new(2047, 1023).unwrap();
    maze.seed(0).depth_first();
    maze.draw(Path::new("tests/output/").join(file_name)).unwrap();
    compare_snapshot(file_name);
}


#[test]
fn output_text() {
    let file_name = Path::new("DepthFirst_default.txt");
    let mut maze = Maze::default();
    maze.seed(0).depth_first();
    maze.write(Path::new("tests/output/").join(file_name)).unwrap();
    compare_snapshot(file_name);
}

#[test]
fn output_stdout() {
    let mut maze = Maze::default();
    maze.seed(0).depth_first();
    maze.print().unwrap();
}