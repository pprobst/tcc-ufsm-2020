use super::{common::count_neighbor_tile, Map, TileType};
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
    fn get_wall(&self, map: &Map, dir: Direction) -> Point;
    fn get_borders(&self, map: &Map) -> Vec<Point>;
}

pub type Room = Rect;

impl Operations for Room {
    fn get_wall(&self, map: &Map, dir: Direction) -> Point {
        let borders = self.get_borders(map);
        let mut rng = RandomNumberGenerator::new();
        let mut x = 0;
        let mut y = 0;

        for pt in borders {
            match dir {
                EAST => {
                    if pt.x == self.x2 - 1 {
                        y = rng.range(self.y1 + 1, self.y2 - 1);
                        return Point::new(pt.x, y);
                    }
                }
                WEST => {
                    if pt.x == self.x1 + 1 {
                        y = rng.range(self.y1 + 1, self.y2 - 1);
                        return Point::new(pt.x, y);
                    }
                }
                NORTH => {
                    if pt.y == self.y1 + 1 {
                        x = rng.range(self.x1 + 1, self.x2 - 1);
                        return Point::new(x, pt.y);
                    }
                }
                _ => {
                    if pt.y == self.y2 - 1 {
                        x = rng.range(self.x1 + 1, self.x2 - 1);
                        return Point::new(x, pt.y);
                    }
                }
            }
        }
        Point::new(0, 0)
    }

    fn get_borders(&self, map: &Map) -> Vec<Point> {
        let mut borders: Vec<Point> = Vec::new();
        for x in (self.x1 + 1)..self.x2 {
            for y in (self.y1 + 1)..self.y2 {
                let pt = Point::new(x, y);
                let wall_counter = count_neighbor_tile(map, pt, TileType::Wall, false);
                if wall_counter >= 1 {
                    borders.push(pt);
                }
            }
        }
        borders
    }
}
