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
mod custom_region;
use custom_region::*;
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
    pub wfc_input: Map,
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
            wfc_input: Map::new(80, 60, false),
            rng: RandomNumberGenerator::new(),
        }
    }

    pub fn push_map(&mut self, width: i32, height: i32) {
        let map = Map::new(width, height, false);
        self.maps.push(map);
    }

    pub fn gen_map(&mut self, idx: usize) {
        //let region = &CustomRegion::new_circ(25, 10, 20);
        let region = &CustomRegion::new_rect(10, 10, 60, 30);

        //self.gen_bsp(idx, Some(region));
        //self.gen_bsp(idx, None);
        //self.gen_bsp_ruin(idx, None);
        //self.gen_bsp_ruin(idx, Some(region));
        //self.gen_digger_inverted(idx, None);
        //self.gen_digger(idx, None);
        //self.gen_forest(idx, Some(region));
        //self.gen_forest(idx, None);
        //HOUSE01.generate(Point::new(20, 20), &mut self.maps[idx]);
        //self.gen_tight_cave(idx, Some(region));
        //self.gen_tight_cave(idx, None);
        //self.gen_cave(idx, None);

        //self.forest_bsp_ruin(idx);
        self.wfc_01(idx);
        /*
        let room = self.rooms.as_ref().unwrap()[0];
        let region = &CustomRegion::new_rect(room.x1, room.y1, room.width(), room.height());
        self.gen_wfc(idx, Some(region), "../rex_resources/wfc_6x6_internal.xp", 9, 9, 3);

        let room = self.rooms.as_ref().unwrap()[1];
        let region = &CustomRegion::new_rect(room.x1, room.y1, room.width(), room.height());
        self.gen_wfc(idx, Some(region), "../rex_resources/wfc_6x6_internal.xp", 9, 9, 3);
        */

        //self.gen_wfc(idx, None, "../rex_resources/wfc_15x15.xp", 15, 15, 5);

        self.maps[idx].add_borders(TileType::InvisibleWall);
        //self.maps[idx].add_borders(TileType::Wall);
        self.maps[idx].pretty_walls();
        //add_vegetation(&mut self.maps[idx]);
        println!("Map generated!");
    }

    pub fn forest_bsp_ruin(&mut self, idx: usize) {
        let region_top = &CustomRegion::new_rect(0, 0, self.maps[idx].width, 25);
        let region_middle = &CustomRegion::new_rect(0, 20, self.maps[idx].width, 15);
        let region_bottom =
            &CustomRegion::new_rect(0, 30, self.maps[idx].width, self.maps[idx].height - 30);
        self.gen_bsp(idx, Some(region_top));
        self.gen_forest(idx, Some(region_middle));
        self.gen_bsp_ruin(idx, Some(region_bottom));
        add_vegetation(&mut self.maps[idx], region_top, false);
    }

    pub fn wfc_01(&mut self, idx: usize) {
        let region_left = &CustomRegion::new_rect(0, 20, 30, 40);
        let region_middle = &CustomRegion::new_rect(28, 0, 30, 60);
        let region_right = &CustomRegion::new_rect(60, 0, 20, 60);
        let region_top_left = &CustomRegion::new_circ(0, 0, 10);
        self.gen_wfc(
            idx,
            Some(region_left),
            "resources/wfc_20x20_5.xp",
            20,
            20,
            10,
        );
        if self.rng.range(0, 2) < 1 {
            self.gen_digger(idx, Some(region_middle));
        } else {
            self.gen_bsp(idx, Some(region_middle));
        }

        if self.rng.range(0, 2) < 1 {
            self.gen_cave(idx, Some(region_right));
        } else {
            self.gen_bsp_ruin(idx, Some(region_right))
        }

        self.gen_forest(idx, Some(region_top_left));
        let all_regions = get_all_regions(&self.maps[idx], &self.maps[idx].get_region());
        connect_regions(&mut self.maps[idx], all_regions, TileType::Floor, false);
    }

    pub fn gen_wfc(
        &mut self,
        idx: usize,
        region: Option<&CustomRegion>,
        template: &'static str,
        w: i32,
        h: i32,
        tile_size: i32,
    ) {
        let map_region = &self.maps[idx].get_region();
        let reg = if region != None {
            region.unwrap()
        } else {
            map_region
        };

        let mut input = PrefabMap::new(template);
        input.generate(&mut self.wfc_input);
        //input.generate(&mut self.maps[idx]);
        let mut wfc = WaveFunctionCollapse::new(tile_size, &reg);
        // (output, input taken, template width, template height, rng)
        wfc.generate(&mut self.maps[idx], &self.wfc_input, w, h, &mut self.rng);
    }

    pub fn gen_forest(&mut self, idx: usize, region: Option<&CustomRegion>) {
        let map_region = &self.maps[idx].get_region();
        let reg = if region != None {
            region.unwrap()
        } else {
            map_region
        };

        make_chaotic(&mut self.maps[idx], reg, 50);

        // n_iterations, n_walls_rule, min_cave_size, open_halls, dry_caves
        let mut cell_automata = CellularAutomata::new(reg, 12, 5, 20, true, true);
        cell_automata.generate(&mut self.maps[idx]);

        // Make two big lakes.
        make_lake(&mut self.maps[idx], reg, TileType::ShallowWater, 600);
        make_lake(&mut self.maps[idx], reg, TileType::ShallowWater, 600);

        let mut cell_automata2 = CellularAutomata::new(reg, 1, 3, 20, true, true);
        cell_automata2.generate(&mut self.maps[idx]);

        apply_forest_theme(&mut self.maps[idx], reg);
        add_vegetation(&mut self.maps[idx], reg, true);
    }

    pub fn gen_cave(&mut self, idx: usize, region: Option<&CustomRegion>) {
        let chance = self.rng.range(0, 2);
        let d: bool = if chance == 0 { false } else { true };

        let map_region = &self.maps[idx].get_region();
        let reg = if region != None {
            region.unwrap()
        } else {
            map_region
        };

        // floor_percent, grouped_walkers, diagonals
        let mut walker = RandomWalker::new(reg, 0.55, false, d);
        walker.generate(&mut self.maps[idx], &mut self.rng);

        // n_iterations, n_walls_rule, min_cave_size, open_halls, dry_caves
        let mut cell_automata = CellularAutomata::new(reg, 12, 5, 20, false, false);
        cell_automata.generate(&mut self.maps[idx]);
        make_lake(&mut self.maps[idx], reg, TileType::ShallowWater, 200);

        let mut cell_automata2 = CellularAutomata::new(reg, 1, 4, 5, true, true);
        cell_automata2.generate(&mut self.maps[idx]);

        if self.rng.range(0, 3) < 1 {
            add_vegetation(&mut self.maps[idx], reg, false);
        }
    }

    pub fn gen_tight_cave(&mut self, idx: usize, region: Option<&CustomRegion>) {
        let mut chance = self.rng.range(0, 2);
        let d = if chance == 0 { false } else { true };

        let map_region = &self.maps[idx].get_region();
        let reg = if region != None {
            region.unwrap()
        } else {
            map_region
        };

        // floor_percent, grouped_walkers, diagonals
        let mut walker = RandomWalker::new(reg, 0.60, true, d);
        walker.generate(&mut self.maps[idx], &mut self.rng);

        // n_iterations, n_walls_rule, min_cave_size, open_halls, dry_caves
        let mut cell_automata = CellularAutomata::new(reg, 12, 5, 5, false, true);
        cell_automata.generate(&mut self.maps[idx]);

        chance = self.rng.range(0, 3);
        let rule = if chance <= 1 { 5 } else { 2 };

        let mut cell_automata2 = CellularAutomata::new(reg, 5, rule, 5, true, true);
        cell_automata2.generate(&mut self.maps[idx]);

        if self.rng.range(0, 5) < 1 {
            add_vegetation(&mut self.maps[idx], reg, false);
        }
    }

    pub fn gen_bsp(&mut self, idx: usize, region: Option<&CustomRegion>) {
        let mut chance = self.rng.range(0, 5);
        let c = if chance < 4 { false } else { true };

        // Works properly only for RECTANGULAR/SQUARE regions.
        let map_region = &self.maps[idx].get_region();
        let reg = if region != None {
            region.unwrap()
        } else {
            map_region
        };

        let mut bsp = BSPDungeon::new(reg, 10, c);
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

    pub fn gen_bsp_ruin(&mut self, idx: usize, region: Option<&CustomRegion>) {
        self.gen_bsp(idx, region);

        let map_region = &self.maps[idx].get_region();
        let reg = if region != None {
            region.unwrap()
        } else {
            map_region
        };

        make_lake(&mut self.maps[idx], reg, TileType::ShallowWater, 100);
        let mut cell_automata = CellularAutomata::new(reg, 2, 3, 10, true, false);
        cell_automata.generate(&mut self.maps[idx]);
        add_vegetation(&mut self.maps[idx], reg, false);
    }

    pub fn gen_bsp_ruin_2(&mut self, idx: usize, region: Option<&CustomRegion>) {
        self.gen_tight_cave(idx, region);
        self.gen_bsp(idx, region);

        let map_region = &self.maps[idx].get_region();
        let reg = if region != None {
            region.unwrap()
        } else {
            map_region
        };

        make_lake(&mut self.maps[idx], reg, TileType::ShallowWater, 100);
        //let mut cell_automata = CellularAutomata::new(1, 1, 5, true, false);
        //cell_automata.generate(&mut self.map);
        add_vegetation(&mut self.maps[idx], reg, false);
    }

    pub fn gen_digger(&mut self, idx: usize, region: Option<&CustomRegion>) {
        let map_region = &self.maps[idx].get_region();
        let reg = if region != None {
            region.unwrap()
        } else {
            map_region
        };
        // num_features is approximate because depending on the room size and size of the map it
        // may not be possible to insert all features.
        // Biggers rooms are more aesthetically pleasing, but require a much greater map (from
        // 100x100 to 200x200) to have more features.
        // (min_size, max_size, num_features (approx)
        let mut digger = Digger::new(reg, 10, 15, 30);
        digger.generate(&mut self.maps[idx], &mut self.rng);
        self.rooms = Some(digger.get_rooms());
        add_doors(&mut self.maps[idx], self.rooms.as_ref(), 30, &mut self.rng);
    }

    pub fn gen_digger_inverted(&mut self, idx: usize, region: Option<&CustomRegion>) {
        self.gen_digger(idx, region);

        let map_region = &self.maps[idx].get_region();
        let reg = if region != None {
            region.unwrap()
        } else {
            map_region
        };

        // n_iterations, n_walls_rule, min_cave_size, open_halls, dry_caves
        let mut cell_automata = CellularAutomata::new(reg, 3, 7, 10, false, false);
        cell_automata.generate(&mut self.maps[idx]);
        if self.rng.range(0, 2) < 1 {
            add_vegetation(&mut self.maps[idx], reg, false);
        }
    }

    pub fn get_map(&self, idx: usize) -> Map {
        self.maps[idx].clone()
    }
}
