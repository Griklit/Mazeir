pub mod base;
pub mod depth_first;
mod errors;

pub use base::Maze;
pub use depth_first::DepthFirstBuild;
pub use errors::MazeError;