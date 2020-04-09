use super::{Position};
mod tile;
use tile::{TileType, Tile};
pub mod map;
pub use map::Map;
//mod random_map;
//use random_map::*;
mod random_walk;
use random_walk::*;

pub struct MapGenerator {
    pub map: Map
}

impl MapGenerator {
    pub fn new(width: i32, height: i32) -> Self {
        Self {
           map: Map::new(width, height),
        }
    }

    pub fn gen_map(&mut self) {
        let mut walker = RandomWalker::new(0.2, true);
        walker.generate(&mut self.map);
        //random_map_gen(&mut self.map);
        // future: apply_theme(map)
        println!("Map generated!");
    }

    pub fn get_map(&self) -> Map {
        self.map.clone()
    }
}
