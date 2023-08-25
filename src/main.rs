use mazeir::Orthogonal;
use mazeir::arithmetic::DepthFirst;
use mazeir::output::{Print};

fn main() {
    let mut map = Orthogonal::new(2_i32.pow(1) as usize, 2_i32.pow(1) as usize);
    map.depth_first_with_str_seed("Hello, World");
    map.print();
    // let file = std::fs::File::create("test.png").unwrap();
    // map.draw(file).unwrap();
}