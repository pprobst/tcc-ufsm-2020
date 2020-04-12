use super::{Map, Tile, Room};

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
            make_floor(map, idx);
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
        make_floor(map, idx);
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
        make_floor(map, idx);
        tunnel.push(idx);
    }

    tunnel
}

/// Makes the given index on the map a Floor tile.
fn make_floor(map: &mut Map, idx: usize) {
    map.tiles[idx] = Tile::floor();
}
