use rand::{Rng, prelude::SliceRandom};

use super::Direction;

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