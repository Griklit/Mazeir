use super::super::{Maze, Around};


pub trait DepthFirstBuild {
    fn depth_first(&mut self);
}

impl DepthFirstBuild for Maze {
    fn depth_first(&mut self) {
        let start_point = self.center_point();
        if start_point.0.is_none() || start_point.1.is_none() { return; }
        let start_point = (start_point.0.unwrap(), start_point.1.unwrap());
        self.map[start_point.1][start_point.0] = true;
        let mut route = vec![start_point, start_point];
        while let Some((x, y)) = route.pop() {
            let (mut x, mut y) = (x as i64, y as i64);
            loop {
                let around = Around::from_map(&self.map, x as usize, y as usize);
                let r = around.choose(&mut self.rng);
                if r.is_none() { break; }
                let vector = r.unwrap().to_vector();
                let vector = (vector.0 as i64, vector.1 as i64);
                self.map[(y + vector.1) as usize][(x + vector.0) as usize] = true;
                x += vector.0 * 2;
                y += vector.1 * 2;
                self.map[y as usize][x as usize] = true;
                route.push((x as usize, y as usize));
            }
        }
    }
}