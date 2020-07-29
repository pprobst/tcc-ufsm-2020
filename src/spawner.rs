use super::{
    map_gen::Map, raws::*, utils::colors::*, BaseStats, Contained, Container,
    Description, Equipment, Fov, Health, InventoryCapacity, 
    Mob, Name, Player, Position, Renderable, InBackpack,
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

fn player(ecs: &mut World, x: i32, y: i32) -> Entity {
    entity_with_position(ecs, x, y)
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

fn get_all_tiered_containers(ecs: &World) -> Vec<(Entity, Vec<u8>)> {
    let entities = ecs.entities();
    let pos = ecs.read_storage::<Position>();
    let containers = ecs.read_storage::<Container>();

    (&pos, &entities, &containers)
        .join()
        .map(|(_p, e, c)| (e, c.tiers.clone()))
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

fn populate_containers(ecs: &mut World, raws: &RawMaster, rng: &mut RandomNumberGenerator) {
    let containers = get_all_tiered_containers(ecs);

    for c in containers {
        for tier in c.1 {
            let items = get_items_tier(tier, raws);
            if rng.range(0, 4) < 3 { 
                let random_item = rng.random_slice_entry(&items).unwrap().to_string();
                spawn_item(
                    &random_item,
                    None,
                    entity_in_container(ecs, c.0),
                    raws
                );
            }
        }
    }
}

fn equip_mobs(ecs: &mut World, raws: &RawMaster, rng: &mut RandomNumberGenerator) {
    let mobs = get_all_named_mobs(ecs);

    for mob in mobs {
        if let Some(equips) = get_random_possible_equips(&mob.1, raws, rng) {
            for equip in equips.iter() {
                if equip != "None" {
                    if let Some(e) = spawn_item(
                        equip.as_str(),
                        None,
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
                            .expect("FAILED to equip item.");
                        // For simplicity's sake, mobs will have a clone of the item they're
                        // equipping in their inventory, so as to make their remains' drop more
                        // generic --  this is not the case for the player. Mobs don't really
                        // have to think about inventory management, after all.
                        let mut backpack = ecs.write_storage::<InBackpack>();
                        backpack.insert(e, InBackpack { owner: mob.0 }).expect("FAILED to insert item in backpack.");
                    }
                }
            }
        }
    }
}

pub fn spawn_remains(ecs: &mut World, items: Vec<Entity>, ent_name: String, pos: Position) {
    let remains = entity_with_position(ecs, pos.x, pos.y)
        .with(Renderable {
            glyph: to_cp437('▓'),
            color: ColorPair::new(color("Red", 0.5), color("Background", 1.0)),
            layer: 0,
        })
        .with(Container { tiers: vec![0], max_items: 15 })
        .with(Name {
            name: format!("Remains of {}", ent_name),
        })
        .build();

    let mut contain = ecs.write_storage::<Contained>();
    for item in items {
        contain.insert(item, Contained { container: remains }).expect("FAILED to insert item in remains.");
    }
}

pub fn spawn_map(ecs: &mut World, map: &Map) {
    let idx = map.idx(map.width / 2 + 2, map.height / 2 + 2);
    let pt = map.idx_pos(idx);
    ecs.insert(Point::new(pt.x, pt.y));
    let player = player(ecs, pt.x, pt.y);
    ecs.insert(player);
    let raws = &RAWS.lock().unwrap();

    spawn_item(
        "Med-Kit",
        Some(Position::new(pt.x + 2, pt.y + 1)),
        ecs.create_entity(),
        raws
    );

    spawn_item(
        "Tantou",
        Some(Position::new(pt.x + 1, pt.y + 1)),
        ecs.create_entity(),
        raws
    );

    spawn_item(
        "Old Leather Armor",
        Some(Position::new(pt.x + 1, pt.y + 2)),
        ecs.create_entity(),
        raws
    );

    spawn_container(
        "Chest",
        Position::new(pt.x + 3, pt.y + 1),
        ecs.create_entity(),
        raws
    );


    let mut rng = RandomNumberGenerator::new();

    populate_containers(ecs, raws, &mut rng);

    for _i in 0..15 {
        let x = rng.roll_dice(1, map.width - 2);
        let y = rng.roll_dice(1, map.height - 2);
        let idx = map.idx(x, y);
        if !map.tiles[idx].block {
            spawn_mob(
                "Man-ape",
                Position::new(x, y),
                ecs.create_entity(),
                raws,
            );
        }
    }

    equip_mobs(ecs, raws, &mut rng);
}
