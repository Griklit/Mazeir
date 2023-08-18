enum Wall {
    Left,
    Right,
    Up,
    Down,
}

/// Orthogonal map
///
/// Cell:
///
/// | 0 | 1 | 2 | 3 | 4-5 | 6 | 7 |
/// |:-:|:-:|:-:|:-:|:-----:|:-:|:-:|
/// |   |   |   |   | last cell direction | right wall | down wall |
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

    pub fn get(&self, x: usize, y: usize) -> u8 {
        self.map[y * self.width + x]
    }

    pub fn set(&mut self, x: usize, y: usize, value: u8) {
        self.map[y * self.width + x] = value;
    }

    pub fn get_walls(&self, x: usize, y: usize) -> Vec<Wall> {
        let mut walls = Vec::with_capacity(4);
        if x == 0 || self.get(x - 1, y) & 0b0000_0010 == 0 { walls.push(Wall::Left); }
        if x == self.width - 1 || self.get(x, y) & 0b0000_0010 == 0 { walls.push(Wall::Right); }
        if y == 0 || self.get(x, y - 1) & 0b0000_0001 == 0 { walls.push(Wall::Up); }
        if y == self.height - 1 || self.get(x, y) & 0b0000_0001 == 0 { walls.push(Wall::Down); }
        walls
    }

    pub fn break_wall(&mut self, x: usize, y: usize, wall: Wall) {
        match wall {
            Wall::Up => self.map[(y * self.width - 1) + x] |= 0b0000_0001,
            Wall::Down => self.map[(y * self.width) + x] |= 0b0000_0001,
            Wall::Left => self.map[y * self.width + x - 1] |= 0b0000_0010,
            Wall::Right => self.map[y * self.width + x] |= 0b0000_0010,
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