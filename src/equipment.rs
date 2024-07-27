use crate::custom::Point;

pub struct Gun {
    //sprite_sheet: Texture2D,
    origin: Point
}

impl Gun {
    pub fn initialize() -> Gun {
        return Gun {
            origin: Point {
                x: 0.0,
                y: 0.0,
            }
        }
    }

    pub fn draw() {
        
    }
}