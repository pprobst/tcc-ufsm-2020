use super::{
    map_gen::Map, utils::colors::*, BaseStats, Blocker, Consumable, Fov, Health, Item, Mob, Name,
    Player, Position, Renderable,
};
use bracket_lib::prelude::{to_cp437, ColorPair, Point, RandomNumberGenerator, BLACK, RGB, WHITE};
use specs::prelude::*;

/*
 *
 * spawner.rs
 * ----------
 * Controls basic spawning of entities and inserts them into the ECS.
 *
 */

pub fn player(ecs: &mut World, x: i32, y: i32) -> Entity {
    ecs.create_entity()
        .with(Position { x, y })
        .with(Renderable {
            glyph: to_cp437('@'),
            color: ColorPair::new(RGB::named(WHITE), RGB::named(BLACK)),
            layer: 1,
        })
        .with(Player {})
        .with(Name {
            name: "Severian".to_string(),
        })
        .with(Fov {
            range: 10,
            visible_pos: Vec::new(),
            dirty: true,
        })
        .with(BaseStats {
            health: Health { max_hp: 15, hp: 10 },
            defense: 3,
            attack: 6,
            god: true,
        })
        .build()
}

pub fn test_mob(ecs: &mut World, x: i32, y: i32) -> Entity {
    ecs.create_entity()
        .with(Position { x, y })
        .with(Mob {})
        .with(Renderable {
            glyph: to_cp437('t'),
            color: ColorPair::new(RGB::from_hex(BLOOD_RED).unwrap(), RGB::named(BLACK)),
            layer: 1,
        })
        .with(Name {
            name: "Test Mob".to_string(),
        })
        .with(Fov {
            range: 10,
            visible_pos: Vec::new(),
            dirty: true,
        })
        .with(Blocker {})
        .with(BaseStats {
            health: Health { max_hp: 7, hp: 7 },
            defense: 3,
            attack: 5,
            god: false,
        })
        .build()
}

pub fn test_consumable(ecs: &mut World, x: i32, y: i32) {
    ecs.create_entity()
        .with(Position { x, y })
        .with(Renderable {
            glyph: to_cp437('!'),
            color: ColorPair::new(RGB::from_hex(MED_RED).unwrap(), RGB::named(BLACK)),
            layer: 0,
        })
        .with(Name {
            name: "Test Consumable".to_string(),
        })
        .with(Item {})
        .with(Consumable { heal: 5 })
        .build();
}

pub fn spawn_map(ecs: &mut World, map: &Map) {
    let idx = map.idx(map.width / 2 + 2, map.height / 2 + 2);
    let pt = map.idx_pos(idx);
    ecs.insert(Point::new(pt.x, pt.y));
    let player = player(ecs, pt.x, pt.y);
    ecs.insert(player);

    test_consumable(ecs, pt.x + 1, pt.y + 1);
    test_consumable(ecs, pt.x + 2, pt.y + 1);

    let mut rng = RandomNumberGenerator::new();

    for _i in 0..15 {
        let x = rng.roll_dice(1, map.width - 2);
        let y = rng.roll_dice(1, map.height - 2);
        let idx = map.idx(x, y);
        if !map.tiles[idx].block {
            test_mob(ecs, x, y);
        }
    }
}
