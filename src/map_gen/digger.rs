use super::{
    common::{create_h_tunnel_room, create_room, create_v_tunnel_room},
    room::Operations,
    Map, Room,
};
use crate::utils::directions::*;
use bracket_lib::prelude::{DistanceAlg, Point, RandomNumberGenerator};

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
}

#[allow(dead_code)]
impl Digger {
    pub fn new() -> Self {
        Self { rooms: vec![] }
    }

    pub fn generate(&mut self, map: &mut Map, rng: &mut RandomNumberGenerator) {
        // Create initial room somewhere on the map.
        let xi = map.width / 2 - map.width / 10;
        let yi = map.height / 3;
        let wi = rng.range(10, 20);
        let hi = wi;
        let initial_room = Room::with_size(xi, yi, wi, hi);
        create_room(map, initial_room);
        self.rooms.push(initial_room);
        self.gen_feature(map, rng);
    }

    fn add_feature(&mut self, map: &mut Map, room: Room, rng: &mut RandomNumberGenerator) -> bool {
        let w = rng.range(5, 10);
        let h = rng.range(w, 10);
        let dir = get_random_dir();
        let ndir = dir.clone();

        let mut pt = room.get_wall(map, dir);
        let room_gap = rng.range(3, 8);

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
        self.connect_rooms(map, room, new_room, rng);
        return true;
    }

    fn gen_feature(&mut self, map: &mut Map, rng: &mut RandomNumberGenerator) {
        let mut num_features = 0;
        let mut repeat = 100;
        let mut prev_idx = 0;
        while num_features <= 60 && repeat > 0 {
            repeat -= 1;
            for _i in num_features..60 {
                let idx = rng.range(0, self.rooms.len());
                if idx == prev_idx && self.rooms.len() > 1 {
                    repeat += 1;
                    continue;
                }
                prev_idx = idx;
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
        let borders1 = room1.get_borders(map);
        let borders2 = room2.get_borders(map);

        let mut room_c = Point::new(0, 0);
        let mut other_c = Point::new(0, 0);

        let mut shortest_dist = DistanceAlg::Pythagoras.distance2d(borders1[0], borders2[0]);
        for b1 in borders1.iter() {
            for b2 in borders2.iter() {
                let d = DistanceAlg::Pythagoras.distance2d(*b1, *b2);
                if d < shortest_dist {
                    room_c = *b1;
                    other_c = *b2;
                    shortest_dist = d;
                }
            }
        }

        let size = rng.range(1, 4);

        match rng.range(0, 2) {
            0 => {
                if room_c.x <= other_c.x {
                    self.rooms.push(create_h_tunnel_room(
                        map, room_c.x, other_c.x, room_c.y, size,
                    ));
                } else {
                    self.rooms.push(create_h_tunnel_room(
                        map, other_c.x, room_c.x, room_c.y, size,
                    ));
                }

                if room_c.y <= other_c.y {
                    self.rooms.push(create_v_tunnel_room(
                        map, room_c.y, other_c.y, other_c.x, size,
                    ));
                } else {
                    self.rooms.push(create_v_tunnel_room(
                        map, other_c.y, room_c.y, other_c.x, size,
                    ));
                }
            }
            _ => {
                if room_c.y <= other_c.y {
                    self.rooms.push(create_v_tunnel_room(
                        map, room_c.y, other_c.y, room_c.x, size,
                    ));
                } else {
                    self.rooms.push(create_v_tunnel_room(
                        map, other_c.y, room_c.y, room_c.x, size,
                    ));
                }

                if room_c.x <= other_c.x {
                    self.rooms.push(create_h_tunnel_room(
                        map, room_c.x, other_c.x, other_c.y, size,
                    ));
                } else {
                    self.rooms.push(create_h_tunnel_room(
                        map, other_c.x, room_c.x, other_c.y, size,
                    ));
                }
            }
        }
    }
}
