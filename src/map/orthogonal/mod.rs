mod depth_first;
mod stdout;
mod image;

use super::Direction2D;

pub const LEFT_WALL: u8 = 0b0000_0010;
pub const UP_WALL: u8 = 0b0000_0001;

/// Cell:
///
/// | flag | description |
/// |:-----|:------------|
/// | 0-5  | Algorithm customization |
/// | 6    | Left wall |
/// | 7    | Up wall |

pub struct Orthogonal {
    width: usize,
    height: usize,
    map: Vec<u8>,
}

impl Orthogonal {
    pub fn new(width: usize, height: usize) -> Self {
        Orthogonal {
            width,
            height,
            map: vec![0; width * height],
        }
    }
    pub fn center_point(&self) -> (usize, usize) {
        (self.width / 2, self.height / 2)
    }

    pub fn get(&mut self, x: usize, y: usize) -> &mut u8 {
        &mut self.map[y * self.width + x]
    }

    pub fn set(&mut self, x: usize, y: usize, value: u8) {
        self.map[y * self.width + x] = value;
    }

    ///应判断是否越界，但从性能角度考虑，交给生成算法判断
    pub fn break_wall(&mut self, x: usize, y: usize, wall: &Direction2D) {
        match wall {
            Direction2D::Left => self.map[y * self.width + x] |= LEFT_WALL,
            Direction2D::Right => self.map[y * self.width + x + 1] |= LEFT_WALL,
            Direction2D::Up => self.map[y * self.width + x] |= UP_WALL,
            Direction2D::Down => self.map[(y + 1) * self.width + x] |= UP_WALL,
        }
    }
}

impl Default for Orthogonal {
    fn default() -> Self {
        Orthogonal {
            width: 32,
            height: 32,
            map: vec![0; 32 * 32],
        }
    }
}

