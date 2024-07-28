use std::ops::{AddAssign, Mul, Add};

#[derive(Clone)]
pub struct Point{
    pub x: f32,
    pub y: f32,
}

impl Mul<f32> for Point {
    // The multiplication of rational numbers is a closed operation.
    type Output = Point;

    fn mul(mut self, rhs: f32) -> Self::Output {
        self.x = self.x * rhs;
        self.y = self.y * rhs;
        self
    }
}

impl AddAssign for Point {
    fn add_assign(&mut self, rhs: Self) {
        *self = Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        };
    }
}

impl Add for Point {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

#[derive(Clone)]
pub struct Direction {
    pub point: Point
}

