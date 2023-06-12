use std::str::FromStr;

use super::errors::MazeError;

pub enum GeneratorType {
    DepthFirst,
}

impl FromStr for GeneratorType {
    type Err = MazeError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_ascii_lowercase().as_str() {
            "depth_first" | "depth" => Ok(Self::DepthFirst),
            _ => Err(MazeError::InvalidGeneratorType),
        }
    }
}

pub enum OutputType {
    Image(Path),
    Stdout,
    Text(Path),
}

impl FromStr for OutputType {
    type Err = MazeError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_ascii_lowercase().as_str() {
            "image" | "pic" | "picture" => Ok(Self::Image()),
            "stdout" | "print" => Ok(Self::Stdout),
            "text" | "txt" => Ok(Self::Text),
            _ => Err(MazeError::InvalidOutputType),
        }
    }
}