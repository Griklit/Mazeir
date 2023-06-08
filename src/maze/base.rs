use std::fmt::{Debug, Display, Formatter};
use std::default::Default;
use std::path::Path;

use rand::{SeedableRng, Rng};
use rand::rngs::StdRng;
use rand::prelude::{random, SliceRandom};
use image::{GrayImage, ImageBuffer, Luma, ImageError};

use super::errors::MazeError;


pub struct Maze {
    raw_width: usize,
    raw_height: usize,
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

    pub fn draw(&self, path: &Path) -> Result<(), ImageError> {
        let mut img: GrayImage = ImageBuffer::new(self.raw_width as u32, self.raw_height as u32);
        let pixel_white = Luma([255]);
        let pixel_black = Luma([0]);
        for y in 0..self.height {
            for x in 0..self.width {
                img.put_pixel(x as u32, y as u32, if self.map[y][x] { pixel_white } else { pixel_black })
            }
        }
        img.save(path)?;
        Ok(())
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

pub enum Direction {
    Left,
    Right,
    Top,
    Bottom,
}

impl Direction {
    pub fn to_vector(&self) -> (i8, i8) {
        match self {
            Direction::Left => { (-1, 0) }
            Direction::Right => { (1, 0) }
            Direction::Top => { (0, -1) }
            Direction::Bottom => { (0, 1) }
        }
    }
}

pub struct Around {
    around: Vec<Direction>,
}

impl Around {
    pub fn from_map(map: &Vec<Vec<bool>>, x: usize, y: usize) -> Self {
        let mut around: Vec<Direction> = vec![];
        if x > 2 && !map[y][x - 2] { around.push(Direction::Left); }// 左
        if x + 2 < map[0].len() && !map[y][x + 2] { around.push(Direction::Right); }// 右
        if y > 2 && !map[y - 2][x] { around.push(Direction::Top); }// 上
        if y + 2 < map.len() && !map[y + 2][x] { around.push(Direction::Bottom); }// 下
        Self { around }
    }
    pub fn choose<R>(&self, rng: &mut R) -> Option<&Direction>
        where R: Rng + ?Sized {
        self.around.choose(rng).map(|x| x)
    }
}