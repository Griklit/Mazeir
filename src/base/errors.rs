#[derive(Debug)]
pub enum MazeError {
    WidthIsZero,
    HeightIsZero,
    Unsupported(String),
    Unknown,
}