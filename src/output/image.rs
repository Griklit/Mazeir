use std::path::Path;

use image::{GrayImage, ImageBuffer, Luma, ImageError};

use super::super::base::{Maze, MazeError};

pub trait Image {
    fn draw(&self, path: &Path) -> Result<(), MazeError>;
}

impl Image for Maze {
    fn draw(&self, path: &Path) -> Result<(), MazeError> {
        let mut img: GrayImage = ImageBuffer::new(self.raw_width as u32, self.raw_height as u32);
        let pixel_white = Luma([255]);
        let pixel_black = Luma([0]);
        for y in 0..self.height {
            for x in 0..self.width {
                img.put_pixel(x as u32, y as u32, if self.map[y][x] { pixel_white } else { pixel_black })
            }
        }
        match img.save(path) {
            Ok(_) => Ok(()),
            Err(e) => return Err(MazeError::OutputError(e.to_string())),
        }
    }
}