use super::TileType;
use crate::utils::directions::*;
use bracket_lib::prelude::RandomNumberGenerator;
use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq)]
pub struct MapTile {
    pub idx: usize,
    pub pattern: Vec<TileType>,
    //pub compatible: Vec<(Vec<TileType>, Direction)>, // overlaps
    pub compatible: Vec<(usize, Direction)>, // overlaps with MapTile of idx usize at Direction
    pub size: i32,
}

impl MapTile {
    //pub fn get_compatible_dir(&self, dir: Direction) -> Vec<Vec<TileType>> {
    pub fn get_compatible_dir(&self, dir: Direction) -> Vec<usize> {
        //let mut compats: Vec<Vec<TileType>> = Vec::new();
        let mut compats: Vec<usize> = Vec::new();
        for c in self.compatible.iter() {
            //if c.1 == dir { compats.push(c.0.to_vec()); }
            if c.1 == dir {
                compats.push(c.0);
            }
        }
        compats
    }
}

#[derive(Debug, Clone)]
pub struct TileEnablerCount {
    // EAST, WEST, NORTH, SOUTH
    pub by_direction: [usize; 4],
}

impl TileEnablerCount {
    pub fn any_zero(&self) -> bool {
        self.by_direction.iter().any(|d| *d == 0)
    }
}

#[derive(Debug, Clone)]
pub struct Cell {
    pub patterns: Vec<MapTile>, // Possible patterns of a cell
    sum_possible_weights: f32,
    sum_possible_weights_log: f32,
    entropy_noise: f32,
    pub collapsed: bool,
    pub enabler_count: Vec<TileEnablerCount>,
}

impl Cell {
    pub fn new(patterns: Vec<MapTile>, entropy_noise: f32) -> Self {
        Self {
            patterns,
            sum_possible_weights: 0.0,
            sum_possible_weights_log: 0.0,
            entropy_noise,
            collapsed: false,
            enabler_count: Vec::new(),
        }
    }

    pub fn initial_enabler_count(&mut self) {
        for tile in self.patterns.iter() {
            let mut counts = TileEnablerCount {
                by_direction: [0, 0, 0, 0],
            };
            counts.by_direction[0] = tile.get_compatible_dir(EAST).len();
            counts.by_direction[1] = tile.get_compatible_dir(WEST).len();
            counts.by_direction[2] = tile.get_compatible_dir(NORTH).len();
            counts.by_direction[3] = tile.get_compatible_dir(SOUTH).len();
            self.enabler_count.push(counts);
        }
    }

    pub fn entropy(&self) -> f32 {
        return self.sum_possible_weights.log2()
            - (self.sum_possible_weights_log / self.sum_possible_weights as f32)
            + self.entropy_noise;
    }

    // Removes a map tile (pattern) from the list of possible tiles.
    // Updates the sums of possible weights for Entropy calculation.
    //pub fn remove_tile(&mut self, tile: &MapTile, freq: &HashMap<Vec<TileType>, f32>) {
    //pub fn remove_tile(&mut self, tile: &MapTile, freq: &HashMap<usize, f32>) {
    pub fn remove_tile(&mut self, tile_idx: usize, freq: &HashMap<usize, f32>) {
        //self.patterns.retain(|x| *x != *tile);
        self.patterns.retain(|x| x.idx != tile_idx);

        //let f = freq.get(&tile.pattern).unwrap();
        let f = freq.get(&tile_idx).unwrap();
        self.sum_possible_weights -= f;
        self.sum_possible_weights_log -= f * f.log2();
    }

    /*
    pub fn remove_tile_idx(&mut self, tile_idx: usize, freq: &HashMap<Vec<TileType>, f32>) {
        self.patterns.retain(|x| x.idx != tile_idx);

    }
    */

    // Add up the relative frequencies of all possible tiles.
    // It's the total weight in the Entropy equation.
    //pub fn total_possible_tile_freq(&mut self, freq: &HashMap<Vec<TileType>, f32>) {
    pub fn total_possible_tile_freq(&mut self, freq: &HashMap<usize, f32>) {
        let mut total = 0.0;
        let mut total_log = 0.0;
        //for (i, p) in self.possible.iter().enumerate() {
        for maptile in self.patterns.iter() {
            //let pattern = &maptile.pattern;
            let tile_index = &maptile.idx;
            if freq.contains_key(tile_index) {
                let freq_hint = freq.get(tile_index).unwrap();
                total += freq_hint;
                total_log += freq_hint * freq_hint.log2();
            }
        }
        //println!("{} {}", total, total_log);
        self.sum_possible_weights = total;
        self.sum_possible_weights_log = total_log;
    }

    pub fn choose_tile(
        &self,
        //freq: &HashMap<Vec<TileType>, f32>,
        freq: &HashMap<usize, f32>,
        rng: &mut RandomNumberGenerator,
    ) -> MapTile {
        let mut remain = rng.range(0.0, self.sum_possible_weights);

        for tile in self.patterns.iter() {
            let weight = *freq.get(&tile.idx).unwrap();
            if remain >= weight {
                remain -= weight;
            } else {
                return tile.clone();
            }
        }

        unreachable!("sum_possible_weights was inconsistent!");
    }
}
