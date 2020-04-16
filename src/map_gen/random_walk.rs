use super::{Map, Position, Tile, TileType};
use crate::utils::directions::*;
use bracket_lib::prelude::RandomNumberGenerator;

/*
 *
 * random_walk.rs
 * --------------
 * Random Walk algorithm, with some nice additions.
 *
 * http://www.roguebasin.com/index.php?title=Random_Walk_Cave_Generation
 *
 */

#[derive(Clone)]
struct Walker {
    size: i32,
    life: i32,
    pos: Position,
}

// percent >= 0.4 if walkers start on random positions.
// percent <= 0.25 if walkers start on the center.
// Walkers that can only move orthogonally produce neater dungeons,
// while allowing diagonal movement produces chaotic dungeons.
#[allow(dead_code)]
pub struct RandomWalker {
    percent: f32,
    grouped_walkers: bool,
    can_walk_diagonally: bool,
}

#[allow(dead_code)]
impl RandomWalker {
    pub fn new(percent: f32, grouped_walkers: bool, can_walk_diagonally: bool) -> Self {
        Self {
            percent,
            grouped_walkers,
            can_walk_diagonally,
        }
    }
    pub fn generate(&mut self, map: &mut Map, rng: &mut RandomNumberGenerator) {
        let w = map.width - 1;
        let h = map.height - 1;

        let mut n_floor_tiles = map
            .tiles
            .iter()
            .filter(|tile| tile.ttype == TileType::Floor)
            .count();
        let needed_floor_tiles = (self.percent * map.size as f32) as usize;

        let mut _n_walkers = 0;
        // While insufficient cells have been turned into floor, take one step in a random direction.
        // If the new map cell is wall, turn the new map cell into floor and increment the count of floor tiles.
        while n_floor_tiles < needed_floor_tiles {
            let mut walker;
            if self.grouped_walkers {
                walker = Walker {
                    size: rng.range(1, 3),
                    life: rng.range(200, 500),
                    pos: Position::new(w / 2, h / 2),
                };
            } else {
                walker = Walker {
                    size: rng.range(1, 3),
                    life: rng.range(200, 500),
                    pos: Position::new(rng.range(2, w - 1), rng.range(2, h - 1)),
                };
            }
            _n_walkers += 1;
            while walker.life > 0 {
                let idx = map.idx(walker.pos.x, walker.pos.y);
                if map.in_map_bounds(walker.pos) {
                    let new_dir = rng.range(0, 8);
                    match new_dir {
                        0 => {
                            walker.pos += EAST;
                        }
                        1 => {
                            walker.pos += WEST;
                        }
                        2 => {
                            walker.pos += NORTH;
                        }
                        3 => {
                            walker.pos += SOUTH;
                        }
                        4 => {
                            if self.can_walk_diagonally {
                                walker.pos += NORTHEAST;
                            }
                        }
                        5 => {
                            if self.can_walk_diagonally {
                                walker.pos += NORTHWEST;
                            }
                        }
                        6 => {
                            if self.can_walk_diagonally {
                                walker.pos += SOUTHEAST;
                            }
                        }
                        _ => {
                            if self.can_walk_diagonally {
                                walker.pos += SOUTHWEST;
                            }
                        }
                    }
                    if map.tiles[idx].ttype == TileType::Wall {
                        map.tiles[idx] = Tile::floor();
                        n_floor_tiles += 1;
                    }
                }
                walker.life -= 1;
            }
        }
        //println!("Total walkers: {}", _n_walkers);
    }
}
