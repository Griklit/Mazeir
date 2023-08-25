//! Cell bit flags:
//!
//! | flag | description |
//! |:-----|:------------|
//! | 0 | visited |
//! | 1-3 | last direction |
use super::{Orthogonal, Direction2D};

use rand::Rng;
use rand::seq::SliceRandom;

use crate::arithmetic::DepthFirst;

const FLAG: u8 = 0b1000_0000;
const LAST_DIRECTION_MASK: u8 = 0b0111_0000;
const LAST_DIRECTION_MASK_INVERSE: u8 = 0b1000_1111;
const LAST_DIRECTION_NONE: u8 = 0b0000_0000;
const LAST_DIRECTION_LEFT: u8 = 0b0001_0000;
const LAST_DIRECTION_RIGHT: u8 = 0b0010_0000;
const LAST_DIRECTION_UP: u8 = 0b0011_0000;
const LAST_DIRECTION_DOWN: u8 = 0b00100_0000;


impl Orthogonal {
    #[inline]
    fn add_walls_to_vec_with_flag(&mut self, x: usize, y: usize, vec: &mut Vec<Direction2D>) {
        if x != 0 && *self.get_mut(x - 1, y) & FLAG == 0 { vec.push(Direction2D::Left); }
        if x != self.width - 1 && *self.get_mut(x + 1, y) & FLAG == 0 { vec.push(Direction2D::Right); }
        if y != 0 && *self.get_mut(x, y - 1) & FLAG == 0 { vec.push(Direction2D::Up); }
        if y != self.height - 1 && *self.get_mut(x, y + 1) & FLAG == 0 { vec.push(Direction2D::Down); }
    }
}


impl DepthFirst for Orthogonal {
    fn depth_first_with_rng<R: Rng + ?Sized>(&mut self, rng: &mut R) {
        if self.width == 0 || self.height == 0 { return; }
        let mut rng = rng;
        let start_point = self.center_point();
        let (mut x, mut y) = (start_point.0, start_point.1);
        self.set(x, y, FLAG);
        let mut walls = Vec::with_capacity(4);
        loop {
            walls.clear();
            self.add_walls_to_vec_with_flag(x, y, &mut walls);
            let wall = walls.choose(&mut rng);
            match wall {
                Some(wall) => {
                    self.break_wall(x, y, wall);
                    match wall {
                        Direction2D::Left => {
                            x -= 1;
                            self.map[y * self.width + x] &= LAST_DIRECTION_MASK_INVERSE;
                            self.map[y * self.width + x] |= LAST_DIRECTION_RIGHT;
                        }
                        Direction2D::Right => {
                            x += 1;
                            self.map[y * self.width + x] &= LAST_DIRECTION_MASK_INVERSE;
                            self.map[y * self.width + x] |= LAST_DIRECTION_LEFT;
                        }
                        Direction2D::Up => {
                            y -= 1;
                            self.map[y * self.width + x] &= LAST_DIRECTION_MASK_INVERSE;
                            self.map[y * self.width + x] |= LAST_DIRECTION_DOWN;
                        }
                        Direction2D::Down => {
                            y += 1;
                            self.map[y * self.width + x] &= LAST_DIRECTION_MASK_INVERSE;
                            self.map[y * self.width + x] |= LAST_DIRECTION_UP;
                        }
                    }
                    self.map[y * self.width + x] |= FLAG;
                }
                None => {
                    let point_value = self.get_mut(x, y);
                    match *point_value & LAST_DIRECTION_MASK {
                        LAST_DIRECTION_LEFT => { x -= 1; }
                        LAST_DIRECTION_RIGHT => { x += 1; }
                        LAST_DIRECTION_UP => { y -= 1; }
                        LAST_DIRECTION_DOWN => { y += 1; }
                        LAST_DIRECTION_NONE => { break; }
                        _ => {}//不会发生
                    }
                }
            }
        }
    }
}
