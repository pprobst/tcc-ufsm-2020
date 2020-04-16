use super::{common::make_exact_tunnel, Map, Point, Tile, TileType};
use crate::utils::directions::*;
use bracket_lib::prelude::DistanceAlg;

/*
 *
 * cellular_automata.rs
 * --------------------
 * Cellular Automata cave generation.
 *
 * http://www.roguebasin.com/index.php?title=Cellular_Automata_Method_for_Generating_Random_Cave-Like_Levels
 * https://github.com/vurmux/urizen/blob/master/urizen/generators/dungeons/dungeon_cellular.py
 * https://github.com/SPIGS/Polymorph/blob/master/src/level_generation/cellular_automata.rs
 */

#[allow(dead_code)]
pub struct CellularAutomata {
    n_iterations: u8, // the more iterations we have, the smoother the map will be
    n_walls_rule: u8,
    min_cave_size: usize,
    open_halls: bool,
    dry_caves: bool,
}

#[allow(dead_code)]
impl CellularAutomata {
    pub fn new(
        n_iterations: u8,
        n_walls_rule: u8,
        min_cave_size: usize,
        open_halls: bool,
        dry_caves: bool,
    ) -> Self {
        Self {
            n_iterations,
            n_walls_rule,
            min_cave_size,
            open_halls,
            dry_caves,
        }
    }

    pub fn generate(&mut self, map: &mut Map) {
        let w = map.width - 1;
        let h = map.height - 1;

        // We need to make a clone here because the already replaced cells MUST NOT
        // affect the current cell.
        let mut tiles = map.tiles.clone();

        for _i in 0..self.n_iterations {
            for y in 1..h {
                for x in 1..w {
                    let mut flag = false;
                    let curr_pt = Point::new(x, y);
                    let curr_idx = map.idx(x, y);
                    let wall_counter = self.count_neighbor_tile(map, curr_pt, TileType::Wall, true);
                    let water_counter =
                        self.count_neighbor_tile(map, curr_pt, TileType::ShallowWater, true);
                    if wall_counter >= self.n_walls_rule || (wall_counter == 0 && !self.open_halls)
                    {
                        tiles[curr_idx] = Tile::wall();
                        flag = true;
                    }
                    if water_counter > 2 && water_counter < 4 {
                        tiles[curr_idx] = Tile::shallow_water();
                        flag = true;
                    }
                    if water_counter >= 5 {
                        tiles[curr_idx] = Tile::deep_water();
                        flag = true;
                    }
                    if flag == false {
                        tiles[curr_idx] = Tile::floor();
                    }
                }
            }
        }

        map.tiles = tiles.clone();
        //self.smooth_map(map);

        let mut main_caves = self.get_all_caves(map);
        let mut lesser_caves = main_caves.clone();

        // Get caves < min_cave_size.
        lesser_caves.retain(|a| a.len() < self.min_cave_size);

        // Get caves >= min_cave_size
        main_caves.retain(|a| a.len() >= self.min_cave_size);
        main_caves.sort_by(|a, b| b.len().cmp(&a.len()));

        for cave in lesser_caves {
            if self.dry_caves {
                self.fill_cave(map, cave, TileType::Wall);
            } else {
                self.fill_cave(map, cave, TileType::ShallowWater);
            }
        }

        if main_caves.len() > 2 && !self.dry_caves {
            let last_main_cave = main_caves[main_caves.len() - 1].clone();
            self.fill_cave(map, last_main_cave, TileType::ShallowWater);
        }

        main_caves.sort_by(|a, b| a[0].cmp(&b[0]));
        self.connect_caves(map, main_caves);
        self.smooth_map(map);
    }

    fn count_neighbor_tile(&self, map: &mut Map, curr_pt: Point, tt: TileType, moore: bool) -> u8 {
        let mut wall_counter = 0;

        /*if map.tiles[map.idx_pt(curr_pt)].ttype == tt {
            wall_counter += 1;
        } // avoid many single tile blockers
        */
        if map.tiles[map.idx_pt(curr_pt + EAST)].ttype == tt {
            wall_counter += 1;
        }
        if map.tiles[map.idx_pt(curr_pt + WEST)].ttype == tt {
            wall_counter += 1;
        }
        if map.tiles[map.idx_pt(curr_pt + NORTH)].ttype == tt {
            wall_counter += 1;
        }
        if map.tiles[map.idx_pt(curr_pt + SOUTH)].ttype == tt {
            wall_counter += 1;
        }
        if moore {
            if map.tiles[map.idx_pt(curr_pt + NORTHEAST)].ttype == tt {
                wall_counter += 1;
            }
            if map.tiles[map.idx_pt(curr_pt + NORTHWEST)].ttype == tt {
                wall_counter += 1;
            }
            if map.tiles[map.idx_pt(curr_pt + SOUTHEAST)].ttype == tt {
                wall_counter += 1;
            }
            if map.tiles[map.idx_pt(curr_pt + SOUTHWEST)].ttype == tt {
                wall_counter += 1;
            }
        }

        wall_counter
    }

    fn smooth_map(&self, map: &mut Map) {
        let mut tiles = map.tiles.clone();

        for _i in 0..self.n_iterations {
            for y in 1..map.height - 1 {
                for x in 1..map.width - 1 {
                    let curr_pt = Point::new(x, y);
                    let curr_idx = map.idx(x, y);
                    if !map.is_water(curr_idx) {
                        let wall_counter =
                            self.count_neighbor_tile(map, curr_pt, TileType::Wall, false);
                        let water_counter =
                            self.count_neighbor_tile(map, curr_pt, TileType::ShallowWater, false);
                        let deep_counter =
                            self.count_neighbor_tile(map, curr_pt, TileType::DeepWater, false);
                        if wall_counter <= 1 {
                            tiles[curr_idx] = Tile::floor();
                        }
                        if water_counter >= 2 || deep_counter >= 1 {
                            tiles[curr_idx] = Tile::shallow_water();
                        }
                    }
                }
            }
        }

        map.tiles = tiles;
    }

    /// Connect with tunnels the caves that have >= than the minimum size.
    fn connect_caves(&self, map: &mut Map, caves: Vec<Vec<usize>>) {
        // Algorithm idea:
        // - get the two points (x, y) that are the closest between two caves
        // - make a tunnel between then
        let mut cave_pts: Vec<Vec<Point>> = Vec::new();

        // Populate the vector cave_pts (same as before, but considering the
        // coordinates on the map instead of the index).
        for cave in caves {
            let mut pts: Vec<Point> = Vec::new();
            for idx in cave {
                let pt = map.idx_pos(idx);
                if self.is_probably_edge(pt, map) {
                    pts.push(pt);
                }
            }
            cave_pts.push(pts);
        }

        for i in 0..cave_pts.len() - 1 {
            let this_cave = &cave_pts[i];
            let other_cave = &cave_pts[i + 1];
            let mut shortest_dist = other_cave.len();
            let mut this_idx = 0;
            let mut other_idx = 0;
            for j in 0..this_cave.len() - 1 {
                for k in 0..other_cave.len() - 1 {
                    let d =
                        DistanceAlg::Pythagoras.distance2d(this_cave[j], other_cave[k]) as usize;
                    if d < shortest_dist {
                        this_idx = j;
                        other_idx = k;
                        shortest_dist = d;
                    }
                }
            }
            make_exact_tunnel(
                map,
                this_cave[this_idx].x,
                this_cave[this_idx].y,
                other_cave[other_idx].x,
                other_cave[other_idx].y,
                TileType::Floor,
                true,
            );
        }
    }

    /// Returns true if the point is probably an edge of a region.
    /// While not 100% accurate (it detects blockers not only on edges),
    /// it cuts our distance computations by a lot!
    fn is_probably_edge(&self, pt: Point, map: &mut Map) -> bool {
        let east = pt + EAST;
        let west = pt + WEST;
        let north = pt + NORTH;
        let south = pt + SOUTH;

        if map.in_map_bounds(east) && map.tiles[map.idx_pt(east)].block {
            return true;
        }
        if map.in_map_bounds(west) && map.tiles[map.idx_pt(west)].block {
            return true;
        }
        if map.in_map_bounds(north) && map.tiles[map.idx_pt(north)].block {
            return true;
        }
        if map.in_map_bounds(south) && map.tiles[map.idx_pt(south)].block {
            return true;
        }

        return false;
    }

    /// Fill with wall tiles the caves that have < than the minimum size.
    // idea: maybe fill them with water tiles for a nice twist?
    fn fill_cave(&self, map: &mut Map, cave: Vec<usize>, ttype: TileType) {
        for idx in cave {
            map.paint_tile(idx, ttype);
        }
    }

    /// Gets a list of all separated caves on the map.
    fn get_all_caves(&self, map: &mut Map) -> Vec<Vec<usize>> {
        let w = map.width;
        let h = map.height;
        let mut caves: Vec<Vec<usize>> = Vec::new();
        let mut marked_map: Vec<bool> = vec![false; map.size as usize];

        for y in 1..h - 1 {
            for x in 1..w - 1 {
                let idx = map.idx(x, y);
                if !marked_map[idx] && map.tiles[idx].ttype == TileType::Floor {
                    let new_cave = self.get_cave(idx, map);

                    for idx in new_cave.iter() {
                        marked_map[*idx] = true;
                    }

                    caves.push(new_cave);
                }
            }
        }

        caves
    }

    /// Gets a cave using the flood-fill algorithm.
    fn get_cave(&self, start_idx: usize, map: &mut Map) -> Vec<usize> {
        use std::collections::VecDeque;
        let mut cave_tiles: Vec<usize> = Vec::new();
        let mut marked_map: Vec<bool> = vec![false; map.size as usize];
        let mut queue: VecDeque<usize> = VecDeque::new();

        queue.push_back(start_idx);
        marked_map[start_idx] = true;

        while !queue.is_empty() {
            let tile = queue.pop_front().unwrap();
            cave_tiles.push(tile);
            let pt = map.idx_pos(tile);
            for y in pt.y - 1..pt.y + 2 {
                for x in pt.x - 1..pt.x + 2 {
                    let idx = map.idx(x, y);
                    if map.in_map_bounds_xy(x, y) && (y == pt.y || x == pt.x) {
                        if !marked_map[idx] && map.tiles[idx].ttype == TileType::Floor {
                            marked_map[idx] = true;
                            queue.push_back(idx);
                        }
                    }
                }
            }
        }

        cave_tiles
    }
}
