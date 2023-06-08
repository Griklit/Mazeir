use std::fmt::{Debug, Display, Formatter};
use std::default::Default;
use std::path::Path;

use rand::{SeedableRng, Rng};
use rand::rngs::StdRng;
use rand::prelude::{random, SliceRandom};
use image::{GrayImage, ImageBuffer, Luma, ImageError};

use super::errors::MazeError;


pub struct Maze {
    pub(crate) raw_width: usize,
    pub(crate) raw_height: usize,
    pub width: usize,
    pub height: usize,
    pub map: Vec<Vec<bool>>,
    pub rng: StdRng,
}

impl Maze {
    pub fn new(width: usize, height: usize) -> Result<Self, MazeError> {
        if width == 0 {
            return Err(MazeError::WidthIsZero);
        }
        if height == 0 {
            return Err(MazeError::HeightIsZero);
        }
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

    pub fn seed(mut self, seed: u64) -> Self {
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
}

impl Default for Maze {
    fn default() -> Self {
        Maze::new(511, 511).unwrap()
    }
}

impl Debug for Maze {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Maze {{{}x{}}}", self.raw_width, self.raw_height)
    }
}

impl Display for Maze {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Maze({}x{})", self.raw_width, self.raw_height)
    }
}

