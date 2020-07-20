use super::{
    map_gen::Map, raws::*, utils::colors::*, BaseStats, Blocker, Consumable, Contained, Container,
    EquipSlot, Equipable, Fov, Health, InventoryCapacity, Item, MeleeWeapon, Mob, Name, Player,
    Position, Renderable,
};
use bracket_lib::prelude::{to_cp437, ColorPair, Point, RandomNumberGenerator};
use specs::prelude::*;

/*
 *
 * spawner.rs
 * ----------
 * Controls basic spawning of entities and inserts them into the ECS.
 *
 */

// Some of this stuff is based on https://github.com/tylervipond/apprentice/blob/master/src/spawner.rs
fn entity_in_container(ecs: &mut World, container: Entity) -> EntityBuilder {
    ecs.create_entity().with(Contained {
        container: container,
    })
}

fn entity_with_position(ecs: &mut World, x: i32, y: i32) -> EntityBuilder {
    ecs.create_entity().with(Position { x, y })
}

pub fn player(ecs: &mut World, x: i32, y: i32) -> Entity {
    entity_with_position(ecs, x, y)
        .with(Position { x, y })
        .with(Renderable {
            glyph: to_cp437('@'),
            color: ColorPair::new(color("BrightWhite", 1.0), color("Background", 1.0)),
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
        .with(InventoryCapacity { curr: 0, max: 15 })
        .build()
}

pub fn test_mob(ecs: &mut World, x: i32, y: i32) -> Entity {
    entity_with_position(ecs, x, y)
        .with(Mob {})
        .with(Renderable {
            glyph: to_cp437('t'),
            color: ColorPair::new(color("Red", 1.0), color("Background", 1.0)),
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

pub fn test_consumable(builder: EntityBuilder) -> Entity {
    builder
        .with(Renderable {
            glyph: to_cp437('!'),
            color: ColorPair::new(color("BrightRed", 1.0), color("Background", 1.0)),
            layer: 0,
        })
        .with(Name {
            name: "Test Consumable".to_string(),
        })
        .with(Item {})
        .with(Consumable { heal: 5 })
        .build()
}

pub fn test_sword(builder: EntityBuilder) -> Entity {
    builder
        .with(Renderable {
            glyph: to_cp437('/'),
            color: ColorPair::new(color("BrightCyan", 1.0), color("Background", 1.0)),
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

pub fn test_sword_container(ecs: &mut World, container: Entity) -> Entity {
    test_sword(entity_in_container(ecs, container))
}

pub fn test_consumable_container(ecs: &mut World, container: Entity) -> Entity {
    test_consumable(entity_in_container(ecs, container))
}

pub fn test_container(builder: EntityBuilder) -> Entity {
    builder
        .with(Renderable {
            glyph: to_cp437('Æ'),
            color: ColorPair::new(color("Magenta", 1.0), color("Background", 1.0)),
            layer: 1,
        })
        .with(Name {
            name: "Chest".to_string(),
        })
        .with(Blocker {})
        .with(Container {})
        .build()
}

fn get_all_containers(ecs: &World) -> Vec<Entity> {
    let entities = ecs.entities();
    let pos = ecs.read_storage::<Position>();
    let containers = ecs.read_storage::<Container>();

    (&containers, &pos, &entities)
        .join()
        .map(|(_c, _p, e)| e)
        .collect()
}

pub fn populate_containers(ecs: &mut World) {
    let containers = get_all_containers(ecs);

    for c in containers {
        test_sword_container(ecs, c);
        test_consumable_container(ecs, c);
        test_consumable_container(ecs, c);
    }
}

pub fn spawn_map(ecs: &mut World, map: &Map) {
    let idx = map.idx(map.width / 2 + 2, map.height / 2 + 2);
    let pt = map.idx_pos(idx);
    ecs.insert(Point::new(pt.x, pt.y));
    let player = player(ecs, pt.x, pt.y);
    ecs.insert(player);

    spawn_item(
        "Med-Kit",
        pt.x + 2,
        pt.y + 1,
        ecs.create_entity(),
        &RAWS.lock().unwrap(),
    );

    spawn_item(
        "Tantou",
        pt.x + 1,
        pt.y + 1,
        ecs.create_entity(),
        &RAWS.lock().unwrap(),
    );

    test_container(entity_with_position(ecs, pt.x + 3, pt.y + 1));

    populate_containers(ecs);

    let mut rng = RandomNumberGenerator::new();

    for _i in 0..15 {
        let x = rng.roll_dice(1, map.width - 2);
        let y = rng.roll_dice(1, map.height - 2);
        let idx = map.idx(x, y);
        if !map.tiles[idx].block {
            spawn_mob(
                "Man-ape",
                x,
                y,
                ecs.create_entity(),
                &RAWS.lock().unwrap(),
            );
        }
    }
}
