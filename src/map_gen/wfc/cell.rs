use super::{MapTile, TileType};
use bracket_lib::prelude::RandomNumberGenerator;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct Cell {
    pub patterns: Vec<MapTile>, // Possible patterns of a cell
    sum_possible_weights: f32,
    sum_possible_weights_log: f32,
    entropy_noise: f32,
    pub collapsed: bool,
}

impl Cell {
    pub fn new(patterns: Vec<MapTile>, entropy_noise: f32) -> Self {
        Self {
            patterns,
            sum_possible_weights: 0.0,
            sum_possible_weights_log: 0.0,
            entropy_noise,
            collapsed: false,
        }
    }

    pub fn entropy(&self) -> f32 {
        return self.sum_possible_weights.log2()
            - (self.sum_possible_weights_log / self.sum_possible_weights as f32)
            + self.entropy_noise;
    }

    // Removes a map tile (pattern) from the list of possible tiles.
    // Updates the sums of possible weights for Entropy calculation.
    pub fn remove_tile(&mut self, tile: MapTile, freq: &HashMap<Vec<TileType>, f32>) {
        self.patterns.retain(|x| x != &tile);

        let f = freq.get(&tile.pattern).unwrap();
        self.sum_possible_weights -= f;
        self.sum_possible_weights_log -= f * f.log2();
    }

    // Add up the relative frequencies of all possible tiles.
    // It's the total weight in the Entropy equation.
    //fn total_possible_tile_freq(&self, freq: HashMap<usize, f32>) -> f32 {
    pub fn total_possible_tile_freq(&mut self, freq: &HashMap<Vec<TileType>, f32>) {
        let mut total = 0.0;
        let mut total_log = 0.0;
        //for (i, p) in self.possible.iter().enumerate() {
        for maptile in self.patterns.iter() {
            let pattern = &maptile.pattern;
            //let tile_index = i;
            if freq.contains_key(pattern) {
                let freq_hint = freq.get(pattern).unwrap();
                total += freq_hint;
                total_log += freq_hint * freq_hint.log2();
            }
        }
        println!("{} {}", total, total_log);
        self.sum_possible_weights = total;
        self.sum_possible_weights_log = total_log;
    }

    pub fn choose_tile(
        &self,
        freq: &HashMap<Vec<TileType>, f32>,
        rng: &mut RandomNumberGenerator,
    ) -> MapTile {
        let mut remain = rng.range(0.0, self.sum_possible_weights);

        for tile in self.patterns.iter() {
            let weight = *freq.get(&tile.pattern).unwrap();
            if remain >= weight {
                remain -= weight;
            } else {
                return tile.clone();
            }
        }

        unreachable!("sum_possible_weights was inconsistent!");
    }
}
