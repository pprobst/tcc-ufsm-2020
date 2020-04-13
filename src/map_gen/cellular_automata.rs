use super::{Map, Tile, TileType, Point};
use crate::utils::directions::*;

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
    pub n_iterations: u8, // the more iterations we have, the smoother the map will be
    pub n_walls_rule: u8,
    pub min_cave_size: usize,
    pub open_halls: bool
}

#[allow(dead_code)]
impl CellularAutomata {
    pub fn new(n_iterations: u8, n_walls_rule: u8, min_cave_size: usize, open_halls: bool) -> Self {
        Self {
            n_iterations, n_walls_rule, min_cave_size, open_halls
        }
    }

    pub fn generate(&mut self, map: &mut Map) {
        let w = map.width-1;
        let h = map.height-1;

        // We need to make a clone here because the already replaced cells MUST NOT
        // affect the current cell.
        let mut tiles = map.tiles.clone();

        for _i in 0 .. self.n_iterations {
            for y in 1 .. h {
                for x in 1 .. w {
                    let mut wall_counter = 0;
                    let curr_pt = Point::new(x, y);
                    let curr_idx = map.idx(x, y);
                    // Moore neighborhood.
                    if map.tiles[map.idx_pt(curr_pt)].block { wall_counter += 1; }
                    if map.tiles[map.idx_pt(curr_pt + EAST)].block { wall_counter += 1; }
                    if map.tiles[map.idx_pt(curr_pt + WEST)].block { wall_counter += 1; }
                    if map.tiles[map.idx_pt(curr_pt + NORTH)].block { wall_counter += 1; }
                    if map.tiles[map.idx_pt(curr_pt + SOUTH)].block { wall_counter += 1; }
                    if map.tiles[map.idx_pt(curr_pt + NORTHEAST)].block { wall_counter += 1; }
                    if map.tiles[map.idx_pt(curr_pt + NORTHWEST)].block { wall_counter += 1; }
                    if map.tiles[map.idx_pt(curr_pt + SOUTHEAST)].block { wall_counter += 1; }
                    if map.tiles[map.idx_pt(curr_pt + SOUTHWEST)].block { wall_counter += 1; }

                    if wall_counter >= self.n_walls_rule || (wall_counter == 0 && !self.open_halls) { 
                        tiles[curr_idx] = Tile::wall();
                    } else { 
                        tiles[curr_idx] = Tile::floor(); 
                    }
                }
            }
        }

        map.tiles = tiles.clone();

        // TODO 
        // - [x] Get list of all caves.
        // - [ ] Connect all caves.
        let mut main_caves = self.get_all_caves(map);
        let mut lesser_caves = main_caves.clone();

        // Get caves < min_cave_size.
        lesser_caves.retain(|a| a.len() < self.min_cave_size);

        // Get caves >= min_cave_size and sort them by size (decresc.).
        main_caves.retain(|a| a.len() >= self.min_cave_size);
        main_caves.sort_by(|a, b| b.len().cmp(&a.len()));

        self.fill_caves(map, lesser_caves);
        self.connect_caves(map, main_caves);
    }

    /// Connect with tunnels the caves that have >= than the minimum size.
    fn connect_caves(&self, map: &mut Map, caves: Vec<Vec<usize>>) {
        // Algorithm idea:
        // - get the two points (x, y) that are the closest between two caves
        // - make a tunnel between then
        let mut cave_pts: Vec<Vec<Point>> = Vec::new();
        let mut pts: Vec<Point> = Vec::new();

        // Populate the vector cave_pts (same as before, but considering the
        // coordinates on the map instead of the index).
        for cave in caves {
            for idx in cave {
                let pt = map.idx_pos(idx);
                if self.is_probably_edge(pt, map) {
                    map.tiles[idx].shadowed();
                    // TODO: continue this.
                    //pts.push(pt);
                }
            }
            //cave_pts.push(pts);
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

        if map.in_map_bounds(east) && map.tiles[map.idx_pt(east)].block { return true; }
        if map.in_map_bounds(west) && map.tiles[map.idx_pt(west)].block { return true; }
        if map.in_map_bounds(north) && map.tiles[map.idx_pt(north)].block { return true; }
        if map.in_map_bounds(south) && map.tiles[map.idx_pt(south)].block { return true; }

        return false;
    }

    /// Fill with wall tiles the caves that have < than the minimum size.
    // idea: maybe fill them with water tiles for a nice twist?
    fn fill_caves(&self, map: &mut Map, caves: Vec<Vec<usize>>) {
        for cave in caves {
            for idx in cave {
                map.tiles[idx] = Tile::wall();
            }
        }
    }

    /// Gets a list of all separated caves on the map.
    fn get_all_caves(&self, map: &mut Map) -> Vec<Vec<usize>> {
        let w = map.width;
        let h = map.height;
        let mut caves: Vec<Vec<usize>> = Vec::new();
        let mut marked_map: Vec<bool> = vec![false; map.size as usize];

        for y in 1 .. h-1 {
            for x in 1 .. w-1 {
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
            for y in pt.y-1 .. pt.y+2 {
                for x in pt.x-1 .. pt.x+2 {
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
