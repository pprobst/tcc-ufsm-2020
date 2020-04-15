use bracket_lib::prelude::{RandomNumberGenerator};
use super::{Position, Point};
mod tile;
use tile::{TileType, Tile};
mod room;
use room::*;
pub mod map;
pub use map::Map;
mod common;
use common::*;
mod random_walk;
use random_walk::*;
mod cellular_automata;
use cellular_automata::*;
mod bsp_tree;
use bsp_tree::*;

pub struct MapGenerator {
    pub map: Map,
    pub rooms: Option<Vec<Room>>,
    pub tunnels: Option<Vec<Vec<usize>>>,
    pub caves: Option<Vec<Vec<usize>>>,
}

impl MapGenerator {
    pub fn new(width: i32, height: i32) -> Self {
        Self {
           map: Map::new(width, height),
           rooms: None,
           tunnels: None,
           caves: None,
        }
    }

    pub fn gen_map(&mut self) {
        let mut rng = RandomNumberGenerator::new();

        // Run make_chaotic if we don't have a prior map for the Cellular Automata.
        //self.map.make_chaotic(45);

        // If we create a walker with (0.45, true, false) and after that we run
        // Cellular Automata with (12, 5, 50, false), we get linear organic
        // dungeons. Then, if we run a BSP generator, it'll become like 
        // cave with man-made rooms!
        let mut walker = RandomWalker::new(0.50, false, false);
        walker.generate(&mut self.map, &mut rng);

        let mut cell_automata = CellularAutomata::new(15, 5, 80, false);
        cell_automata.generate(&mut self.map);

        // If we have two Cellulat Automata generators:
        // - 1st with open halls and lots of iterations.
        // - 2nd without big open halls and only one iteration.
        // We generate very claustrophobic dungeons!
        //
        // Generally speaking, a second run of Cellular Automata (only one generation)
        // is pretty good to smooth things out.
        let mut cell_automata2 = CellularAutomata::new(1, 5, 80, true);
        cell_automata2.generate(&mut self.map);

        //let mut bsp = BSPDungeon::new(8, false);
        //bsp.generate(&mut self.map, &mut rng);
        //bsp.build_tunnels(&mut self.map, &mut rng);

        self.map.add_borders();
        //self.map.pretty_walls();
        // future: apply_theme(map)
        println!("Map generated!");
    }

    pub fn get_map(&self) -> Map {
        self.map.clone()
    }
}
