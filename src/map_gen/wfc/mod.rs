use super::{Map, Point, Tile, TileType};
use crate::utils::directions::*;
use std::collections::{HashMap, HashSet};

pub struct Cell {
    patterns: Vec<MapTile>,
}

impl Cell {
    pub fn new(patterns: Vec<MapTile>) -> Self {
        Self { patterns }
    }
}

pub struct Wave {
    cells: Vec<Cell>,
    uncollapsed_cells: usize,
}

impl Wave {
    pub fn new(cells: Vec<Cell>) -> Self {
        let cells_len = cells.len();
        Self {
            cells,
            uncollapsed_cells: cells_len,
        }
    }
}

#[derive(Debug, Clone)]
pub struct MapTile {
    pattern: Vec<TileType>,
    compatible: Vec<(Vec<TileType>, Direction)>, // overlaps
    size: i32,
}

pub fn tile_idx(tile_size: i32, x: i32, y: i32) -> usize {
    ((y * tile_size) + x) as usize
}

pub fn in_tile_bounds(tile_size: i32, x: i32, y: i32) -> bool {
    x >= 0 && x < tile_size && y >= 0 && y < tile_size
}

#[allow(dead_code)]
pub struct WaveFunctionCollapse {
    tile_size: i32,
    patterns: Vec<Vec<TileType>>,
    frequencies: HashMap<TileType, f32>,
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
        self.compute_frequencies(); // frequency hints
        let constraints = self.build_constraints(); // patterns + adjacency rules

        //let output_size = map.width * map.height;
        let output_size = map.width / self.tile_size * map.height / self.tile_size;
        let mut cells: Vec<Cell> = Vec::new();
        for _i in 0..output_size {
            let mut cell = Cell {
                patterns: Vec::new(),
            };
            for p in constraints.iter() {
                cell.patterns.push(p.clone());
            }
            cells.push(cell);
        }

        let mut wave = Wave::new(cells);

        /*
        while wave.uncollapsed_cells > 0 {
            let next_coord = wave.choose_next_cell();
            wave.collapse_cell_at(next_coord);
            wave.propagate();
            wave.uncollapsed_cells -= 1;
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
                //let inverted_pattern = self.get_pattern(map, start, end, "invert");
                self.patterns.push(normal_pattern);
                self.patterns.push(vert_pattern);
                self.patterns.push(horiz_pattern);
                self.patterns.push(verthoriz_pattern);
                //self.patterns.push(inverted_pattern);
            }
        }

        // Remove identical pattern.
        deduplicate(&mut self.patterns);
    }

    fn compute_frequencies(&mut self) {
        // Calculate frequencies (absolute).
        for pattern in self.patterns.iter() {
            for ttype in pattern.iter() {
                *self.frequencies.entry(*ttype).or_insert(0.0) += 1.0;
            }
        }

        // Update frequencies to relative frequencies.
        let total: f32 = self.frequencies.values().sum();
        for v in self.frequencies.values_mut() {
            *v /= total;
        }
    }

    fn build_constraints(&self) -> Vec<MapTile> {
        let mut constraints: Vec<MapTile> = Vec::new();

        for p1 in self.patterns.iter() {
            let mut map_tile = MapTile {
                pattern: p1.to_vec(),
                compatible: Vec::new(),
                size: self.tile_size,
            };
            for p2 in self.patterns.iter() {
                if p1 != p2 {
                    if self.is_compatible(p1.to_vec(), p2.to_vec(), NORTH) {
                        map_tile.compatible.push((p2.to_vec(), NORTH));
                    }
                    if self.is_compatible(p1.to_vec(), p2.to_vec(), SOUTH) {
                        map_tile.compatible.push((p2.to_vec(), SOUTH));
                    }
                    if self.is_compatible(p1.to_vec(), p2.to_vec(), EAST) {
                        map_tile.compatible.push((p2.to_vec(), EAST));
                    }
                    if self.is_compatible(p1.to_vec(), p2.to_vec(), WEST) {
                        map_tile.compatible.push((p2.to_vec(), WEST));
                    }
                }
            }
            println!("{:?}", map_tile);
            constraints.push(map_tile);
        }

        constraints
    }

    /// Checks if there is overlap.
    /// I need to review this, because it's probably wrong!
    fn is_compatible(&self, p1: Vec<TileType>, p2: Vec<TileType>, dir: Direction) -> bool {
        for y in 0..self.tile_size {
            for x in 0..self.tile_size {
                let p1_pos = Point::new(x, y);
                let offset = p1_pos + dir;
                if !in_tile_bounds(self.tile_size, offset.x, offset.y) {
                    continue;
                }
                if p1[tile_idx(self.tile_size, p1_pos.x, p1_pos.y)]
                    != p2[tile_idx(self.tile_size, offset.x, offset.y)]
                {
                    return false;
                }
            }
        }
        true
    }

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
                    "invert" => {
                        if map.in_map_bounds_xy(y, x) {
                            idx = map.idx(y, x);
                        } else {
                            idx = map.idx(x, y);
                        }
                        //idx = map.idx(end.y - (y + 1), end.x - (x + 1));
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
