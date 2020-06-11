use super::{common::{rect_region, circular_region, region_width, region_height}};
use crate::components::Position;

#[derive(Clone, Debug)]
pub struct CustomRegion {
    pub pos: Vec<Position>,
    pub x1: i32,
    pub x2: i32,
    pub y1: i32,
    pub y2: i32,
    pub width: i32,
    pub height: i32,
    pub size: i32,
}

#[allow(dead_code)]
impl CustomRegion {
    pub fn new_rect(x1: i32, y1: i32, width: i32, height: i32) -> Self {
        let s = width * height;
        Self {
            pos: rect_region(x1, y1, width, height),
            x1,
            x2: x1 + width,
            y1,
            y2: y1 + height,
            width,
            height,
            size: s,
        }
    }

    pub fn new_circ(x1: i32, y1: i32, radius: i32) -> Self {
        let region = circular_region(x1, y1, radius);
        let w = region_width(&region);
        let h = region_height(&region);
        let s = w * h;
        Self {
            pos: region,
            x1,
            x2: x1 + radius,
            y1,
            y2: y1 + radius,
            width: w,
            height: h,
            size: s,
        }
    }

    pub fn in_bounds(&self, p: Position) -> bool {
        p.x > self.x1 && p.x < self.width && p.y > 0 && p.y < self.height
    }

    pub fn get_positions(&self) -> Vec<Position> {
        self.pos.to_vec()
    }

    pub fn get_center(&self) -> Position {
        Position::new((self.x1+self.x2)/2, (self.y1+self.y2)/2)
    }
}
