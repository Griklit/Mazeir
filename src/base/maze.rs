use std::fmt::{Debug, Display, Formatter};
use std::default::Default;

use rand::SeedableRng;
use rand::rngs::StdRng;
use rand::prelude::random;

use super::errors::MazeError;
use super::enumerates::{GeneratorType, OutputType};
use super::super::arithmetic::*;
use super::super::output::*;


pub struct Maze {
    pub(crate) raw_width: usize,
    pub(crate) raw_height: usize,
    pub width: usize,
    pub height: usize,
    pub map: Vec<Vec<bool>>,
    pub(crate) rng: StdRng,
}

impl Maze {
    pub fn new(width: usize, height: usize) -> Result<Self, MazeError> {
        if width == 0 || height == 0 { return Err(MazeError::SizeIsZero); }
        let fix_width = if width % 2 == 0 { width - 1 } else { width };
        let fix_height = if height % 2 == 0 { height - 1 } else { height };
        let maze = Maze {
            raw_width: width,
            raw_height: height,
            width: fix_width,
            height: fix_height,
            map: vec![vec![false; fix_width]; fix_height],
            rng: SeedableRng::seed_from_u64(random()),
        };
        Ok(maze)
    }

    pub fn seed(&mut self, seed: u64) -> &mut Self {
        self.rng = SeedableRng::seed_from_u64(seed);
        self
    }

    pub fn center_point(&self) -> (Option<usize>, Option<usize>) {
        let x = if self.width > 2 {
            Some(if self.width / 2 % 2 == 0 { self.width / 2 - 1 } else { self.width / 2 })
        } else { None };
        let y = if self.height > 2 {
            Some(if self.height / 2 % 2 == 0 { self.height / 2 - 1 } else { self.height / 2 })
        } else { None };
        (x, y)
    }

    pub fn build(&mut self, generator_type: GeneratorType) -> &mut Self {
        match generator_type {
            GeneratorType::DepthFirst => self.depth_first()
        }
        self
    }

    pub fn output(&self, output_type: OutputType) -> Result<&Self, MazeError> {
        match output_type {
            OutputType::Image(path) => self.draw(&path)?,
            OutputType::Text(path) => self.write(&path)?,
            OutputType::Stdout => self.print()?,
        };
        Ok(self)
    }
}

impl Default for Maze {
    fn default() -> Self {
        Maze::new(127, 127).unwrap()
    }
}

impl Debug for Maze {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Maze {{raw_width: {}, raw_height: {}, width: {}, height: {}}}",
               self.raw_width, self.raw_height, self.width, self.height)
    }
}

impl Display for Maze {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Maze({}x{})", self.raw_width, self.raw_height)
    }
}

