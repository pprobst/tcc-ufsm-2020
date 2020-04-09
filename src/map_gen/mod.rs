use super::{Position, Point};
mod tile;
use tile::{TileType, Tile};
pub mod map;
pub use map::Map;
mod random_walk;
use random_walk::*;
mod cellular_automata;
use cellular_automata::*;

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
        //self.map.make_chaotic(45);
        let mut walker = RandomWalker::new(0.45, false, false);
        walker.generate(&mut self.map);
        let mut cell_automata = CellularAutomata::new(15, 4);
        cell_automata.generate(&mut self.map);
        self.map.add_borders();
        self.map.pretty_walls();
        //random_map_gen(&mut self.map);
        // future: apply_theme(map)
        println!("Map generated!");
    }

    pub fn get_map(&self) -> Map {
        self.map.clone()
    }
}
