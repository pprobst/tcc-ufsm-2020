use super::{
    map_gen::Map, raws::*, utils::colors::*, BaseStats, Blocker, Consumable, Contained, Container,
    Description, EquipSlot, Equipable, Equipment, Fov, Health, InventoryCapacity, Item,
    MeleeWeapon, MeleeWeaponClass, Mob, Name, Player, Position, Renderable,
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
        .with(Description {
            descr: "It is you, wanderer.".to_string(),
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

/*
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
*/

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
        .with(Item { tier: 3 })
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
        .with(Item { tier: 1 })
        .with(Equipable {
            slot: EquipSlot::Weapon1,
        })
        .with(MeleeWeapon {
            base_damage: 5,
            class: MeleeWeaponClass::Sword,
        })
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

fn get_all_named_mobs(ecs: &World) -> Vec<(Entity, String)> {
    let entities = ecs.entities();
    let mobs = ecs.read_storage::<Mob>();
    let names = ecs.read_storage::<Name>();

    (&mobs, &entities, &names)
        .join()
        .map(|(_c, e, n)| (e, n.name.clone()))
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

fn equip_mobs(ecs: &mut World, rng: &mut RandomNumberGenerator) {
    let mobs = get_all_named_mobs(ecs);

    let raws = &RAWS.lock().unwrap();

    for mob in mobs {
        if let Some(equips) = get_random_possible_equips(&mob.1, raws, rng) {
            for equip in equips.iter() {
                if equip != "None" {
                    if let Some(e) = spawn_item(
                        equip.as_str(),
                        Position::new(0, 0),
                        ecs.create_entity(),
                        raws,
                    ) {
                        let mut equipments = ecs.write_storage::<Equipment>();
                        equipments
                            .insert(
                                e,
                                Equipment {
                                    user: mob.0,
                                    equip: e,
                                },
                            )
                            .expect("FAILED equipping item.");
                    }
                }
            }
        }
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
        Position::new(pt.x + 2, pt.y + 1),
        ecs.create_entity(),
        &RAWS.lock().unwrap(),
    );

    spawn_item(
        "Tantou",
        Position::new(pt.x + 1, pt.y + 1),
        ecs.create_entity(),
        &RAWS.lock().unwrap(),
    );

    spawn_item(
        "Old Leather Armor",
        Position::new(pt.x + 1, pt.y + 2),
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
                Position::new(x, y),
                ecs.create_entity(),
                &RAWS.lock().unwrap(),
            );
        }
    }
    equip_mobs(ecs, &mut rng);
}
