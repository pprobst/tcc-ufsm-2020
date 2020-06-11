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
    pub maps: Vec<Map>,
    pub rooms: Option<Vec<Room>>,
    pub tunnels: Option<Vec<Tunnel>>,
    pub regions: Option<Vec<Region>>,
    pub rng: RandomNumberGenerator,
}

#[allow(dead_code)]
impl MapGenerator {
    pub fn new() -> Self {
        Self {
            //maps: Map::new(width, height).push(),
            maps: Vec::new(),
            rooms: None,
            tunnels: None,
            regions: None,
            rng: RandomNumberGenerator::new(),
        }
    }

    pub fn push_map(&mut self, width: i32, height: i32) {
        self.maps.push(Map::new(width, height));
    }

    pub fn gen_map(&mut self, idx: usize) {
        self.gen_forest(idx);
        //HOUSE01.generate(Point::new(20, 20), &mut self.map);
        //self.gen_cave(&mut rng);
        //self.gen_tight_cave(&mut rng);
        //self.gen_bsp(&mut rng);
        //self.gen_bsp_ruin(&mut rng);
        //self.gen_bsp_ruin_2(&mut rng);
        //self.gen_digger(&mut rng);
        //self.gen_digger_inverted(&mut rng);
        //self.map.add_borders();
        //self.map.pretty_walls();
        //add_vegetation(&mut self.map);
        println!("Map generated!");
    }

    pub fn gen_forest(&mut self, idx: usize) {
        self.maps[idx].make_chaotic(50);

        // n_iterations, n_walls_rule, min_cave_size, open_halls, dry_caves
        let mut cell_automata = CellularAutomata::new(12, 5, 20, true, true);
        cell_automata.generate(&mut self.maps[idx]);

        // Make two big lakes.
        make_lake(&mut self.maps[idx], TileType::ShallowWater, 600);
        make_lake(&mut self.maps[idx], TileType::ShallowWater, 600);

        let mut cell_automata2 = CellularAutomata::new(1, 3, 20, true, true);
        cell_automata2.generate(&mut self.maps[idx]);

        self.maps[idx].apply_forest_theme();
        add_vegetation(&mut self.maps[idx], true);
    }

    pub fn gen_cave(&mut self, idx: usize) {
        let chance = self.rng.range(0, 2);
        let d: bool = if chance == 0 { false } else { true };

        // floor_percent, grouped_walkers, diagonals
        let mut walker = RandomWalker::new(0.55, false, d);
        walker.generate(&mut self.maps[idx], &mut self.rng);

        // n_iterations, n_walls_rule, min_cave_size, open_halls, dry_caves
        let mut cell_automata = CellularAutomata::new(12, 5, 20, false, false);
        cell_automata.generate(&mut self.maps[idx]);
        make_lake(&mut self.maps[idx], TileType::ShallowWater, 200);

        let mut cell_automata2 = CellularAutomata::new(1, 4, 5, true, true);
        cell_automata2.generate(&mut self.maps[idx]);

        if self.rng.range(0, 3) < 1 {
            add_vegetation(&mut self.maps[idx], false);
        }
    }

    pub fn gen_tight_cave(&mut self, idx: usize) {
        let mut chance = self.rng.range(0, 2);
        let d = if chance == 0 { false } else { true };

        // floor_percent, grouped_walkers, diagonals
        let mut walker = RandomWalker::new(0.60, true, d);
        walker.generate(&mut self.maps[idx], &mut self.rng);

        // n_iterations, n_walls_rule, min_cave_size, open_halls, dry_caves
        let mut cell_automata = CellularAutomata::new(12, 5, 5, false, true);
        cell_automata.generate(&mut self.maps[idx]);

        chance = self.rng.range(0, 3);
        let rule = if chance <= 1 { 5 } else { 2 };

        let mut cell_automata2 = CellularAutomata::new(5, rule, 5, true, true);
        cell_automata2.generate(&mut self.maps[idx]);

        if self.rng.range(0, 5) < 1 {
            add_vegetation(&mut self.maps[idx], false);
        }
    }

    pub fn gen_bsp(&mut self, idx: usize) {
        let mut chance = self.rng.range(0, 5);
        let c = if chance < 4 { false } else { true };

        let mut bsp = BSPDungeon::new(10, c);
        bsp.generate(&mut self.maps[idx], &mut self.rng);

        chance = self.rng.range(0, 3);
        if c == false {
            match chance {
                // With smaller block sizes (e.g. 5), tunnels_left and tunnels_down become bad.
                0 => {
                    bsp.build_tunnels_left(&mut self.maps[idx], &mut self.rng);
                }
                1 => {
                    bsp.build_tunnels_down(&mut self.maps[idx], &mut self.rng);
                }
                _ => {
                    bsp.build_tunnels(&mut self.maps[idx], &mut self.rng);
                }
            }
        } else {
            match chance {
                // Tunnels get too clutered when they're ordered and we have "big connected rooms".
                _ => {
                    bsp.build_tunnels(&mut self.maps[idx], &mut self.rng);
                }
            }
        }
        self.rooms = Some(bsp.get_rooms());
        add_doors(&mut self.maps[idx], self.rooms.as_ref(), 30, &mut self.rng);
    }

    pub fn gen_bsp_ruin(&mut self, idx: usize) {
        self.gen_bsp(idx);
        make_lake(&mut self.maps[idx], TileType::ShallowWater, 100);
        let mut cell_automata = CellularAutomata::new(2, 3, 10, true, false);
        cell_automata.generate(&mut self.maps[idx]);
        add_vegetation(&mut self.maps[idx], false);
    }

    pub fn gen_bsp_ruin_2(&mut self, idx: usize) {
        self.gen_tight_cave(idx);
        self.gen_bsp(idx);
        make_lake(&mut self.maps[idx], TileType::ShallowWater, 100);
        //let mut cell_automata = CellularAutomata::new(1, 1, 5, true, false);
        //cell_automata.generate(&mut self.map);
        add_vegetation(&mut self.maps[idx], false);
    }

    pub fn gen_digger(&mut self, idx: usize) {
        // num_features is approximate because depending on the room size and size of the map it
        // may not be possible to insert all features.
        // Biggers rooms are more aesthetically pleasing, but require a much greater map (from
        // 100x100 to 200x200) to have more features.
        // (min_size, max_size, num_features (approx)
        let mut digger = Digger::new(10, 15, 30);
        digger.generate(&mut self.maps[idx], &mut self.rng);
        self.rooms = Some(digger.get_rooms());
        add_doors(&mut self.maps[idx], self.rooms.as_ref(), 30, &mut self.rng);
    }

    pub fn gen_digger_inverted(&mut self, idx: usize) {
        self.gen_digger(idx);
        // n_iterations, n_walls_rule, min_cave_size, open_halls, dry_caves
        let mut cell_automata = CellularAutomata::new(3, 7, 10, false, false);
        cell_automata.generate(&mut self.maps[idx]);
        if self.rng.range(0, 2) < 1 {
            add_vegetation(&mut self.maps[idx], false);
        }
    }

    pub fn get_map(&self, idx: usize) -> Map {
        self.maps[idx].clone()
    }
}
