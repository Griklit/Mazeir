use std::path::{Path, PathBuf};
use std::str::FromStr;

use super::errors::MazeError;

#[derive(Debug)]
pub enum GeneratorType {
    DepthFirst,
}

impl FromStr for GeneratorType {
    type Err = MazeError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_ascii_lowercase().as_str() {
            "depth_first" | "depth" | "depthfirst" | "depth-first" => Ok(Self::DepthFirst),
            _ => Err(MazeError::InvalidGeneratorType(s.to_string())),
        }
    }
}

pub enum OutputType {
    Image(PathBuf),
    Stdout,
    Text(PathBuf),
}

// impl From<> for OutputType{
//     fn from(path: Path) -> Self {
//         Self::Image(path)
//     }
// }
//
// impl FromStr for OutputType {
//     type Err = MazeError;
//
//     fn from_str(s: &str) -> Result<Self, Self::Err> {
//         match s.to_ascii_lowercase().as_str() {
//             "image" | "pic" | "picture" => Ok(Self::Image()),
//             "stdout" | "print" => Ok(Self::Stdout),
//             "text" | "txt" => Ok(Self::Text),
//             _ => Err(MazeError::InvalidOutputType),
//         }
//     }
// }