use parry2d::bounding_volume::{Aabb, BoundingVolume};


pub struct Collision {
    pub obj1 : Aabb,
    pub obj2 : Aabb,
}

impl Collision {
    pub fn intersect(&self) -> bool {
        self.obj1.intersects(&self.obj2)
    }
}