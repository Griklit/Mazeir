use mazeir::Orthogonal;
use mazeir::arithmetic::DepthFirst;
use mazeir::output::Stdout;

fn main() {
    let mut map = Orthogonal::new(10, 7);
    map.depth_first(None);
    map.print();
}