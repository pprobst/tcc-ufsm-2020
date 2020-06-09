use crate::components::{CollectItem, Contained, InBackpack, InventoryCapacity, Name, Position};
use crate::log::Log;
use bracket_lib::prelude::{ORANGE, RGB, WHITE};
use specs::prelude::*;

/*
 *
 * item_collect.rs
 * ---------------
 * Manages the acquiring of items on the map, inserting them in the player's backpack.
 *
 */

pub struct ItemCollectSystem {}

impl<'a> System<'a> for ItemCollectSystem {
    type SystemData = (
        ReadExpect<'a, Entity>,
        ReadStorage<'a, Name>,
        WriteExpect<'a, Log>,
        WriteStorage<'a, InventoryCapacity>,
        WriteStorage<'a, Position>,
        WriteStorage<'a, CollectItem>,
        WriteStorage<'a, InBackpack>,
        WriteStorage<'a, Contained>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (
            player,
            name,
            mut log,
            mut capacity,
            mut pos,
            mut collect,
            mut backpack,
            mut contained,
        ) = data;
        let mut inventory_cap = capacity.get_mut(*player).unwrap();
        for p in collect.join() {
            for c in p.collects.iter() {
                if inventory_cap.curr == inventory_cap.max && c.1 == *player {
                    log.add(format!("Your inventory is full!"), RGB::named(ORANGE));
                    break;
                }
                backpack
                    .insert(c.0, InBackpack { owner: c.1 })
                    .expect("FAILED to insert item in backpack.");
                if c.1 == *player {
                    log.add(
                        format!("You pick up {}.", name.get(c.0).unwrap().name),
                        RGB::named(WHITE),
                    );
                }
                pos.remove(c.0);
                contained.remove(c.0);
                inventory_cap.curr += 1;
            }
        }
        collect.clear();
    }
}
