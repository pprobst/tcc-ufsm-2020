use super::{Map, Tile, Point};
use crate::utils::directions::*;

/*
 *
 * cellular_automata.rs
 * -------------
 * Cellular Automata cave generation.
 * http://www.roguebasin.com/index.php?title=Cellular_Automata_Method_for_Generating_Random_Cave-Like_Levels
 * https://github.com/vurmux/urizen/blob/master/urizen/generators/dungeons/dungeon_cellular.py
 */

#[allow(dead_code)]
pub struct CellularAutomata { 
    pub n_iterations: u8,
    pub n_walls_rule: u8
}

#[allow(dead_code)]
impl CellularAutomata {
    pub fn new(n_iterations: u8, n_walls_rule: u8) -> Self {
        Self {
            n_iterations, n_walls_rule
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
                    // Moore neighbourhood.
                    if map.tiles[map.idx_pt(curr_pt)].block { wall_counter += 1; }
                    if map.tiles[map.idx_pt(curr_pt + EAST)].block { wall_counter += 1; }
                    if map.tiles[map.idx_pt(curr_pt + WEST)].block { wall_counter += 1; }
                    if map.tiles[map.idx_pt(curr_pt + NORTH)].block { wall_counter += 1; }
                    if map.tiles[map.idx_pt(curr_pt + SOUTH)].block { wall_counter += 1; }
                    if map.tiles[map.idx_pt(curr_pt + NORTHEAST)].block { wall_counter += 1; }
                    if map.tiles[map.idx_pt(curr_pt + NORTHWEST)].block { wall_counter += 1; }
                    if map.tiles[map.idx_pt(curr_pt + SOUTHEAST)].block { wall_counter += 1; }
                    if map.tiles[map.idx_pt(curr_pt + SOUTHWEST)].block { wall_counter += 1; }

                    //if wall_counter >= self.n_walls_rule || wall_counter < 1 { 
                    if wall_counter >= self.n_walls_rule { 
                        tiles[curr_idx] = Tile::wall();
                    } else { 
                        tiles[curr_idx] = Tile::floor(); 
                    }
                }
            }
        }
        map.tiles = tiles.clone();
    }
}
