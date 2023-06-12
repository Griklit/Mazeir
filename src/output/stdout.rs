use super::super::base::{Maze, MazeError};

pub trait Stdout {
    fn print(&self) -> Result<(), MazeError>;
}

impl Stdout for Maze {
    fn print(&self) -> Result<(), MazeError> {
        for line in &self.map {
            let mut s = String::new();
            for cell in line {
                s.push_str(if *cell { "   " } else { "\x1b[40m   \x1b[0m" });
            }
            println!("{}", s);
        }
        Ok(())
    }
}
