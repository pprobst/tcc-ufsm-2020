use super::{Point, Position};
use bracket_lib::prelude::RandomNumberGenerator;
mod tile;
use tile::{Tile, TileType};
mod room;
use room::*;
mod tunnel;
use tunnel::*;
mod region;
use region::*;
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
mod digger;
use digger::*;
mod prefab_map;
use prefab_map::*;
mod wfc;
use wfc::*;

pub struct MapGenerator {
    pub map: Map,
    pub rooms: Option<Vec<Room>>,
    pub tunnels: Option<Vec<Tunnel>>,
    pub regions: Option<Vec<Region>>,
}

impl MapGenerator {
    pub fn new(width: i32, height: i32) -> Self {
        Self {
            map: Map::new(width, height),
            rooms: None,
            tunnels: None,
            regions: None,
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
        //let mut walker = RandomWalker::new(0.45, false, false);
        //walker.generate(&mut self.map, &mut rng);

        // n_iterations, n_walls_rule, min_cave_size, open_halls, dry_caves
        //let mut cell_automata = CellularAutomata::new(12, 5, 20, false, false);
        //cell_automata.generate(&mut self.map);
        //make_lake(&mut self.map, TileType::ShallowWater, 500);

        // If we have two Cellular Automata generators:
        // - 1st with open halls and lots of iterations.
        // - 2nd without big open halls and only one iteration.
        // We generate very claustrophobic dungeons!
        //
        // Generally speaking, a second run of Cellular Automata (only one generation)
        // is pretty good to smooth things out.
        //let mut cell_automata2 = CellularAutomata::new(1, 5, 20, true, true);
        //cell_automata2.generate(&mut self.map);

        //let mut bsp = BSPDungeon::new(5, false);
        //bsp.generate(&mut self.map, &mut rng);
        //bsp.build_tunnels(&mut self.map, &mut rng);
        //make_lake(&mut self.map, TileType::ShallowWater, 500);

        // (min_size, max_size, num_features (approx)
        // num_features is approximate because depending on the room size and size of the map it
        // may not be possible to insert all features.
        // Biggers rooms are more aesthetically pleasing, but require a much greater map (from
        // 100x100 to 200x200) to have more features.
        //let mut digger = Digger::new(10, 20, 30);
        //digger.generate(&mut self.map, &mut rng);

        //let mut handmade_map = PrefabMap::new("../rex_resources/dungeon03_60x60.xp");
        //let mut handmade_map = PrefabMap::new("../rex_resources/wfc_9x9.xp");
        let mut handmade_map = PrefabMap::new("../rex_resources/wfc_20x20_4.xp");
        handmade_map.generate(&mut self.map);

        // About WFC (overlapping model):
        // WFC is good, but the results can vary from extremely bad to extremely good.
        // Generally, WFC has trouble differentiating between "outside" and "inside",
        // so it's a good idea to use a different floor tile to represent the inside of
        // structures. Also, it's not a good idea do add too much details in WFC, like
        // doors, chests and other stuff: they tend do lose meaning in the output!
        //
        // WFC, however, is great at mimickying the overlaying architecture of an input to
        // different degrees: a bigger tile size can give us an output very similar to
        // the input, but with less variability; with smaller tile sizes, the output
        // can be seen as a "micro architecture" of the input.
        //
        // Depending on the input, WFC also doesn't assure connectivity. This can be fixed
        // with a myriad of methods afterwards.
        //
        let mut wfc = wfc::WaveFunctionCollapse::new(7, "similarity");
        wfc.generate(&mut self.map, 20, 20, &mut rng);

        self.regions = Some(get_all_regions(&self.map));
        let mut regions = self.regions.clone().unwrap(); // get_all_regions is guaranteed to return at least one region
        regions.retain(|a| a.len() >= 5);
        regions.sort_by(|a, b| self.map.idx_pos(a[0]).x.cmp(&self.map.idx_pos(b[0]).x));
        //println!("{}", regions.len());
        if regions.len() > 1 {
            connect_regions(&mut self.map, regions, TileType::WoodenFloor, false); 
        }
        make_lake(&mut self.map, TileType::ShallowWater, 500);

        // Sometimes, applying cellular automata to a WFC output is cool.
        //let mut cell_automata = CellularAutomata::new(1, 4, 100, false, true);
        //cell_automata.generate(&mut self.map);

        self.map.add_borders();
        //self.map.pretty_walls();
        add_vegetation(&mut self.map);
        // future: apply_theme(map)
        println!("Map generated!");
    }

    pub fn get_map(&self) -> Map {
        self.map.clone()
    }
}
