use super::{Orthogonal, LEFT_WALL, UP_WALL};

use crate::output::Print;

const SPACE: &str = "   ";
const WALL: &str = "\x1b[40m   \x1b[0m";

impl Print for Orthogonal {
    fn print(&self) {
        for i in 0..self.height {
            let line = &self.map[self.width * i..self.width * (i + 1)];
            for v in line.iter() {
                print!("{}", WALL);
                print!("{}", if *v & UP_WALL == 0 { WALL } else { SPACE });
            }
            println!("{}", WALL);
            for v in line.iter() {
                print!("{}", if *v & LEFT_WALL == 0 { WALL } else { SPACE });
                print!("{}", SPACE);
            }
            println!("{}", WALL);
        }
        for _ in 0..self.width * 2 + 1 { print!("{}", WALL); }
    }
}