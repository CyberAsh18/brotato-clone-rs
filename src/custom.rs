
#[derive(Clone)]
pub struct Point{
    pub x: f32,
    pub y: f32,
} 

pub struct Camera{
    point: Point,
}

pub struct BoundaryHit {
    pub left: bool,
    pub right: bool,
    pub top: bool,
    pub bottom: bool,
}