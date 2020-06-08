use super::{Point, Position};
use bracket_lib::prelude::RandomNumberGenerator;
pub mod tile;
pub use tile::{Tile, TileType};
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
mod prefab_section;
use prefab_section::*;
mod wfc;
use wfc::*;

pub struct MapGenerator {
    pub map: Map,
    pub rooms: Option<Vec<Room>>,
    pub tunnels: Option<Vec<Tunnel>>,
    pub regions: Option<Vec<Region>>,
}

#[allow(dead_code)]
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
        //let mut handmade_map = PrefabMap::new("../rex_resources/wfc_20x20_4.xp");
        //handmade_map.generate(&mut self.map);

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
        /*
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
        */

        // Sometimes, applying cellular automata to a WFC output is cool.
        //let mut cell_automata = CellularAutomata::new(1, 4, 100, false, true);
        //cell_automata.generate(&mut self.map);

        self.gen_forest();
        //HOUSE01.generate(Point::new(20, 20), &mut self.map);
        //self.gen_cave(&mut rng);
        //self.gen_tight_cave(&mut rng);
        //self.gen_bsp(&mut rng);
        //self.gen_bsp_ruin(&mut rng);
        //self.gen_bsp_ruin_2(&mut rng);
        //self.gen_digger(&mut rng);
        //self.gen_digger_inverted(&mut rng);
        self.map.add_borders();
        self.map.pretty_walls();
        //add_vegetation(&mut self.map);
        println!("Map generated!");
    }

    pub fn gen_forest(&mut self) {
        self.map.make_chaotic(50);

        // n_iterations, n_walls_rule, min_cave_size, open_halls, dry_caves
        let mut cell_automata = CellularAutomata::new(12, 5, 20, true, true);
        cell_automata.generate(&mut self.map);

        // Make two big lakes.
        make_lake(&mut self.map, TileType::ShallowWater, 600);
        make_lake(&mut self.map, TileType::ShallowWater, 600);

        let mut cell_automata2 = CellularAutomata::new(1, 3, 20, true, true);
        cell_automata2.generate(&mut self.map);

        self.map.apply_forest_theme();
        add_vegetation(&mut self.map, true);
    }

    pub fn gen_cave(&mut self, rng: &mut RandomNumberGenerator) {
        let chance = rng.range(0, 2);
        let d: bool = if chance == 0 { false } else { true };

        // floor_percent, grouped_walkers, diagonals
        let mut walker = RandomWalker::new(0.55, false, d);
        walker.generate(&mut self.map, rng);

        // n_iterations, n_walls_rule, min_cave_size, open_halls, dry_caves
        let mut cell_automata = CellularAutomata::new(12, 5, 20, false, false);
        cell_automata.generate(&mut self.map);
        make_lake(&mut self.map, TileType::ShallowWater, 200);

        let mut cell_automata2 = CellularAutomata::new(1, 4, 5, true, true);
        cell_automata2.generate(&mut self.map);

        if rng.range(0, 3) < 1 { add_vegetation(&mut self.map, false); }
    }

    pub fn gen_tight_cave(&mut self, rng: &mut RandomNumberGenerator) {
        let mut chance = rng.range(0, 2);
        let d = if chance == 0 { false } else { true };

        // floor_percent, grouped_walkers, diagonals
        let mut walker = RandomWalker::new(0.60, true, d);
        walker.generate(&mut self.map, rng);

        // n_iterations, n_walls_rule, min_cave_size, open_halls, dry_caves
        let mut cell_automata = CellularAutomata::new(12, 5, 5, false, true);
        cell_automata.generate(&mut self.map);

        chance = rng.range(0, 3);
        let rule = if chance <= 1 { 5 } else { 2 };

        let mut cell_automata2 = CellularAutomata::new(5, rule, 5, true, true);
        cell_automata2.generate(&mut self.map);

        if rng.range(0, 5) < 1 { add_vegetation(&mut self.map, false); }
    }

    pub fn gen_bsp(&mut self, rng: &mut RandomNumberGenerator) {
        let mut chance = rng.range(0, 5);
        let c = if chance < 4 { false } else { true };

        let mut bsp = BSPDungeon::new(10, c);
        bsp.generate(&mut self.map, rng);

        chance = rng.range(0, 3);
        if c == false {
            match chance {
                // With smaller block sizes (e.g. 5), tunnels_left and tunnels_down become bad.
                0 => {
                    bsp.build_tunnels_left(&mut self.map, rng);
                }
                1 => {
                    bsp.build_tunnels_down(&mut self.map, rng);
                }
                _ => {
                    bsp.build_tunnels(&mut self.map, rng);
                }
            }
        } else {
            match chance {
                // Tunnels get too clutered when they're ordered and we have "big connected rooms".
                _ => {
                    bsp.build_tunnels(&mut self.map, rng);
                }
            }
        }
        self.rooms = Some(bsp.get_rooms());
        add_doors(&mut self.map, self.rooms.as_ref(), 30, rng);
    }

    pub fn gen_bsp_ruin(&mut self, rng: &mut RandomNumberGenerator) {
        self.gen_bsp(rng);
        make_lake(&mut self.map, TileType::ShallowWater, 100);
        let mut cell_automata = CellularAutomata::new(2, 3, 10, true, false);
        cell_automata.generate(&mut self.map);
        add_vegetation(&mut self.map, false);
    }

    pub fn gen_bsp_ruin_2(&mut self, rng: &mut RandomNumberGenerator) {
        self.gen_tight_cave(rng);
        self.gen_bsp(rng);
        make_lake(&mut self.map, TileType::ShallowWater, 100);
        //let mut cell_automata = CellularAutomata::new(1, 1, 5, true, false);
        //cell_automata.generate(&mut self.map);
        add_vegetation(&mut self.map, false);
    }

    pub fn gen_digger(&mut self, rng: &mut RandomNumberGenerator) {
        // num_features is approximate because depending on the room size and size of the map it
        // may not be possible to insert all features.
        // Biggers rooms are more aesthetically pleasing, but require a much greater map (from
        // 100x100 to 200x200) to have more features.
        // (min_size, max_size, num_features (approx)
        let mut digger = Digger::new(10, 15, 30);
        digger.generate(&mut self.map, rng);
        self.rooms = Some(digger.get_rooms());
        add_doors(&mut self.map, self.rooms.as_ref(), 30, rng);
    }

    pub fn gen_digger_inverted(&mut self, rng: &mut RandomNumberGenerator) {
        self.gen_digger(rng);
        // n_iterations, n_walls_rule, min_cave_size, open_halls, dry_caves
        let mut cell_automata = CellularAutomata::new(3, 7, 10, false, false);
        cell_automata.generate(&mut self.map);
        if rng.range(0, 2) < 1 { add_vegetation(&mut self.map, false); }
    }

    pub fn get_map(&self) -> Map {
        self.map.clone()
    }
}
