mod depth_first;
mod stdout;

const RIGHT_WALL: u8 = 0b0000_0010;
const DOWN_WALL: u8 = 0b0000_0001;

#[derive(Debug, Clone, Copy)]
pub enum Direction { Left, Right, Up, Down }

/// Cell:
///
/// | flag | description |
/// |:-----|:------------|
/// | 0-5  | Algorithm customization |
/// | 6    | Right wall |
/// | 7    | Down wall |

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
    pub fn break_wall(&mut self, x: usize, y: usize, wall: &Direction) {
        match wall {
            Direction::Left => self.map[y * self.width + x - 1] |= RIGHT_WALL,
            Direction::Right => self.map[y * self.width + x] |= RIGHT_WALL,
            Direction::Up => self.map[(y - 1) * self.width + x] |= DOWN_WALL,
            Direction::Down => self.map[y * self.width + x] |= DOWN_WALL,
        }
    }
}

impl Default for Orthogonal {
    fn default() -> Self {
        Orthogonal {
            width: 64,
            height: 64,
            map: vec![0; 64 * 64],
        }
    }
}

