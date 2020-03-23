use bracket_lib::prelude::*;
use specs::prelude::*;

use super::{Position, Renderable, Player, Fov};
use crate::map_gen::Map;

pub fn player(ecs: &mut World, x: i32, y: i32) -> Entity {
    ecs
        .create_entity()
        .with(Position { x, y })
        .with(Renderable {
            glyph: to_cp437('@'),
            fg: RGB::named(WHITE),
            bg: RGB::named(BLACK),
        })
        .with(Player{})
        .with(Fov { range: 23, visible_pos: Vec::new(), dirty: true })
        .build()
}

pub fn spawn_map(ecs: &mut World, map: &Map) {
    let idx = map.idx(map.width/2, map.height/2);
    let pt = map.idx_pos(idx);
    ecs.insert(Point::new(pt.x, pt.y));
    let player = player(ecs, pt.x, pt.y);
    ecs.insert(player);
}
