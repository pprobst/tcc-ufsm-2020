use super::{Map, Point, TileType};
use crate::utils::directions::*;
use bracket_lib::prelude::RandomNumberGenerator;
use std::collections::{HashMap, HashSet};

mod common;
use common::*;
mod cell;
use cell::*;
mod wave;
use wave::*;

/*
 * This file contains the core WFC algorithm.
 *
 * This is NOT meant to be the fastest WFC algorithm possible; stevebob/wfc is plenty
 * fast already. Our point here is to LEARN and USE WFC, comparing it with traditional
 * PCG methods.
 *
 * For reference, I used the following resources:
 * - https://gridbugs.org/wave-function-collapse/
 * - https://frame.42yeah.casa/2020/01/30/wfc.html
 * - https://www.youtube.com/watch?v=ws4r3wLPNSE&list=PLcRSafycjWFeKAS40OdIvhL7j-vsgE3eg
 * - https://robertheaton.com/2018/12/17/wavefunction-collapse-algorithm/
 *
 */

/*
#[derive(Debug, PartialEq, Clone)]
struct MinFloat(f32);

impl Eq for MinFloat {}

impl PartialOrd for MinFloat {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        other.0.partial_cmp(&self.0)
    }
}

impl Ord for MinFloat {
    fn cmp(&self, other: &MinFloat) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

#[derive(PartialEq, Eq, Debug, Clone)]
struct CoordEntropy {
    entropy: MinFloat,
    coord: Point,
}

impl PartialOrd for CoordEntropy {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        other.entropy.partial_cmp(&self.entropy)
    }
}

impl Ord for CoordEntropy {
    fn cmp(&self, other: &Self) -> Ordering {
        if self < other {
            return Ordering::Less;
        }
        if self == other {
            return Ordering::Equal;
        }
        return Ordering::Greater;
    }
}

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct RemovalUpdate {
    tile: MapTile,
    coord: Point,
}
*/

/*

#[derive(Debug, Clone)]
pub struct Wave {
    cells: Vec<Cell>,
    uncollapsed_cells: usize,
    entropy_queue: BinaryHeap<CoordEntropy>,
    tile_removals: Vec<RemovalUpdate>,
    out_width: i32,
    out_height: i32,
}

impl Wave {
    pub fn new(cells: Vec<Cell>, out_width: i32, out_height: i32) -> Self {
        let cells_len = cells.len();
        Self {
            cells,
            uncollapsed_cells: cells_len,
            entropy_queue: BinaryHeap::new(),
            tile_removals: Vec::new(),
            out_height,
            out_width,
        }
    }

    fn cell_at(&self, x: i32, y: i32) -> usize {
        (y as usize * self.out_width as usize) + x as usize
    }

    fn in_bound(&self, x: i32, y: i32) -> bool {
        x >= 0 && x < self.out_width && y > 0 && y < self.out_height
    }

    fn choose_next_cell(&mut self) -> Point {
        while let Some(entropy_coord) = self.entropy_queue.pop() {
            let idx = self.cell_at(entropy_coord.coord.x, entropy_coord.coord.y);
            let cell = &self.cells[idx];
            if !cell.collapsed {
                return entropy_coord.coord;
            }
        }
        unreachable!("entropy_queue is empty!");
    }

    pub fn collapse_cell_at(
        &mut self,
        pt: Point,
        freq: &HashMap<Vec<TileType>, f32>,
        rng: &mut RandomNumberGenerator,
    ) {
        let idx = self.cell_at(pt.x, pt.y);
        let mut cell = &mut self.cells[idx];
        let locked_tile = cell.choose_tile(freq, rng);

        cell.collapsed = true;

        let patterns = cell.patterns.clone();
        for tile in patterns {
            if tile != locked_tile {
                let t = tile.clone();
                cell.remove_tile(tile, freq);
                self.tile_removals
                    .push(RemovalUpdate { tile: t, coord: pt })
            }
        }
    }

    pub fn propagate(&mut self, freq: &HashMap<Vec<TileType>, f32>) {
        while let Some(removal_update) = self.tile_removals.pop() {
            for i in 0..4 {
                let dir;
                match i {
                    0 => {
                        dir = EAST;
                    }
                    1 => {
                        dir = WEST;
                    }
                    2 => {
                        dir = NORTH;
                    }
                    _ => {
                        dir = SOUTH;
                    }
                }
                let neighbor_coord = removal_update.coord + dir;
                let neighbor_idx = self.cell_at(neighbor_coord.x, neighbor_coord.y);
                let neighbor_patterns = self.cells[neighbor_idx].patterns.clone();
                let neighbor_cell = &mut self.cells[neighbor_idx];

                for pattern in neighbor_patterns {
                    let possible =
                        removal_update.tile.compatible.iter().any(|c| c.0 == pattern.pattern && c.1 == dir);
                    if !possible {
                        neighbor_cell.remove_tile(pattern.clone(), freq);
                        if neighbor_cell.patterns.len() == 0 {
                            println!("Contradiction!"); // do something
                        }
                        self.entropy_queue.push(CoordEntropy{
                            entropy: MinFloat(neighbor_cell.entropy()),
                            coord: neighbor_coord
                        });
                        self.tile_removals.push(RemovalUpdate{
                            tile: pattern,
                            coord: neighbor_coord
                        });
                    }
                }
            }
        }
    }
}
*/

/*
#[derive(Debug, Clone, PartialEq)]
pub struct MapTile {
    idx: usize,
    pattern: Vec<TileType>,
    compatible: Vec<(Vec<TileType>, Direction)>, // overlaps
    size: i32,
}
*/

pub fn tile_idx(tile_size: i32, x: i32, y: i32) -> usize {
    ((y * tile_size) + x) as usize
}

pub fn in_tile_bounds(tile_size: i32, x: i32, y: i32) -> bool {
    x >= 0 && x < tile_size && y >= 0 && y < tile_size
}

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct WaveFunctionCollapse {
    tile_size: i32,
    patterns: Vec<Vec<TileType>>,
    frequencies: HashMap<Vec<TileType>, f32>,
    //frequencies: HashMap<usize, f32>,
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

    pub fn generate(&mut self, map: &mut Map, rng: &mut RandomNumberGenerator) {
        self.build_patterns(map);
        self.compute_frequencies(); // frequency hints
        deduplicate(&mut self.patterns);
        let constraints = self.build_constraints(); // patterns + adjacency rules

        //let output_size = map.width * map.height;
        let out_width = map.width / self.tile_size;
        let out_height = map.height / self.tile_size;
        let output_size = out_width * out_height;
        let mut cells: Vec<Cell> = Vec::new();
        for _i in 0..output_size {
            let noise = rng.range(0.001f32, 0.005f32);
            let cell = Cell::new(constraints.clone(), noise);
            /*
            for p in constraints.iter() {
                cell.patterns.push(p.clone());
            }
            */
            // Initially a cell can have all patterns.
            cells.push(cell);
        }

        for cell in cells.iter_mut() {
            cell.total_possible_tile_freq(&self.frequencies);
        }

        let mut wave = Wave::new(cells, out_width, out_height);

        /*
        while wave.uncollapsed_cells > 0 {
            let next_coord = wave.choose_next_cell();
            wave.collapse_cell_at(next_coord, &self.frequencies, rng);
            wave.propagate(&self.frequencies);
            wave.uncollapsed_cells -= 1;
        }
        */
    }

    /// Build tiles of size tile_size x tile_size from map cells.
    fn build_patterns(&mut self, map: &mut Map) {
        // Navigate the coordinates of each tile.
        // Change map.height and map.width to specific input size?
        for ty in 0..(14 / self.tile_size) {
            for tx in 0..(8 / self.tile_size) {
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
    }

    fn compute_frequencies(&mut self) {
        // Calculate frequencies (absolute).
        //for (i, pattern) in self.patterns.iter().enumerate() {
        for pattern in self.patterns.iter() {
            *self.frequencies.entry(pattern.to_vec()).or_insert(0.0) += 1.0;
            //*self.frequencies.entry(i).or_insert(0.0) += 1.0;
            /*for ttype in pattern.iter() {
                *self.frequencies.entry(*ttype).or_insert(0.0) += 1.0;
            }*/
        }

        // Update frequencies to relative frequencies.
        let total: f32 = self.frequencies.values().sum();
        for v in self.frequencies.values_mut() {
            *v /= total;
        }
    }

    fn build_constraints(&self) -> Vec<MapTile> {
        let mut constraints: Vec<MapTile> = Vec::new();

        for (i, p1) in self.patterns.iter().enumerate() {
            let mut map_tile = MapTile {
                idx: i,
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
            //println!("{:?}", map_tile);
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
