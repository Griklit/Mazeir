use mazeir::Orthogonal;
use mazeir::arithmetic::DepthFirst;
use mazeir::output::{Print, Draw};

fn main() {
    let mut map = Orthogonal::new(2_i32.pow(10) as usize, 2_i32.pow(11) as usize);
    map.depth_first(None);
    // map.print();
    let file = std::fs::File::create("test.png").unwrap();
    map.draw(file).unwrap();
}