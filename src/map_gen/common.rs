use bracket_lib::prelude::RandomNumberGenerator;
use super::{Map, TileType, Room};

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

    for y in y1.min(y2) .. (y1.max(y2) + 1) {
        let idx = map.idx(x, y);
        map.paint_tile(idx, TileType::Floor);
        tunnel.push(idx);
    }

    tunnel
}

#[allow(dead_code)]
pub fn make_exact_tunnel(map: &mut Map, x1: i32, y1: i32, x2: i32, y2: i32, ttype: TileType, natural: bool) {
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
        if map.tiles[idx].ttype != TileType::ShallowWater && map.tiles[idx].ttype != TileType::DeepWater {
            map.paint_tile(idx, ttype);

            if natural {
                let mut rng = RandomNumberGenerator::new();
                let sign_x = rng.range(0, 2);
                let sign_y = rng.range(0, 2);
                let add_x = if sign_x < 1 { 1 } else { -1 };
                let add_y = if sign_y < 1 { 1 } else { -1 };
                let mut idx2 = map.idx(x+add_x, y+add_y);
                if map.tiles[idx2].ttype != TileType::ShallowWater && map.tiles[idx2].ttype != TileType::DeepWater {
                    if map.in_map_bounds_xy(x+add_x, y+add_y) {
                        map.paint_tile(idx2, ttype);
                        let one_more = rng.range(0, 4);
                        idx2 = map.idx(x+(add_x*2), y+(add_y)*2);
                        if map.tiles[idx2].ttype != TileType::ShallowWater && map.tiles[idx2].ttype != TileType::DeepWater {
                            if one_more < 1 && map.in_map_bounds_xy(x+(add_x*2), y+(add_y*2)) {
                                idx2 = map.idx(x+(add_x*2), y+(add_y)*2);
                                map.paint_tile(idx2, ttype);
                            }
                        }
                    }
                }
            }
        }
    }
}
