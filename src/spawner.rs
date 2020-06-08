use super::{
    map_gen::Map, utils::colors::*, BaseStats, Blocker, Consumable, EquipSlot, Equipable, Fov,
    Health, Item, MeleeWeapon, Mob, Name, Player, Position, Renderable, Container,
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
            range: 20,
            visible_pos: Vec::new(),
            dirty: true,
        })
        .with(BaseStats {
            health: Health { max_hp: 15, hp: 2 },
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
            range: 20,
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

pub fn test_consumable(ecs: &mut World, x: i32, y: i32) -> Entity {
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
        .build()
}

pub fn test_sword(ecs: &mut World, x: i32, y: i32) -> Entity {
    ecs.create_entity()
        .with(Position { x, y })
        .with(Renderable {
            glyph: to_cp437('/'),
            color: ColorPair::new(RGB::from_hex(SWORD_GRAY).unwrap(), RGB::named(BLACK)),
            layer: 0,
        })
        .with(Name {
            name: "Terminus Est".to_string(),
        })
        .with(Item {})
        .with(Equipable {
            slot: EquipSlot::Weapon1,
        })
        .with(MeleeWeapon { base_damage: 5 })
        .build()
}

pub fn test_container(ecs: &mut World, x: i32, y: i32) {
    ecs.create_entity()
        .with(Position { x, y })
        .with(Renderable {
            glyph: to_cp437('Æ'),
            color: ColorPair::new(RGB::from_hex(CHEST_BROWN).unwrap(), RGB::named(BLACK)),
            layer: 1,
        })
        .with(Name {
            name: "Chest".to_string(),
        })
        .with(Blocker {})
        .with(Container {})
        .build();
}

// TODO
pub fn spawn_items_in_chests(ecs: &mut World) {

}

pub fn spawn_map(ecs: &mut World, map: &Map) {
    let idx = map.idx(map.width / 2 + 2, map.height / 2 + 2);
    let pt = map.idx_pos(idx);
    ecs.insert(Point::new(pt.x, pt.y));
    let player = player(ecs, pt.x, pt.y);
    ecs.insert(player);

    test_consumable(ecs, pt.x + 1, pt.y + 1);
    test_consumable(ecs, pt.x + 2, pt.y + 1);
    test_container(ecs, pt.x + 3, pt.y + 1);
    test_sword(ecs, pt.x + 2, pt.y + 2);

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
