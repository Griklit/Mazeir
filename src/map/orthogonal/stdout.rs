use super::Orthogonal;

use crate::output::Stdout;

const SPACE: &str = "   ";
const WALL: &str = "\x1b[40m   \x1b[0m";

impl Stdout for Orthogonal {
    fn print(&self) {
        for _ in 0..self.width * 2 + 1 { print!("{}", WALL); }
        println!();
        for i in 0..self.height {
            let line = &self.map[self.width * i..self.width * (i + 1)];
            print!("{}", WALL);
            for v in line.iter() {
                print!("{}", SPACE);
                print!("{}", if *v & 0b0000_0010 == 0 { WALL } else { SPACE })
            }
            println!();
            print!("{}", WALL);
            for v in line.iter() {
                print!("{}", if *v & 0b0000_0001 == 0 { WALL } else { SPACE });
                print!("{}", WALL)
            }
            println!();
        }
    }
}