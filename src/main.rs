use mazeir::Orthogonal;
use mazeir::arithmetic::DepthFirst;
use mazeir::output::{Print, Draw};

fn main() {
    let mut map = Orthogonal::new(4, 4);
    map.depth_first(None);
    map.print();
    let file = std::fs::File::create("test.png").unwrap();
    map.draw(file);
}