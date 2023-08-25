use std::fs;
use std::path::PathBuf;

use mazeir::Orthogonal;
use mazeir::arithmetic::DepthFirst;
use mazeir::output::Draw;

use crate::assert_func::*;


fn get_path(arithmetic: &str, width: usize, height: usize) -> (PathBuf, PathBuf) {
    let snapshot_path = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join(format!("tests/orthogonal/snapshot/{}_{}x{}.png", arithmetic, width, height));
    let output_path = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join(format!("tests/orthogonal/output/{}_{}x{}.png", arithmetic, width, height));
    (snapshot_path, output_path)
}

#[test]
fn depth_first_0x0() {
    let mut map = Orthogonal::new(0, 0);
    let (snapshot_path, output_path) = get_path("depth-first", 0, 0);
    map.depth_first_with_str_seed("0x0");
    map.draw(fs::File::create(&output_path).unwrap()).unwrap();
    assert_files_eq(&output_path, &snapshot_path);
}

#[test]
fn depth_first_2x3() {
    let mut map = Orthogonal::new(2, 3);
    let (snapshot_path, output_path) = get_path("depth-first", 2, 3);
    map.depth_first_with_str_seed("2x3");
    map.draw(fs::File::create(&output_path).unwrap()).unwrap();
    assert_files_eq(&output_path, &snapshot_path);
}

#[test]
fn depth_first_1076x593() {
    let mut map = Orthogonal::new(1076, 593);
    let (snapshot_path, output_path) = get_path("depth-first", 1076, 593);
    map.depth_first_with_str_seed("1076x593");
    map.draw(fs::File::create(&output_path).unwrap()).unwrap();
    assert_files_eq(&output_path, &snapshot_path);
}