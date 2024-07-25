
#[derive(Clone)]
pub struct Point{
    pub x: f32,
    pub y: f32,
}

pub struct BoundaryHit {
    pub left: bool,
    pub right: bool,
    pub top: bool,
    pub bottom: bool,
}

pub struct Direction {
    pub point: Point
}

pub struct Velocity{
    pub point: Point
}