use super::{Map, Room, Tile, TileType};
use crate::utils::directions::*;
use bracket_lib::prelude::{Point, RandomNumberGenerator};

/*
 *
 * common.rs
 * ---------
 * Contains some general code that can be used by various map generators.
 * https://github.com/Vinatorul/dungeon-generator-rs/blob/master/src/bsp_generator.rs
 *
 */

/// Creates a rectangular room and returns it.
#[allow(dead_code)]
pub fn create_room(map: &mut Map, room: Room) -> Room {
    for x in (room.x1 + 1)..room.x2 {
        for y in (room.y1 + 1)..room.y2 {
            let idx = map.idx(x, y);
            map.paint_tile(idx, TileType::Floor);
        }
    }

    room
}

/// Creates a horizontal tunnel (corridor) and returns it.
#[allow(dead_code)]
pub fn create_h_tunnel(map: &mut Map, x1: i32, x2: i32, y: i32) -> Vec<usize> {
    let mut tunnel = Vec::new();

    for x in x1.min(x2)..(x1.max(x2) + 1) {
        let idx = map.idx(x, y);
        map.paint_tile(idx, TileType::Floor);
        tunnel.push(idx);
    }

    tunnel
}

/// Creates a vertical tunnel and returns it.
#[allow(dead_code)]
pub fn create_v_tunnel(map: &mut Map, y1: i32, y2: i32, x: i32) -> Vec<usize> {
    let mut tunnel = Vec::new();

    for y in y1.min(y2)..(y1.max(y2) + 1) {
        let idx = map.idx(x, y);
        map.paint_tile(idx, TileType::Floor);
        tunnel.push(idx);
    }

    tunnel
}

#[allow(dead_code)]
pub fn make_exact_tunnel(
    map: &mut Map,
    x1: i32,
    y1: i32,
    x2: i32,
    y2: i32,
    ttype: TileType,
    natural: bool,
) {
    let mut x = x1;
    let mut y = y1;

    while x != x2 || y != y2 {
        if x < x2 {
            x += 1;
        } else if x > x2 {
            x -= 1;
        } else if y < y2 {
            y += 1;
        } else if y > y2 {
            y -= 1;
        }

        let idx = map.idx(x, y);
        if map.tiles[idx].ttype != TileType::ShallowWater
            && map.tiles[idx].ttype != TileType::DeepWater
        {
            map.paint_tile(idx, ttype);

            if natural {
                let mut rng = RandomNumberGenerator::new();
                let sign_x = rng.range(0, 3);
                let sign_y = rng.range(0, 3);
                let add_x = if sign_x < 1 { 1 } else { -1 };
                let add_y = if sign_y < 1 { 1 } else { -1 };
                if map.in_map_bounds_xy(x + add_x, y + add_y) {
                    let mut idx2 = map.idx(x + add_x, y + add_y);
                    if map.tiles[idx2].ttype != TileType::ShallowWater
                        && map.tiles[idx2].ttype != TileType::DeepWater
                    {
                        map.paint_tile(idx2, ttype);
                        let one_more = rng.range(0, 5);
                        if one_more < 1 && map.in_map_bounds_xy(x + (add_x * 2), y + (add_y * 2)) {
                            idx2 = map.idx(x + (add_x * 2), y + (add_y) * 2);
                            if map.tiles[idx2].ttype != TileType::ShallowWater
                                && map.tiles[idx2].ttype != TileType::DeepWater
                            {
                                map.paint_tile(idx2, ttype);
                            }
                        }
                    }
                }
            }
        }
    }
}

#[allow(dead_code)]
pub fn make_lake(map: &mut Map, liquid: TileType, total_tiles: u32) {
    let mut rng = RandomNumberGenerator::new();

    let x = rng.range(10, map.width - 10);
    let y = rng.range(10, map.height - 10);

    let mut walker_pos = Point::new(x, y);
    let mut n_tiles = 0;
    let mut max_tries = total_tiles * 2;

    while n_tiles <= total_tiles && max_tries > 0 {
        if map.in_map_bounds(walker_pos) {
            let idx = map.idx_pt(walker_pos);
            match liquid {
                TileType::DeepWater => {
                    map.tiles[idx] = Tile::deep_water();
                }
                _ => {
                    map.tiles[idx] = Tile::shallow_water();
                }
            }
            let dir = rng.range(0, 4);
            match dir {
                0 => {
                    walker_pos += EAST;
                }
                1 => {
                    walker_pos += WEST;
                }
                2 => {
                    walker_pos += NORTH;
                }
                _ => {
                    walker_pos += SOUTH;
                }
            }
            n_tiles += 1;
        }
        max_tries -= 1;
    }
}

pub fn count_neighbor_tile(map: &Map, curr_pt: Point, tt: TileType, moore: bool) -> u8 {
    let mut counter = 0;

    /*if map.tiles[map.idx_pt(curr_pt)].ttype == tt {
        wall_counter += 1;
    } // avoid many single tile blockers
    */
    if map.tiles[map.idx_pt(curr_pt + EAST)].ttype == tt {
        counter += 1;
    }
    if map.tiles[map.idx_pt(curr_pt + WEST)].ttype == tt {
        counter += 1;
    }
    if map.tiles[map.idx_pt(curr_pt + NORTH)].ttype == tt {
        counter += 1;
    }
    if map.tiles[map.idx_pt(curr_pt + SOUTH)].ttype == tt {
        counter += 1;
    }
    if moore {
        if map.tiles[map.idx_pt(curr_pt + NORTHEAST)].ttype == tt {
            counter += 1;
        }
        if map.tiles[map.idx_pt(curr_pt + NORTHWEST)].ttype == tt {
            counter += 1;
        }
        if map.tiles[map.idx_pt(curr_pt + SOUTHEAST)].ttype == tt {
            counter += 1;
        }
        if map.tiles[map.idx_pt(curr_pt + SOUTHWEST)].ttype == tt {
            counter += 1;
        }
    }

    counter
}

#[allow(dead_code)]
pub fn add_vegetation(map: &mut Map) {
    let mut rng = RandomNumberGenerator::new();
    for y in 1..map.height - 1 {
        for x in 1..map.width - 1 {
            let idx = map.idx(x, y);
            if !map.tiles[idx].block && !map.is_water(idx) {
                let mut chance = rng.range(0, 4);
                if chance < 2 {
                    let pt = map.idx_pos(idx);
                    let water_counter = count_neighbor_tile(map, pt, TileType::ShallowWater, false);
                    if water_counter >= 1 {
                        map.tiles[idx] = Tile::tallgrass();
                    } else {
                        chance = rng.range(0, 60);
                        if chance < 59 {
                            map.tiles[idx] = Tile::grass();
                        } else {
                            map.tiles[idx] = Tile::flower();
                        }
                    }
                }
            }
        }
    }
}
