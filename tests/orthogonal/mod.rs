use std::fs;
use std::path::PathBuf;

use mazeir::Orthogonal;
use mazeir::algorithm::DepthFirst;
use mazeir::output::Draw;

use crate::assert_func::*;


fn get_path(file_name: &str) -> (PathBuf, PathBuf) {
    let snapshot_path = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join(format!("tests/orthogonal/snapshot/{}.png", file_name));
    let output_path = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join(format!("tests/orthogonal/output/{}.png", file_name));
    (snapshot_path, output_path)
}

#[test]
fn depth_first_default() {
    let mut map = Orthogonal::default();
    let (snapshot_path, output_path) = get_path("depth-first_default");
    map.depth_first_with_str_seed("default");
    map.draw(fs::File::create(&output_path).unwrap()).unwrap();
    assert_files_eq(&output_path, &snapshot_path);
}

#[test]
fn depth_first_0x0() {
    let mut map = Orthogonal::new(0, 0);
    let (snapshot_path, output_path) = get_path("depth-first_0x0");
    map.depth_first_with_str_seed("0x0");
    map.draw(fs::File::create(&output_path).unwrap()).unwrap();
    assert_files_eq(&output_path, &snapshot_path);
}

#[test]
fn depth_first_1x1() {
    let mut map = Orthogonal::new(1, 1);
    let (snapshot_path, output_path) = get_path("depth-first_1x1");
    map.depth_first_with_str_seed("1x1");
    map.draw(fs::File::create(&output_path).unwrap()).unwrap();
    assert_files_eq(&output_path, &snapshot_path);
}

#[test]
fn depth_first_2x3() {
    let mut map = Orthogonal::new(2, 3);
    let (snapshot_path, output_path) = get_path("depth-first_2x3");
    map.depth_first_with_str_seed("2x3");
    map.draw(fs::File::create(&output_path).unwrap()).unwrap();
    assert_files_eq(&output_path, &snapshot_path);
}

#[test]
fn depth_first_1076x593() {
    let mut map = Orthogonal::new(1076, 593);
    let (snapshot_path, output_path) = get_path("depth-first_1076x593");
    map.depth_first_with_str_seed("1076x593");
    map.draw(fs::File::create(&output_path).unwrap()).unwrap();
    assert_files_eq(&output_path, &snapshot_path);
}