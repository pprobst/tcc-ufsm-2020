use bracket_lib::prelude::{RandomNumberGenerator, RGB, to_cp437, WHITE, BLACK, Point};
use specs::prelude::*;

use super::{Position, Renderable, Player, Mob, Name, Fov, Blocker, Health, BaseStats};
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
        .with(Name { name: "Severian".to_string() })
        .with(Fov { range: 13, visible_pos: Vec::new(), dirty: true })
        .with(BaseStats{ health: Health { max_hp: 15, hp: 15 }, defense: 3, attack: 5, god: true})
        .build()
}

pub fn test_mob(ecs: &mut World, x: i32, y: i32) -> Entity {
    ecs
        .create_entity()
        .with(Position { x, y })
        .with(Renderable {
            glyph: to_cp437('t'),
            fg: RGB::from_hex("#ED3D3D").expect("Invalid hex string"),
            bg: RGB::named(BLACK),
        })
        .with(Mob{})
        .with(Name { name: "Test Mob".to_string() })
        .with(Fov { range: 13, visible_pos: Vec::new(), dirty: true })
        .with(Blocker{})
        .with(BaseStats{ health: Health { max_hp: 7, hp: 7 }, defense: 3, attack: 5, god: false})
        .build()
}

pub fn spawn_map(ecs: &mut World, map: &Map) {
    let idx = map.idx(map.width/2, map.height/2);
    let pt = map.idx_pos(idx);
    ecs.insert(Point::new(pt.x, pt.y));
    let player = player(ecs, pt.x, pt.y);
    ecs.insert(player);
    
    let mut rng = RandomNumberGenerator::new();

    for _i in 0..15 {
        let x = rng.roll_dice(1, map.width-2); 
        let y = rng.roll_dice(1, map.height-2); 
        let idx = map.idx(x, y);
        if !map.tiles[idx].block {
           test_mob(ecs, x, y);
        }
    }
}
