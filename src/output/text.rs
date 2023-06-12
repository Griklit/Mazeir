use std::io::Write;
use std::path::Path;

use super::super::base::{Maze, MazeError};

pub trait Text {
    fn write(&self, path: &Path) -> Result<(), MazeError>;
}

impl Text for Maze {
    fn write(&self, path: &Path) -> Result<(), MazeError> {
        let mut f = std::fs::File::create(path).map_err(|e| MazeError::OutputError(e.to_string()))?;
        for line in &self.map {
            let mut buff = line.iter().map(|cell| if *cell { b' ' } else { b'#' }).collect::<Vec<u8>>();
            buff.push(b'\n');
            f.write(&buff).map_err(|e| MazeError::OutputError(e.to_string()))?;
        }
        Ok(())
    }
}