pub enum Direction {
    Left,
    Right,
    Top,
    Bottom,
}

impl Direction {
    pub fn to_vector(&self) -> (i8, i8) {
        match self {
            Direction::Left => { (-1, 0) }
            Direction::Right => { (1, 0) }
            Direction::Top => { (0, -1) }
            Direction::Bottom => { (0, 1) }
        }
    }
}
