use rand::SeedableRng;
use rand::seq::SliceRandom;
use rand_xorshift::XorShiftRng;

use crate::arithmetic::*;

#[derive(Debug, Clone, Copy)]
enum Direction { Left, Right, Up, Down }

/// Orthogonal map
///
/// Cell:
///
/// | 0-5 | 6 | 7 |
/// |:---:|:-:|:-:|
/// | Algorithm customization | Right wall | Down wall |
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

    pub fn get(&self, x: usize, y: usize) -> u8 {
        self.map[y * self.width + x]
    }

    pub fn set(&mut self, x: usize, y: usize, value: u8) {
        self.map[y * self.width + x] = value;
    }

    ///获取周围可破坏墙面
    pub fn get_walls(&self, x: usize, y: usize) -> Vec<Direction> {
        let mut walls = Vec::with_capacity(4);
        self.add_walls_to_vec(x, y, &mut walls);
        walls
    }

    ///不重新分配内存，直接在vec上操作
    pub fn add_walls_to_vec(&self, x: usize, y: usize, vec: &mut Vec<Direction>) {
        if x != 0 && self.get(x - 1, y) & 0b0000_0010 == 0 { vec.push(Direction::Left); }
        if x != self.width - 1 && self.get(x, y) & 0b0000_0010 == 0 { vec.push(Direction::Right); }
        if y != 0 && self.get(x, y - 1) & 0b0000_0001 == 0 { vec.push(Direction::Up); }
        if y != self.height - 1 && self.get(x, y) & 0b0000_0001 == 0 { vec.push(Direction::Down); }
    }
    ///应判断是否越界，但从性能角度考虑，交给生成算法判断
    pub fn break_wall(&mut self, x: usize, y: usize, wall: &Direction) {
        match wall {
            Direction::Up => self.map[(y * self.width - 1) + x] |= 0b0000_0001,
            Direction::Down => self.map[(y * self.width) + x] |= 0b0000_0001,
            Direction::Left => self.map[y * self.width + x - 1] |= 0b0000_0010,
            Direction::Right => self.map[y * self.width + x] |= 0b0000_0010,
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

mod last_direction {
    pub const UP: u8 = 0b0000_0000;
    pub const DOWN: u8 = 0b0000_0100;
    pub const LEFT: u8 = 0b0000_1000;
    pub const RIGHT: u8 = 0b0000_1100;
}

impl DepthFirst for Orthogonal {
    fn depth_first(&mut self, seed: Option<[u8; 16]>) {
        const LAST_DIRECTION_MASK: u8 = !0b0000_1100;
        let mut rng = if let Some(seed) = seed { XorShiftRng::from_seed(seed) } else { XorShiftRng::from_entropy() };
        let start_point = self.center_point();
        self.set(start_point.0, start_point.1, 0b0000_0001);
        let (mut x, mut y) = start_point;
        let mut walls = Vec::with_capacity(4);
        loop {
            walls.clear();
            self.add_walls_to_vec(x, y, &mut walls);
            let wall = walls.choose(&mut rng);
            match wall {
                Some(wall) => {
                    self.break_wall(x, y, wall);
                    match wall {
                        Direction::Up => {
                            y -= 1;
                            self.map[y * self.width + x] &= LAST_DIRECTION_MASK;
                            self.map[y * self.width + x] |= last_direction::DOWN;
                        }
                        Direction::Down => {
                            y += 1;
                            self.map[y * self.width + x] &= LAST_DIRECTION_MASK;
                            self.map[y * self.width + x] |= last_direction::UP;
                        }
                        Direction::Left => {
                            x -= 1;
                            self.map[y * self.width + x] &= LAST_DIRECTION_MASK;
                            self.map[y * self.width + x] |= last_direction::RIGHT;
                        }
                        Direction::Right => {
                            x += 1;
                            self.map[y * self.width + x] &= LAST_DIRECTION_MASK;
                            self.map[y * self.width + x] |= last_direction::LEFT;
                        }
                    }
                }
                None => {
                    let point_value = self.get(x, y);
                    match point_value & 0b00001100 {
                        last_direction::UP => { y -= 1; }
                        last_direction::DOWN => { y += 1; }
                        last_direction::LEFT => { x -= 1; }
                        last_direction::RIGHT => { y += 1; }
                        _ => {}//不会发生
                    }
                }
            }
        }
    }
}
