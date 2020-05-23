use super::{region::Operations, Map, Region, Room, Tile, TileType, Tunnel};
use crate::utils::directions::*;
use bracket_lib::prelude::{DistanceAlg, Point, RandomNumberGenerator};
use std::cmp;

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
pub fn create_h_tunnel(map: &mut Map, x1: i32, x2: i32, y: i32, size: i32) -> Tunnel {
    let mut tunnel = Vec::new();

    for x in cmp::min(x1, x2)..cmp::max(x1, x2) + 1 {
        let mut idx = map.idx(x, y);
        map.paint_tile(idx, TileType::Floor);
        tunnel.push(idx);
        if size > 1 {
            for i in 1..2 {
                idx = map.idx(x, y + i);
                map.paint_tile(idx, TileType::Floor);
                tunnel.push(idx);
            }
        }
    }

    tunnel
}

/// Creates a vertical tunnel and returns it.
#[allow(dead_code)]
pub fn create_v_tunnel(map: &mut Map, y1: i32, y2: i32, x: i32, size: i32) -> Tunnel {
    let mut tunnel = Vec::new();
    for y in cmp::min(y1, y2)..cmp::max(y1, y2) + 1 {
        let mut idx = map.idx(x, y);
        map.paint_tile(idx, TileType::Floor);
        tunnel.push(idx);
        if size > 1 {
            for i in 1..size {
                idx = map.idx(x + i, y);
                map.paint_tile(idx, TileType::Floor);
                tunnel.push(idx);
            }
        }
    }

    tunnel
}

#[allow(dead_code)]
pub fn create_h_tunnel_room(map: &mut Map, x1: i32, x2: i32, y: i32, size: i32) -> Room {
    let left = cmp::min(x1, x2);
    let right = cmp::max(x1, x2);
    let top = y - 1;
    let bottom = y + 1;
    let room = Room::with_size(left, top, right - left + size - 1, bottom - top + 1);
    create_room(map, room);
    room
}

#[allow(dead_code)]
pub fn create_v_tunnel_room(map: &mut Map, y1: i32, y2: i32, x: i32, size: i32) -> Room {
    let top = cmp::min(y1, y2);
    let bottom = cmp::max(y1, y2);
    let left = x - 1;
    let right = x + 1;
    let room = Room::with_size(left, top, right - left + size - 1, bottom - top + 1);
    create_room(map, room);
    room
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

    let x = rng.range(15, map.width - 15);
    let y = rng.range(15, map.height - 15);

    let mut walker_pos = Point::new(x, y);
    let mut n_tiles = 0;
    let mut max_tries = total_tiles * 5;

    while n_tiles <= total_tiles && max_tries > 0 {
        if map.in_map_bounds(walker_pos) {
            let idx = map.idx_pt(walker_pos);
            match liquid {
                TileType::DeepWater => {
                    map.tiles[idx] = Tile::deep_water();
                    map.tiles[idx+1] = Tile::deep_water();
                    map.tiles[idx-1] = Tile::deep_water();
                }
                _ => {
                    map.tiles[idx] = Tile::shallow_water();
                    map.tiles[idx+1] = Tile::shallow_water();
                    map.tiles[idx-1] = Tile::shallow_water();
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

/// Counts how many neighbor tiles of a given type curr_pt has.
/// If moore == true, considers a Moore neighborhood (ortoghonal+diagonals neighbors).
/// If moore == false, considers a von Neumann neighborhood (orthogonal neighbors).
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

/// Gets all the separated regions on a map.
pub fn get_all_regions(map: &Map) -> Vec<Region> {
    let w = map.width;
    let h = map.height;
    let mut caves: Vec<Region> = Vec::new();
    let mut marked_map: Vec<bool> = vec![false; map.size as usize];

    for y in 1..h - 1 {
        for x in 1..w - 1 {
            let idx = map.idx(x, y);
            if !marked_map[idx] && map.is_floor(idx) {
                let new_cave = get_region(idx, map);

                for idx in new_cave.iter() {
                    marked_map[*idx] = true;
                }

                caves.push(new_cave);
            }
        }
    }

    caves
}

/// Gets a single region from a map, given an initial index (flood fill).
pub fn get_region(start_idx: usize, map: &Map) -> Region {
    use std::collections::VecDeque;
    let mut region_tiles: Region = Vec::new();
    let mut marked_map: Vec<bool> = vec![false; map.size as usize];
    let mut queue: VecDeque<usize> = VecDeque::new();

    queue.push_back(start_idx);
    marked_map[start_idx] = true;

    while !queue.is_empty() {
        let tile = queue.pop_front().unwrap();
        region_tiles.push(tile);
        let pt = map.idx_pos(tile);
        for y in pt.y - 1..pt.y + 2 {
            for x in pt.x - 1..pt.x + 2 {
                let idx = map.idx(x, y);
                if map.in_map_bounds_xy(x, y) && (y == pt.y || x == pt.x) {
                    if !marked_map[idx] && map.is_floor(idx) {
                        marked_map[idx] = true;
                        queue.push_back(idx);
                    }
                }
            }
        }
    }

    region_tiles
}

/// Connects with tunnels the selected regions.
pub fn connect_regions(map: &mut Map, regions: Vec<Region>, ttype: TileType, natural: bool) {
    // Algorithm idea:
    // - get the two points (x, y) that are the closest between two caves
    // - make a tunnel between then
    let mut region_pts: Vec<Vec<Point>> = Vec::new();

    // Populate the vector cave_pts (same as before, but considering the
    // coordinates on the map instead of the index).
    for region in regions.iter() {
        let mut pts: Vec<Point> = Vec::new();
        for idx in region {
            let pt = map.idx_pos(*idx);
            if region.is_probably_edge(pt, map) {
                pts.push(pt);
            }
        }
        region_pts.push(pts);
    }

    for i in 0..region_pts.len() - 1 {
        let this_region = &region_pts[i];
        let other_region = &region_pts[i + 1];
        let mut shortest_dist = other_region.len();
        let mut this_idx = 0;
        let mut other_idx = 0;
        for j in 0..this_region.len() - 1 {
            for k in 0..other_region.len() - 1 {
                let d =
                    DistanceAlg::Pythagoras.distance2d(this_region[j], other_region[k]) as usize;
                if d < shortest_dist {
                    this_idx = j;
                    other_idx = k;
                    shortest_dist = d;
                }
            }
        }
        make_exact_tunnel(
            map,
            this_region[this_idx].x,
            this_region[this_idx].y,
            other_region[other_idx].x,
            other_region[other_idx].y,
            ttype,
            natural,
        );
    }
}
