use super::{Map, Point, Tile, TileType};
use crate::utils::directions::*;
use std::collections::{HashMap, HashSet};

#[allow(dead_code)]
pub struct WaveFunctionCollapse {
    tile_size: i32,
    patterns: Vec<Vec<TileType>>,
    frequencies: HashMap<TileType, u32>,
}

#[allow(dead_code)]
impl WaveFunctionCollapse {
    pub fn new(tile_size: i32) -> Self {
        Self {
            tile_size,
            patterns: Vec::new(),
            frequencies: HashMap::new(),
        }
    }

    pub fn generate(&mut self, map: &mut Map) {
        self.build_patterns(map);
        self.compute_frequencies();
        /*
        for tile in map.tiles.iter_mut() {
            *tile = Tile::floor();
        }
        */
    }

    /// Build tiles of size tile_size x tile_size from map cells.
    fn build_patterns(&mut self, map: &mut Map) {
        // Navigate the coordinates of each tile.
        for ty in 0..(map.height / self.tile_size) {
            for tx in 0..(map.width / self.tile_size) {
                let start = Point::new(tx * self.tile_size, ty * self.tile_size);
                let end = Point::new((tx + 1) * self.tile_size, (ty + 1) * self.tile_size);
                /*
                 * Example (considering the first tile):
                 * > tile_size = 6
                 *
                 * |--> x1 = 0 * 6 = 0
                 * |
                 * #######--> x2 = (0+1) * 6 = 6
                 * #.....#
                 * #.....#
                 * #.....#
                 * #.....#
                 * #.....#
                 * #######--> y2 = (0+1) * 6 = 6
                 * |
                 * |--> y1 = 0 * 6 = 0
                 *
                 * For the second tile, 0 will be 1.
                 * For the third tile, 1 will be 2.
                 * And so on...
                 * That is, the x2 and y2 of the current tile will be
                 * the x1 and y1 of the next tile.
                 *
                 */
                let normal_pattern = self.get_pattern(map, start, end, "normal");
                let vert_pattern = self.get_pattern(map, start, end, "vertical");
                let horiz_pattern = self.get_pattern(map, start, end, "horizontal");
                let verthoriz_pattern = self.get_pattern(map, start, end, "both");
                self.patterns.push(normal_pattern);
                self.patterns.push(vert_pattern);
                self.patterns.push(horiz_pattern);
                self.patterns.push(verthoriz_pattern);
            }
        }

        // Remove identical pattern.
        deduplicate(&mut self.patterns);
    }

    fn compute_frequencies(&mut self) {
        for pattern in self.patterns.iter() {
            for ttype in pattern.iter() {
                *self.frequencies.entry(*ttype).or_insert(0) += 1;
            }
        }
    }

    fn build_adjacency_rules(&mut self) {}

    fn get_pattern(&mut self, map: &mut Map, start: Point, end: Point, rot: &str) -> Vec<TileType> {
        let mut pattern: Vec<TileType> = Vec::new();
        for y in start.y..end.y {
            for x in start.x..end.x {
                let idx;
                match rot {
                    "vertical" => {
                        idx = map.idx(x, end.y - (y + 1));
                    }
                    "horizontal" => {
                        idx = map.idx(end.x - (x + 1), y);
                    }
                    "both" => {
                        idx = map.idx(end.x - (x + 1), end.y - (y + 1));
                    }
                    _ => {
                        idx = map.idx(x, y);
                    }
                }
                pattern.push(map.tiles[idx].ttype);
            }
        }
        pattern
    }
}

// https://www.reddit.com/r/rust/comments/38zzbk/best_way_to_remove_duplicates_from_a_veclist/
fn deduplicate(vs: &mut Vec<Vec<TileType>>) {
    let set: HashSet<Vec<TileType>> = vs.drain(..).collect();
    vs.extend(set.into_iter());
}
