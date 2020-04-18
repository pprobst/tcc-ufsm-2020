use crate::utils::directions::*;
use bracket_lib::prelude::{Point, RandomNumberGenerator, Rect};

/* room.rs
 * -------
 * Simply defines a room as a bracket_lib rectangle (Rect);
 *
 * Available functions:
 *
 *   pub fn with_size<T>(x: T, y: T, w: T, h: T) -> Rect where
 *       T: TryInto<i32>,
 *   pub fn with_exact<T>(x1: T, y1: T, x2: T, y2: T) -> Rect where
 *       T: TryInto<i32>,
 *   pub fn zero() -> Rect
 *   pub fn intersect(&self, other: &Rect) -> bool
 *   pub fn center(&self) -> Point
 *   pub fn point_in_rect(&self, point: Point) -> bool
 *   pub fn for_each<F>(&self, f: F) where
 *       F: FnMut(Point),
 *   pub fn point_set(&self) -> HashSet<Point, RandomState>
 *   pub fn width(&self) -> i32
 *   pub fn height(&self) -> i32
 *
 */

pub trait Operations {
    fn get_wall(&self, dir: Direction) -> Point;
}

pub type Room = Rect;

impl Operations for Room {
    fn get_wall(&self, dir: Direction) -> Point {
        let mut cx = self.center().x;
        let mut cy = self.center().y;
        let w = self.width() / 2;
        let h = self.height() / 2;

        match dir {
            EAST => {
                cx += h;
            }
            WEST => {
                cx -= w;
            }
            NORTH => {
                cy -= h;
            }
            _ => {
                cy += h;
            }
        }

        Point::new(cx, cy)
    }
}
