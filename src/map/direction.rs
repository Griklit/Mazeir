#[derive(Debug, Clone, Copy)]
pub enum Direction2D { Left, Right, Up, Down }

#[derive(Debug, Clone, Copy)]
pub enum Direction3D { Left, Right, Up, Down, Front, Back }

#[derive(Debug, Clone, Copy)]
pub enum Direction {
    TowD(Direction2D),
    ThreeD(Direction3D),
}