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
        for pattern in self.patterns.iter() {
            *self.frequencies.entry(pattern.to_vec()).or_insert(0.0) += 1.0;
        }

        // Update frequencies to relative frequencies.
        let total: f32 = self.frequencies.values().sum();
        for v in self.frequencies.values_mut() {
            *v /= total;
        }
        //println!("{:?}", self.frequencies);
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
