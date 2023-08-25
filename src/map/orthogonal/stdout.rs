use super::{Orthogonal, LEFT_WALL, UP_WALL};

use crate::output::Print;
use colored::Colorize;


impl Print for Orthogonal {
    fn print(&self) {
        let space = "  ".on_bright_white();
        let wall = "  ".on_truecolor(0, 0, 0);
        for i in 0..self.height {
            let line = &self.map[self.width * i..self.width * (i + 1)];
            for v in line.iter() {
                print!("{}", &wall);
                print!("{}", if *v & UP_WALL == 0 { &wall } else { &space });
            }
            println!("{}", &wall);
            for v in line.iter() {
                print!("{}", if *v & LEFT_WALL == 0 { &wall } else { &space });
                print!("{}", &space);
            }
            println!("{}", &wall);
        }
        for _ in 0..self.width * 2 + 1 { print!("{}", &wall); }
    }
}