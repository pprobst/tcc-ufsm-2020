use super::{
    common::{create_h_tunnel, create_room, create_v_tunnel},
    room::Operations,
    Map, Room,
};
use crate::utils::directions::*;
use bracket_lib::prelude::RandomNumberGenerator;

/*
 *
 * digger.rs
 * ---------
 * The digger/tunneler algorithm, based on:
 * http://www.roguebasin.com/index.php?title=Dungeon-Building_Algorithm
 *
 */

#[allow(dead_code)]
pub struct Digger {
    rooms: Vec<Room>,
    tunnels: Vec<Vec<usize>>,
}

#[allow(dead_code)]
impl Digger {
    pub fn new() -> Self {
        Self {
            rooms: vec![],
            tunnels: vec![],
        }
    }

    pub fn generate(&mut self, map: &mut Map, rng: &mut RandomNumberGenerator) {
        // Create initial room somewhere on the map.
        let xi = map.width / 2 - map.width / 10;
        let yi = map.height / 3;
        let wi = rng.range(15, 20);
        let hi = wi;
        let initial_room = Room::with_size(xi, yi, wi, hi);
        let idx = map.idx(initial_room.center().x, initial_room.center().y);
        map.tiles[idx].change_glyph('#');
        create_room(map, initial_room);
        self.rooms.push(initial_room);

        self.gen_feature(map, rng);

        /*
        self.rooms.sort_by(|a, b| a.x1.cmp(&b.x1));

        for i in 0 .. self.rooms.len()-1 {
            let this_room = self.rooms[i];
            let other_room = self.rooms[i+1];
            self.connect_rooms(map, this_room, other_room, rng);
        }
        */
    }

    fn add_feature(&mut self, map: &mut Map, room: Room, rng: &mut RandomNumberGenerator) -> bool {
        let w = rng.range(5, 15);
        //let h = rng.range(5, 15);
        let h = w;
        let dir = get_random_dir();
        let ndir = dir.clone();

        let mut pt = room.get_wall(dir);
        let room_gap = rng.range(1, 4);

        match ndir {
            NORTH => {
                pt.y -= h + room_gap;
            }
            EAST => {
                pt.x += room_gap;
            }
            SOUTH => {
                pt.y += room_gap;
            }
            _ => {
                pt.x -= w + room_gap;
            }
        }

        let new_room = Room::with_size(pt.x, pt.y, w, h);

        for r in self.rooms.iter() {
            if new_room.intersect(r)
                || !map.in_map_bounds_xy(new_room.x1, new_room.y1)
                || !map.in_map_bounds_xy(new_room.x2, new_room.y2)
            {
                return false;
            }
        }

        self.rooms.push(new_room);
        create_room(map, new_room);
        self.connect_rooms(map, new_room, room, rng);
        return true;
    }

    fn gen_feature(&mut self, map: &mut Map, rng: &mut RandomNumberGenerator) {
        let mut num_features = 0;
        let mut repeat = 300;
        while num_features <= 100 && repeat > 0 {
            repeat -= 1;
            for _i in num_features..100 + 1 {
                let idx = rng.range(0, self.rooms.len());
                if self.add_feature(map, self.rooms[idx], rng) {
                    num_features += 1;
                }
            }
        }
    }

    fn connect_rooms(
        &mut self,
        map: &mut Map,
        room1: Room,
        room2: Room,
        rng: &mut RandomNumberGenerator,
    ) {
        let room_c = room2.center();
        let other_c = room1.center();

        match rng.range(0, 3) {
            0 => {
                if room_c.x <= other_c.x {
                    self.tunnels
                        .push(create_h_tunnel(map, room_c.x, other_c.x, room_c.y));
                } else {
                    self.tunnels
                        .push(create_h_tunnel(map, other_c.x, room_c.x, room_c.y));
                }

                if room_c.y <= other_c.y {
                    self.tunnels
                        .push(create_v_tunnel(map, room_c.y, other_c.y, other_c.x));
                } else {
                    self.tunnels
                        .push(create_v_tunnel(map, other_c.y, room_c.y, other_c.x));
                }
            }
            _ => {
                if room_c.y <= other_c.y {
                    self.tunnels
                        .push(create_v_tunnel(map, room_c.y, other_c.y, other_c.x));
                } else {
                    self.tunnels
                        .push(create_v_tunnel(map, other_c.y, room_c.y, other_c.x));
                }

                if room_c.x <= other_c.x {
                    self.tunnels
                        .push(create_h_tunnel(map, room_c.x, other_c.x, room_c.y));
                } else {
                    self.tunnels
                        .push(create_h_tunnel(map, other_c.x, room_c.x, room_c.y));
                }
            }
        }
    }
}
