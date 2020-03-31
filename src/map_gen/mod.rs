mod tile;
use tile::{TileType, Tile};
pub mod map;
pub use map::Map;
mod random_map;
use random_map::*;

pub struct MapGenerator {
    pub map: Map
}

impl MapGenerator {
    pub fn new() -> Self {
        Self {
           map: Map::new(80, 80),
        }
    }

    pub fn gen_map(&mut self) {
        random_map_gen(&mut self.map);
        // future: apply_theme(map)
        println!("Map generated!");
    }

    pub fn get_map(&self) -> Map {
        self.map.clone()
    }
}
